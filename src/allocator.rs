#![no_std]
#![feature(allocator_api)]

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::cell::UnsafeCell;
use core::ptr::{null_mut, NonNull};
use core::mem::{size_of, align_of};

/// Wewnętrzny stan alokatora.
struct AllocState {
    callback: extern "win64" fn(size:u64, align:u64)->u64
}

/// Prosty linked-list allocator.
pub struct LinkedListAlloc {
    state: UnsafeCell<AllocState>,
}

// Safety: brak synchronizacji; jeśli używasz wielowątkowości, dodaj własną blokadę.
unsafe impl Sync for LinkedListAlloc {}

impl LinkedListAlloc {
    pub const fn new() -> Self {
        extern "win64" fn defaultAlloc(size:u64, align:u64)->u64 {
            return 0
        }
        Self {
            state: UnsafeCell::new(AllocState {
                callback:defaultAlloc
            }),
        }
    }

    /// Inicjalizuje stertę dla alokatora.
    /// ptr - początek regionu pamięci
    /// size - rozmiar regionu
    pub unsafe fn init(&self, callback: extern "win64" fn(size:u64, align:u64)->u64) {
        let state = &mut *self.state.get();
        state.callback=callback;
    }

    /// Alokuje blok dla danego Layout.
    unsafe fn alloc_impl(&self, layout: Layout) -> *mut u8 {
        let state = &mut *self.state.get();
        return (state.callback)(layout.size() as u64, layout.align() as u64) as *mut u8;

    }

    /// Zwalnia blok o podanym Layout.
    unsafe fn dealloc_impl(&self, ptr: *mut u8, layout: Layout) {
        return;
       
    }
}


unsafe impl GlobalAlloc for LinkedListAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc_impl(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc_impl(ptr, layout)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let page = self.alloc(layout);
        for i in 0..layout.size() {
            *(page.offset(i as isize)) = 0;
        }
        return page;
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, _new_size: usize) -> *mut u8 {
        return ptr;
    }
}


#[global_allocator]
pub static ALLOC: LinkedListAlloc = LinkedListAlloc::new();



