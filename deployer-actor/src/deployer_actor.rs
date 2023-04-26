use fil_actors_runtime::{
    actor_dispatch, actor_error, extract_send_result,
    runtime::{ActorCode, Runtime},
    ActorContext, ActorDowncast, ActorError, AsActorError, INIT_ACTOR_ADDR,
};
use frc42_dispatch::method_hash;
use fvm_ipld_encoding::{ipld_block::IpldBlock, RawBytes};
use fvm_shared::{address::Address, econ::TokenAmount, error::ExitCode, METHOD_CONSTRUCTOR};
use num_derive::FromPrimitive;
use num_traits::Zero;

use crate::state::State;
use crate::types::*;

fil_actors_runtime::wasm_trampoline!(Actor);

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    CheckAddress = frc42_dispatch::method_hash!("CheckAddress"),        //2881763653
    DeployActor = frc42_dispatch::method_hash!("DeployActor"),          //4266815605
    CallActorMethod = frc42_dispatch::method_hash!("CallActorMethod"),  //1343012348
}

pub trait DeployerActor {
    fn constructor(rt: &impl Runtime) -> Result<(), ActorError>;

    fn check_address(rt: &impl Runtime) -> Result<Address, ActorError>;

    fn deploy_actor(rt: &impl Runtime, code: Vec<u8>) -> Result<(), ActorError>;

    fn call_actor_method(rt: &impl Runtime) -> Result<String, ActorError>;
}

pub struct Actor;

impl DeployerActor for Actor {
    fn constructor(rt: &impl Runtime) -> Result<(), ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&INIT_ACTOR_ADDR))?;

        let st = State::new().map_err(|e| {
            e.downcast_default(ExitCode::USR_ILLEGAL_STATE, "Failed to create actor state.")
        })?;

        rt.create(&st)?;

        Ok(())
    }

    fn check_address(rt: &impl Runtime) -> Result<Address, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        let st: State = rt.state()?;
        Ok(st.deployed_actor_id)
    }

    fn deploy_actor(rt: &impl Runtime, code: Vec<u8>) -> Result<(), ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        //rt.transaction(|st: &mut State, rt| {
        let code = RawBytes::serialize(code).unwrap();
        let params = InstallParams { code };

        let ret = extract_send_result(rt.send_simple(
            &INIT_ACTOR_ADDR,
            4,
            IpldBlock::serialize_cbor(&params)?,
            TokenAmount::zero(),
        ))
        .context("failed to send install message to init actor".to_string())?;

        let ret_value: InstallReturn = ret
            .with_context_code(ExitCode::USR_ASSERTION_FAILED, || {
                "return expected".to_string()
            })?
            .deserialize()?;

        if ret_value.installed {
            let params = RawBytes::serialize(&ExecParams {
                code_cid: ret_value.code_cid,
                constructor_params: RawBytes::default(),
            })
            .unwrap();

            let ret = extract_send_result(rt.send_simple(
                &INIT_ACTOR_ADDR,
                2,
                params.into(),
                TokenAmount::zero(),
            ))
            .context("failed to send exec message to init actor".to_string())?;

            let ret_value: ExecReturn = ret
                .with_context_code(ExitCode::USR_ASSERTION_FAILED, || {
                    "return expected".to_string()
                })?
                .deserialize()?;

            rt.transaction(|st: &mut State, _| {
                st.deployed_actor_id = ret_value.id_address;
                st.deployed_actor_robust = ret_value.robust_address;
                Ok(())
            })

            //st.deployed_actor_id = ret_value.id_address;
            //st.deployed_actor_robust = ret_value.robust_address;
        } else {
            return Err(actor_error!(assertion_failed, "Init actor returned false"));
        }
        //Ok(())
        //})
    }

    fn call_actor_method(rt: &impl Runtime) -> Result<String, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        let st: State = rt.state()?;

        let ret = extract_send_result(rt.send_simple(
            &st.deployed_actor_id,
            method_hash!("SayHello"),
            None,
            TokenAmount::zero(),
        ))
        .context("failed to send message to HelloWorld actor".to_string())?;

        let ret_value: String = ret
            .with_context_code(ExitCode::USR_ASSERTION_FAILED, || {
                "return expected".to_string()
            })?
            .deserialize()?;

        Ok(ret_value)
    }
}

impl ActorCode for Actor {
    type Methods = Method;

    fn name() -> &'static str {
        "Deployer"
    }

    actor_dispatch! {
        Constructor => constructor,
        CheckAddress => check_address,
        DeployActor => deploy_actor,
        CallActorMethod => call_actor_method,
    }
}
