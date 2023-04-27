use deployer_actor::{deployer_actor::Method, WASM_BINARY as DEPLOYER_BINARY};
use frc42_dispatch::method_hash;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::{address::Address, bigint::Zero, econ::TokenAmount, MethodNum};

mod common;

pub const HELLO_WORLD_BINARY: Option<&[u8]> = Some(include_bytes!(
    "../../../target/debug/wbuild/hello-world-actor/hello_world_actor.compact.wasm"
));

#[test]
fn deployer_constructor_test() {
    // Get wasm bin
    let wasm_bin = DEPLOYER_BINARY.unwrap();
    let actor_address = Address::new_id(10000);

    let (mut tester, sender, ..) =
        common::init_actor_stateless(wasm_bin, actor_address, TokenAmount::zero());

    let mut seq: u64 = 0;
    common::deploy_actor(actor_address, RawBytes::default(), &mut tester);

    let res = common::send_message(
        sender,
        actor_address,
        &mut seq,
        Method::CheckAddress as MethodNum,
        RawBytes::default(),
        &mut tester,
    );

    let addr: Address = res.msg_receipt.return_data.deserialize().unwrap();
    assert_eq!(addr, Address::new_id(0));
}

#[test]
fn deployer_working_test() {
    // This test is currently failing

    // Get wasm bin
    let wasm_bin = DEPLOYER_BINARY.unwrap();
    let actor_address = Address::new_id(10000);

    let (mut tester, sender, ..) =
        //common::init_actor_stateless(wasm_bin, actor_address, TokenAmount::zero());
        common::init_actor_stateless_with_bundle(wasm_bin, actor_address, TokenAmount::zero(),"/home/milmaks/milos-ethernal/builtin-actors/target/debug/build/fil_builtin_actors_bundle-b3eedaa45c3a001b/out/bundle/bundle.car");
    let mut seq: u64 = 0;
    common::deploy_actor(actor_address, RawBytes::default(), &mut tester);

    common::send_message(
        sender,
        actor_address,
        &mut seq,
        Method::DeployActor as MethodNum,
        RawBytes::default(),
        &mut tester,
    );

    let mut res = common::send_message(
        sender,
        actor_address,
        &mut seq,
        Method::CheckAddress as MethodNum,
        RawBytes::default(),
        &mut tester,
    );

    let hello_world_addr: Address = res.msg_receipt.return_data.deserialize().unwrap();
    assert_ne!(hello_world_addr, Address::new_id(0));

    res = common::send_message(
        sender,
        actor_address,
        &mut seq,
        Method::CallActorMethod as MethodNum,
        RawBytes::default(),
        &mut tester,
    );

    assert_eq!("Hello world!".to_string(), res.msg_receipt.return_data.deserialize::<String>().unwrap());

    res = common::send_message(
        sender,
        hello_world_addr,
        &mut seq,
        method_hash!("SayHello"),
        RawBytes::default(),
        &mut tester,
    );

    assert_eq!("Hello world!".to_string(), res.msg_receipt.return_data.deserialize::<String>().unwrap());
}

#[test]
fn hello_world_constructor_test() {
    // Get wasm bin
    let wasm_bin = HELLO_WORLD_BINARY.unwrap();
    let actor_address = Address::new_id(10000);

    let (mut tester, sender, ..) =
        common::init_actor_stateless(wasm_bin, actor_address, TokenAmount::zero());

    let mut seq: u64 = 0;
    common::deploy_actor(actor_address, RawBytes::default(), &mut tester);

    let res = common::send_message(
        sender,
        actor_address,
        &mut seq,
        method_hash!("SayHello"),
        RawBytes::default(),
        &mut tester,
    );

    let str: String = res.msg_receipt.return_data.deserialize().unwrap();
    assert_eq!(str, "Hello world!".to_string());
}
