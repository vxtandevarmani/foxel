use crate::riscvm::{
    instruction::*,
    vm::VmState,
};

pub type Handler = unsafe extern "C" fn(&mut VmState, Instruction);
