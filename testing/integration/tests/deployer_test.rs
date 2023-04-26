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
        common::init_actor_stateless(wasm_bin, actor_address, TokenAmount::from_whole(2147483647));

    let mut seq: u64 = 0;
    common::deploy_actor(actor_address, RawBytes::default(), &mut tester);

    // Test fails here with
    // ExitCode { value: 16 }
    // MessageBacktrace(
    //     Backtrace {
    //         frames: [
    //             Frame {
    //                 source: 1,
    //                 method: 4,
    //                 code: ExitCode { value: 16 }, message: "failed to check state for installed actor: failed to install actor"
    //             }, Frame {
    //                 source: 10000,
    //                 method: 4266815605,
    //                 code: ExitCode { value: 16 }, message: "failed to send install message to init actor: send aborted with code 16"
    //             }
    //         ], cause: Some(Syscall { module: "actor", function: "install_actor", error: IllegalArgument, message: "failed to load actor code" })
    //     }
    // )
    common::send_message(
        sender,
        actor_address,
        &mut seq,
        Method::DeployActor as MethodNum,
        RawBytes::serialize(HELLO_WORLD_BINARY.unwrap()).unwrap(),
        &mut tester,
    );

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
fn hello_world_constructor_test() {
    // Get wasm bin
    let wasm_bin = HELLO_WORLD_BINARY.unwrap();
    let actor_address = Address::new_id(10000);

    let (mut tester, sender, ..) =
        common::init_actor_stateless(wasm_bin, actor_address, TokenAmount::zero());

    let mut seq: u64 = 0;
    common::deploy_actor(actor_address, RawBytes::default(), &mut tester);

    let mut res = common::send_message(
        sender,
        actor_address,
        &mut seq,
        method_hash!("SayHello"),
        RawBytes::default(),
        &mut tester,
    );

    let mut str: String = res.msg_receipt.return_data.deserialize().unwrap();
    assert_eq!(str, "Hello world!".to_string());

    res = common::send_message(
        sender,
        actor_address,
        &mut seq,
        method_hash!("Increment"),
        RawBytes::default(),
        &mut tester,
    );

    str = res.msg_receipt.return_data.deserialize().unwrap();
    assert_eq!(str, "Counter = 1".to_string());
}
