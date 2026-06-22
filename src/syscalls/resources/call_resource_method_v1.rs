use crate::syscalls::SyscallRequest;
use crate::uuid::Uuid;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use crate::typedValue::TypedValue;
pub struct CallResourceMethodV1 {
    pub resource: Uuid,
    pub method: String,
    pub args: TypedValue,
}

pub struct CallResourceMethodV1Response {
    pub value: TypedValue

}
impl CallResourceMethodV1 {
    pub fn create(resource: Uuid, method: String, args: TypedValue) -> Box<SyscallRequest<Self>> {
        Box::new(SyscallRequest {
            size: size_of::<Self>(),
            uuid: crate::uuid::Uuid::from_u128(0xbce7baa2_c3e2_4f7f_9d42_42c94065f5f0),
            payload: Self {
                resource,
                method,
                args,
            },
        })
    }
}
