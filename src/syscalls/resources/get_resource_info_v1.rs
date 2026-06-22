use crate::syscalls::SyscallRequest;
use crate::uuid::Uuid;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
pub struct newGetResourceInfoV1Request {
    pub uuid: Uuid,

}

pub struct RequestResourceV1Response {
    pub uuid: Uuid

}
impl GetResourceInfoV1Request {
    pub fn create(uuid: Uuid) -> Box<SyscallRequest<Self>> {
        Box::new(SyscallRequest {
            size: size_of::<Self>(),
            uuid: crate::uuid::Uuid::from_u128(0xf60347f7_c312_48c2_9db4_0b5efb60db08)                ,
            payload: Self {
               uuid
            },
        })
    }
}
