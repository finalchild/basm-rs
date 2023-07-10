use core::arch::asm;

use crate::solution;
use basm::allocator;
use basm::services;

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator;


#[cfg(target_arch = "x86_64")]
#[no_mangle]
#[naked]
#[link_section = ".init"]
unsafe extern "sysv64" fn _start() -> ! {
    // AMD64 System V ABI requires RSP to be aligned
    //   on the 16-byte boundary BEFORE `call' instruction
    asm!(
        "and    rsp, 0xFFFFFFFFFFFFFFF0",
        "mov    r12, rdi",
        "mov    rdi, QWORD PTR [rdi + 0]",
        "lea    rsi, [rip + _DYNAMIC]",
        "call   {0}",
        "mov    rdi, r12",
        "call   {1}",
        sym basm::platform::amd64::relocate, sym _start_rust, options(noreturn)
    );
}

#[cfg(target_arch = "x86")]
#[no_mangle]
#[naked]
#[link_section = ".data"]
unsafe extern "cdecl" fn _get_dynamic_section_offset() -> ! {
    asm!(
        "lea    eax, [_DYNAMIC]",
        "ret",
        options(noreturn)
    );
}

#[cfg(target_arch = "x86")]
#[no_mangle]
#[naked]
#[link_section = ".init"]
unsafe extern "cdecl" fn _start() -> ! {
    // i386 System V ABI requires ESP to be aligned
    //   on the 16-byte boundary BEFORE `call' instruction
    asm!(
        "mov    edi, DWORD PTR [esp + 4]",
        "and    esp, 0xFFFFFFF0",
        "call   {2}",
        "mov    ebx, DWORD PTR [edi]",
        "add    eax, ebx",
        "sub    esp, 8",
        "push   eax",
        "push   ebx",
        "call   {0}",
        "add    esp, 4",
        "push   edi",
        "call   {1}",
        sym basm::platform::i686::relocate,
        sym _start_rust,
        sym _get_dynamic_section_offset,
        options(noreturn)
    );
}

#[cfg(all(not(target_arch = "x86_64"), not(target_arch = "x86")))]
compile_error!("The target architecture is not supported.");

extern "C" fn _start_rust(service_functions: usize) -> ! {
    services::init(service_functions);
    solution::main();
    services::exit(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_fail(_: core::alloc::Layout) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}


#[cfg(not(test))]
#[no_mangle]
#[allow(non_snake_case)]
pub fn _Unwind_Resume() {
    unsafe { core::hint::unreachable_unchecked() }
}

#[cfg(not(test))]
#[no_mangle]
pub fn rust_eh_personality() {
    unsafe { core::hint::unreachable_unchecked() }
}