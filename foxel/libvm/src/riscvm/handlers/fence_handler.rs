use crate::{
    riscvm::{
        vm::VmState,
        types::*,
        macros::*,
        instruction::*,
    },
};

#[inline(always)]
pub(crate) unsafe extern "C"

fn fence_handler(
    vm: &mut VmState,
    _inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&_inst);
    }

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }

    dispatch!(vm);
}
