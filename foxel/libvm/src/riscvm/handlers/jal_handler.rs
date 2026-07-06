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

fn jal_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }
    let imm: i32 = sign_extend(
            (inst.ujtype().imm_20()    << 20) | 
            (inst.ujtype().imm_4_1()   <<  1) |
            (inst.ujtype().imm_11()    << 11) |
            (inst.ujtype().imm_19_12() << 12),

            20) as i32;

    vm.register_write(
        inst.ujtype().rd(),
        (vm.pc as u64) + (size_of::<u32>() as u64)
    );
    
    unsafe {
        vm.pc = vm.pc.add(imm as usize);
    }
    dispatch!(vm);
}
