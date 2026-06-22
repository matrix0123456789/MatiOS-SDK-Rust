use crate::syscalls::SyscallRequest;
use crate::uuid::Uuid;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
pub struct GetResourceInfoV1Request {
    pub uuid: Uuid,

}

pub struct RequestResourceV1Response {
    pub uuid: Uuid

}
impl GetResourceInfoV1Request {
    pub fn create(uuid: Uuid) -> Box<SyscallRequest<Self>> {
        Box::new(SyscallRequest {
            size: size_of::<Self>(),
            uuid: crate::uuid::Uuid::from_u128(0xb2828475_e770_4bdc_86e0_695695d6bab0)                ,
            payload: Self {
               uuid
            },
        })
    }
}
