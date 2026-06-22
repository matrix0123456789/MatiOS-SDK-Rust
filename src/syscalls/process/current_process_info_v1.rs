use crate::sdk::syscalls::SyscallRequest;
use crate::sdk::uuid::Uuid;
use alloc::boxed::Box;
use alloc::string::String;
pub struct CurrentProcessInfoV1Request {}
#[repr(C)]
pub struct CurrentProcessInfoV1Response {
    pub uuid: Uuid,
    pub name: String,
}

impl CurrentProcessInfoV1Request {
    pub fn create() -> Box<SyscallRequest<Self>> {
        Box::new(SyscallRequest {
            size: size_of::<Self>(),
            uuid: crate::sdk::uuid::Uuid::parse_str("6ac0d646-72dc-4fe4-9fdc-f944f1a61491")
                .unwrap(),
            payload: Self {},
        })
    }
}
