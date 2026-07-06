use crate::{
    riscvm::{
        vm::VmState,
        api::syscall_interface,
        types::*,
        enums::*,
        macros::*,
        helpers::*,
        instruction::*,
    },
};

#[inline(always)]
pub(crate) unsafe extern "C"

fn syscall_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    match bits_to_syscall(inst.itype().imm()) {
        Syscall::Ecall => {
            let mut result: u64     = 0;

            let ssn: u64            = vm.register_read(Registers::A7 as u32);
            let ssn: ServiceNumber  = bits_to_ssn(ssn);

            if !syscall_interface(vm, ssn, &mut result) {
                return;
            }

            vm.register_write(Registers::A0 as u32, result);

        }, 
        Syscall::EBreak => {
            #[cfg(debug_assertions)]
                {eprint!("[Ebreak detected! Halting implant execution]\n");}
            return;
        },

        _ => {return;}
    }

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }
    dispatch!(vm);
}
