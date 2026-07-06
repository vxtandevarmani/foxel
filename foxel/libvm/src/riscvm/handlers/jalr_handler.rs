use crate::{
    riscvm::{
        vm::VmState,
        types::*,
        macros::*,
        helpers::*,
        instruction::*,
    },
};

#[inline(always)]
pub(crate) unsafe extern "C"

fn jalr_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }


    let t: u64 = (unsafe {
            vm.pc.add( size_of::<u32>() )
        }
    ) as u64;

    let imm: i32 = sign_extend(inst.itype().imm(), 12) as i32;

    vm.pc = (  (vm.register_read( inst.itype().rs1() ) 
            +   (imm as u64)) & !1  ) as *mut u8;
    
    vm.register_write(inst.itype().rd(), t);

    dispatch!(vm);
}
