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

fn auipc_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    let imm: i32 = (sign_extend(inst.utype().imm(), 20) << 12) as i32;

    vm.register_write(
        inst.utype().rd(),
        (unsafe {
            vm.pc.add(imm as usize)
        }) as u64
    );

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }
    dispatch!(vm);
}
