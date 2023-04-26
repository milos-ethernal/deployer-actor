use cid::multihash::Multihash;
use cid::Cid;
use fil_actors_runtime::INIT_ACTOR_ADDR;
use fvm::executor::{ApplyKind, ApplyRet, Executor};
use fvm_integration_tests::{bundle, dummy::DummyExterns, tester::Tester};
use fvm_ipld_blockstore::MemoryBlockstore;
use fvm_ipld_encoding::{RawBytes, DAG_CBOR};
use fvm_shared::message::Message;
use fvm_shared::{
    address::Address, econ::TokenAmount, state::StateTreeVersion, version::NetworkVersion,
};
use fvm_shared::{crypto::hash::SupportedHashes, error::ExitCode};
use fvm_shared::{MethodNum, METHOD_CONSTRUCTOR};
use serde::Serialize;
use std::mem;

const fn const_unwrap<T: Copy, E>(r: Result<T, E>) -> T {
    let v = match r {
        Ok(r) => r,
        Err(_) => panic!(),
    };
    mem::forget(r);
    v
}

const EMPTY_ARR_HASH_DIGEST: &[u8] = &[
    0x45, 0xb0, 0xcf, 0xc2, 0x20, 0xce, 0xec, 0x5b, 0x7c, 0x1c, 0x62, 0xc4, 0xd4, 0x19, 0x3d, 0x38,
    0xe4, 0xeb, 0xa4, 0x8e, 0x88, 0x15, 0x72, 0x9c, 0xe7, 0x5f, 0x9c, 0x0a, 0xb0, 0xe4, 0xc1, 0xc0,
];

pub const EMPTY_ARR_CID: Cid = Cid::new_v1(
    DAG_CBOR,
    const_unwrap(Multihash::wrap(
        SupportedHashes::Blake2b256 as u64,
        EMPTY_ARR_HASH_DIGEST,
    )),
);

#[allow(dead_code)]
pub fn init_actor_stateless(
    wasm_bin: &[u8],
    actor_address: Address,
    balance: TokenAmount,
) -> (
    Tester<MemoryBlockstore, DummyExterns>,
    Address,
    u64,
    Address,
    u64,
) {
    let mut tester = create_tester();
    let [(sender_id, sender), (account_id, account)] = tester.create_accounts().unwrap();

    tester
        .set_actor_from_bin(&wasm_bin, EMPTY_ARR_CID, actor_address, balance)
        .unwrap();
    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();
    (tester, sender, sender_id, account, account_id)
}

#[allow(dead_code)]
pub fn init_actor_stateless_with_bundle(
    wasm_bin: &[u8],
    actor_address: Address,
    balance: TokenAmount,
    bundle_path: &str,
) -> (
    Tester<MemoryBlockstore, DummyExterns>,
    Address,
    u64,
    Address,
    u64,
) {
    let mut tester = create_tester_with_bundle(bundle_path);
    let [(sender_id, sender), (account_id, account)] = tester.create_accounts().unwrap();

    tester
        .set_actor_from_bin(&wasm_bin, EMPTY_ARR_CID, actor_address, balance)
        .unwrap();
    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();
    (tester, sender, sender_id, account, account_id)
}

#[allow(dead_code)]
pub fn init_actor_stateless_with_tester(
    wasm_bin: &[u8],
    actor_address: Address,
    balance: TokenAmount,
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
) -> (Address, u64, Address, u64) {
    let [(sender_id, sender), (account_id, account)] = tester.create_accounts().unwrap();

    tester
        .set_actor_from_bin(&wasm_bin, EMPTY_ARR_CID, actor_address, balance)
        .unwrap();

    (sender, sender_id, account, account_id)
}

#[allow(dead_code)]
pub fn init_actor_with_state<S: Serialize>(
    wasm_bin: &[u8],
    actor_address: Address,
    balance: TokenAmount,
    actor_state: &S,
) -> (Tester<MemoryBlockstore, DummyExterns>, Address, u64) {
    let mut tester = create_tester();
    let [(sender_id, sender)] = tester.create_accounts().unwrap();

    let state_cid = tester.set_state(&actor_state).unwrap();
    tester
        .set_actor_from_bin(&wasm_bin, state_cid, actor_address, balance)
        .unwrap();

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();
    (tester, sender, sender_id)
}

pub fn create_tester() -> Tester<MemoryBlockstore, DummyExterns> {
    // Instantiate tester
    let bs = MemoryBlockstore::default();
    let bundle_root = bundle::import_bundle(&bs, actors_v10::BUNDLE_CAR).unwrap();
    Tester::new(NetworkVersion::V18, StateTreeVersion::V5, bundle_root, bs).unwrap()
}

pub fn create_tester_with_bundle(bundle_path: &str) -> Tester<MemoryBlockstore, DummyExterns> {
    // Instantiate tester
    let bs = MemoryBlockstore::default();
    let bundle_root = bundle::import_bundle_from_path(&bs, bundle_path).unwrap();
    Tester::new(NetworkVersion::V18, StateTreeVersion::V5, bundle_root, bs).unwrap()
}

pub fn execute_message_explicit(
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
    message: &Message,
) -> ApplyRet {
    execute_message(tester, message, ApplyKind::Explicit)
}

#[allow(dead_code)]
pub fn execute_message_implicit(
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
    message: &Message,
) -> ApplyRet {
    execute_message(tester, message, ApplyKind::Implicit)
}

fn execute_message(
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
    message: &Message,
    apply_kind: ApplyKind,
) -> ApplyRet {
    let executor = tester.executor.as_mut().unwrap();
    let res = executor
        .execute_message(message.clone(), apply_kind, 100)
        .unwrap();
    res
}

#[allow(dead_code)]
pub fn send_message(
    sender: Address,
    actor_address: Address,
    seq: &mut u64,
    method: MethodNum,
    params: RawBytes,
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
) -> ApplyRet {
    let message = Message {
        from: sender,
        to: actor_address,
        gas_limit: i64::MAX as u64,
        method_num: method,
        sequence: *seq,
        params,
        ..Message::default()
    };

    *seq += 1;

    let res = execute_message_explicit(tester, &message);

    assert_eq!(
        ExitCode::OK,
        res.msg_receipt.exit_code,
        "{:?}",
        res.failure_info
    );

    res
}

#[allow(dead_code)]
pub fn send_message_with_funds(
    sender: Address,
    actor_address: Address,
    seq: &mut u64,
    method: MethodNum,
    params: RawBytes,
    value: TokenAmount,
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
) -> ApplyRet {
    let message = Message {
        from: sender,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: method,
        sequence: *seq,
        params,
        value,
        ..Message::default()
    };

    *seq += 1;

    let res = execute_message_explicit(tester, &message);

    assert_eq!(
        ExitCode::OK,
        res.msg_receipt.exit_code,
        "{:?}",
        res.failure_info
    );

    res
}

#[allow(dead_code)]
pub fn send_message_usr_assertion_failed(
    sender: Address,
    actor_address: Address,
    seq: &mut u64,
    method: MethodNum,
    params: RawBytes,
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
) -> ApplyRet {
    let message = Message {
        from: sender,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: method,
        sequence: *seq,
        params,
        ..Message::default()
    };

    *seq += 1;

    let res = execute_message_explicit(tester, &message);

    assert_eq!(
        ExitCode::USR_ASSERTION_FAILED,
        res.msg_receipt.exit_code,
        "{:?}",
        res.failure_info
    );

    res
}

#[allow(dead_code)]
pub fn deploy_actor(
    actor_address: Address,
    params: RawBytes,
    tester: &mut Tester<MemoryBlockstore, DummyExterns>,
) {
    let message = Message {
        from: INIT_ACTOR_ADDR,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: METHOD_CONSTRUCTOR,
        sequence: 0,
        params,
        ..Message::default()
    };

    let res = execute_message_implicit(tester, &message);

    assert_eq!(
        ExitCode::OK,
        res.msg_receipt.exit_code,
        "{:?}",
        res.failure_info
    );
}
