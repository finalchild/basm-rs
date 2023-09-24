use core::ptr;
use super::dlmalloc_interface::DlmallocAllocator;


pub struct System {
    _priv: (),
}

impl System {
    pub const fn new() -> System {
        System { _priv: () }
    }
}

unsafe impl DlmallocAllocator for System {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let addr = unsafe {
            super::super::os::windows::WINAPI.VirtualAlloc(
                ptr::null_mut(),
                size,
                0x00003000, /* MEM_COMMIT | MEM_RESERVE */
                0x04 /* PAGE_READWRITE */
            )
        };
        if addr.is_null() {
            (ptr::null_mut(), 0, 0)
        } else {
            (addr, size, 0)
        }
    }

    #[allow(unused)]
    fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8 {
        core::ptr::null_mut()
    }

    #[allow(unused)]
    fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
        false
    }

    #[allow(unused)]
    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        unsafe {
            super::super::os::windows::WINAPI.VirtualFree(
                ptr,
                0,
                0x00008000 /* MEM_RELEASE */
            ) != 0
        }
    }

    #[allow(unused)]
    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        4096
    }
}