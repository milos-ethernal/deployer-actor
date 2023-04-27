use cid::Cid;
use fvm_ipld_encoding::{tuple::*, RawBytes};
use fvm_shared::address::Address;

#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct InstallParams {
    pub code: RawBytes,
}

#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct InstallReturn {
    pub code_cid: Cid,
    pub installed: bool,
}

/// Init actor Exec Params
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ExecParams {
    pub code_cid: Cid,
    pub constructor_params: RawBytes,
}

/// Init actor Exec Return value
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct ExecReturn {
    /// ID based address for created actor
    pub id_address: Address,
    /// Reorg safe address for actor
    pub robust_address: Address,
}
