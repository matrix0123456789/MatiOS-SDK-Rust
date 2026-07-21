use crate::syscalls::{SyscallRequest, SyscallResponse};
use crate::uuid::Uuid;
use alloc::boxed::Box;
use core::cell::OnceCell;
use core::panic::PanicInfo;

static mut GLOBAL_PROCESS_START_INFO: *const ProcessStartInfo = 0 as *const ProcessStartInfo;
#[repr(C)]
pub struct ProcessStartInfo {
    pub processId: Uuid,
    pub debugPrint: extern "win64" fn(&str),
    pub debugPrintInt: extern "win64" fn(u64),
    pub debugPrintUuid: extern "win64" fn(Uuid),
    pub debugPanicRust: extern "win64" fn(&PanicInfo),
    pub allocate: extern "win64" fn(size: u64, align: u64) -> u64,
    pub syscallSync: extern "win64" fn(usize) -> usize,
}
impl ProcessStartInfo {
    pub fn init(info: *const ProcessStartInfo) {
        unsafe {
            GLOBAL_PROCESS_START_INFO = info;
            // GLOBAL_PROCESS_START_INFO.set(Box::from(info.clone()));
        }
    }
    pub fn get() -> &'static ProcessStartInfo {
        unsafe { return &*GLOBAL_PROCESS_START_INFO }
    }
}
impl Copy for ProcessStartInfo {}
impl Clone for ProcessStartInfo {
    fn clone(&self) -> Self {
        return Self {
            debugPrint: self.debugPrint,
            allocate: self.allocate,
            debugPrintInt: self.debugPrintInt,
            debugPrintUuid: self.debugPrintUuid,
            debugPanicRust: self.debugPanicRust,
            syscallSync: self.syscallSync,
            processId: self.processId,
        };
    }
}
pub fn syscall_sync<TReq, TResp>(call_uuid: Uuid, payload: TReq) -> *const TResp {
    let response = (ProcessStartInfo::get().syscallSync)(Box::into_raw(Box::new(SyscallRequest {
        size: size_of::<TReq>(),
        uuid: call_uuid,
        payload,
    })) as usize) as *const SyscallResponse<TResp>;
    return unsafe { &(*response).payload };
}
pub fn syscall_sync_noreturn<TReq>(call_uuid: Uuid, payload: TReq) {
    (ProcessStartInfo::get().syscallSync)(Box::into_raw(Box::new(SyscallRequest {
        size: size_of::<TReq>(),
        uuid: call_uuid,
        payload,
    })) as usize);
}
