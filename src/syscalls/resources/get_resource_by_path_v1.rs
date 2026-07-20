use crate::syscalls::SyscallRequest;
use crate::uuid::Uuid;
use alloc::string::String;

#[repr(C)]
pub struct GetResourceByPathV1Request {
    pub path: String,
}
#[repr(C)]
pub struct GetResourceByPathV1Response {
    pub uuid: Uuid
}
