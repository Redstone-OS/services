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
    print_str("         ██████╗ ███████╗██████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗\n");
    print_str("         ██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝\n");
    print_str("         ██████╔╝█████╗  ██║  ██║███████╗   ██║   ██║   ██║██╔██╗ ██║█████╗  \n");
    print_str("         ██╔══██╗██╔══╝  ██║  ██║╚════██║   ██║   ██║   ██║██║╚██╗██║██╔══╝  \n");
    print_str("         ██║  ██║███████╗██████╔╝███████║   ██║   ╚██████╔╝██║ ╚████║███████╗\n");
    print_str("         ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝\n");
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

/// Imprime string no console (via serial port)
fn print_str(s: &str) {
    for byte in s.bytes() {
        unsafe {
            // COM1 port (0x3F8)
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3F8u16,
                in("al") byte,
                options(nomem, nostack)
            );
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
