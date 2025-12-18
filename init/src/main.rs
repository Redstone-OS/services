//! Init - Processo Init do Redstone OS
//!
//! PID 1 - Primeiro processo userspace.
//! Mostra mensagem de boas-vindas e inicializa o sistema.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Syscall write via int 0x80
#[inline]
unsafe fn syscall_write(fd: u64, buf: *const u8, len: u64) -> u64 {
    let ret: u64;
    core::arch::asm!(
        "mov rax, 1",      // SYS_WRITE
        "int 0x80",
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") len,
        lateout("rax") ret,
        options(nostack, preserves_flags)
    );
    ret
}

/// Escreve string em stdout
fn print(s: &str) {
    unsafe {
        syscall_write(1, s.as_ptr(), s.len() as u64);
    }
}

/// Ponto de entrada do init
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Mensagem de boas-vindas
    print("\n");
    print("╔════════════════════════════════════════╗\n");
    print("║   Bem-vindo ao Redstone OS v0.3.5!   ║\n");
    print("╚════════════════════════════════════════╝\n");
    print("\n");
    print("Sistema inicializado com sucesso!\n");
    print("\n");
    print("Init (PID 1) executando...\n");
    print("\n");
    print("> _\n");

    // Loop infinito (init nunca deve terminar)
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("\n[PANIC] Init process panicked!\n");
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
