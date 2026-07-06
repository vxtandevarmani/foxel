use crate::riscvm::{
    vm::VmState,
    instruction::*,
};

#[inline(always)]
pub(crate) unsafe extern "C"
fn invalid_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    vm.trace_state(&inst);
    println!("[-] Invalid instruction detected halting vm execution {:08x}",
            inst.raw());
    
    return;
}
