use crate::sdk::syscalls::SyscallRequest;
use crate::sdk::uuid::Uuid;
use alloc::boxed::Box;
use alloc::string::String;

pub struct PrintV1 {

 pub text:String
}
 impl PrintV1 {
    pub fn create(text: String) -> Box<SyscallRequest<Self>> {
        Box::new(SyscallRequest {
            size: size_of:: <Self>(),
            uuid: crate::sdk::uuid::Uuid::parse_str("7b16bee9-d0b8-4bd5-86d7-8225840ce006").unwrap(),
            payload: Self{text}
        })
    }
}