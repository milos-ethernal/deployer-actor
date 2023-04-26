use fil_actors_runtime::{
    actor_dispatch, actor_error,
    runtime::{ActorCode, Runtime},
    ActorError, INIT_ACTOR_ADDR,
};
use fvm_shared::METHOD_CONSTRUCTOR;
use num_derive::FromPrimitive;

fil_actors_runtime::wasm_trampoline!(Actor);

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    SayHello = frc42_dispatch::method_hash!("SayHello"),    //3310437020
}

pub trait HelloWorldActor {
    fn constructor(rt: &impl Runtime) -> Result<(), ActorError>;

    fn say_hello(rt: &impl Runtime) -> Result<String, ActorError>;
}

pub struct Actor;

impl HelloWorldActor for Actor {
    fn constructor(rt: &impl Runtime) -> Result<(), ActorError> {
        rt.validate_immediate_caller_is(std::iter::once(&INIT_ACTOR_ADDR))?;

        Ok(())
    }

    fn say_hello(rt: &impl Runtime) -> Result<String, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        Ok("Hello world!".to_string())
    }
}

impl ActorCode for Actor {
    type Methods = Method;

    fn name() -> &'static str {
        "HelloWorld"
    }

    actor_dispatch! {
        Constructor => constructor,
        SayHello => say_hello,
    }
}
