use crate::riscvm::{
    vm::VmState,
    enums::*,
};


#[derive(Default)]
#[repr(C)]
pub struct Packet {
    pub base    : u64,
    pub length  : u64,
}

#[cfg(target_os = "windows")]
unsafe fn get_peb() -> u64 {

    #[cfg(target_arch = "x86_64")]
    unsafe {
        let peb: u64;
        core::arch::asm!("mov {}, gs:[0x60]", out(reg) peb);
        return peb;
    }

    #[cfg(target_arch = "x86")]
    unsafe {
        let peb: u64;
        core::arch::asm!("mov {}, fs:[0x30]", out(reg) peb);
        return peb;
    }

}

pub fn syscall_interface(vm: &mut VmState, ssn: ServiceNumber, result: &mut u64) -> bool {
    match ssn {

        ServiceNumber::HOST => {
            let address  : u64      = vm.register_read(Registers::A0 as u32);
            let arguments: *mut u64 = vm.register_read(Registers::A1 as u32) 
                        as *mut u64;
            
            type HostCall = 
            fn(
                u64,u64,u64,u64,u64,u64,u64,u64,u64,u64,u64,u64,u64
            ) -> u64;
            
            let external_call: HostCall = unsafe {
                core::mem::transmute(address)
            };
            unsafe {
                *result = external_call(
                    *arguments.add(0),
                    *arguments.add(1),
                    *arguments.add(2),
                    *arguments.add(3),
                    *arguments.add(4),
                    *arguments.add(5),
                    *arguments.add(6),
                    *arguments.add(7),
                    *arguments.add(8),
                    *arguments.add(9),
                    *arguments.add(10),
                    *arguments.add(11),
                    *arguments.add(12),
                );
            }
        },
        ServiceNumber::GPEB => {
            #[cfg(target_os = "windows")] {
                *result = unsafe { get_peb() };
            }
            #[cfg(not(target_os = "windows"))] {
                *result = 0; return false;
            }
        },
        _ => {
            eprintln!("[-] Unknown syscall: 0x{:x}", ssn as u32);
            return false;
        }
    }

    return true;
}
// this is where all the syscalls will layout
