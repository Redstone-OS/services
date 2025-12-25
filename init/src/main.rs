//! Init Process - PID 1
//!
//! Primeiro processo userspace do Redstone OS

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Entry point do processo init
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Banner ASCII do Redstone OS
    print_str("\n\n");
    print_str("    ____           __      __                  \n");
    print_str("   / __ \\___  ____/ /_____/ /_____  ____  ___ \n");
    print_str("  / /_/ / _ \\/ __  / ___/ __/ __ \\/ __ \\/ _ \\ \n");
    print_str(" / _, _/  __/ /_/ (__  ) /_/ /_/ / / / /  __/ \n");
    print_str("/_/ |_|\\___/\\__,_/____/\\__/\\____/_/ /_/\\___/  \n");
    print_str("\n");
    print_str("         Redstone OS v0.0.1 - A modern microkernel operating system\n");
    print_str("\n");
    print_str("         Kernel: 0.1.0 (x86_64)\n");
    print_str("         Memory: 512 MB\n");
    print_str("         Uptime: 0 days, 0 hours, 0 minutes\n");
    print_str("\n");
    print_str("redstone login: root\n");
    print_str("Password: \n");
    print_str("\n");
    print_str("Welcome to Redstone OS!\n");
    print_str("\n");
    print_str("root@redstone:~# _\n");
    print_str("\n");

    // Loop infinito - processo init nunca termina
    loop {
        core::hint::spin_loop();
        // TODO: Syscall sys_yield();
    }
}

/// Imprime string no console (via Syscall)
fn print_str(s: &str) {
    // Syscall ID 1 = SYS_WRITE
    // Arg1 (RDI) = FD (1 = stdout)
    // Arg2 (RSI) = Ptr
    // Arg3 (RDX) = Len
    let ptr = s.as_ptr() as u64;
    let len = s.len() as u64;

    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") 1u64, // Syscall Num
            in("rdi") 1u64, // FD
            in("rsi") ptr,  // Buffer
            in("rdx") len,  // Len
            lateout("rax") _,
            lateout("rcx") _, // Clobbered by syscall (if sysret) or safe for int
            lateout("r11") _, // Clobbered
            options(nostack, preserves_flags) // INT instruction preserves flags, but handler logic might change things if not careful
        );
    }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
