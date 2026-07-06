use crate::{
    riscvm::{
        vm::VmState,
        types::*,
        enums::*,
        macros::*,
        helpers::*,
        instruction::*,
    },
};

#[inline(always)]
pub(crate) unsafe extern "C"
fn store_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    let imm: i32 = sign_extend(
        (inst.stype().imm_11_5() << 5) | 
        (inst.stype().imm_4_0()      ),
        12
    );
    
    let addr: u64 = vm.register_read(inst.stype().rs1()) + (imm as u64);
    let val : u64 = vm.register_read(inst.stype().rs2());

    match bits_to_store(inst.stype().funct3()) {
        Store::SB => {
            unsafe {
                *(addr as *mut u8) = val as u8;
            }
        },
        Store::SH => {
            unsafe {
                *(addr as *mut u16) = val as u16; 
            }
        },
        Store::SW => {
            unsafe {
                *(addr as *mut u32) = val as u32;
            }
        }
        Store::SD => {
            unsafe {
                *(addr as *mut u64) = val as u64;
            }
        }
        _ => {return;},
    }

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }
    dispatch!(vm);
}