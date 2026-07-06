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

fn branch_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    let imm: i32 =  ((inst.sbtype().imm_12()   << 12) |
                     (inst.sbtype().imm_11()   << 11) |
                     (inst.sbtype().imm_10_5() <<  5) |
                     (inst.sbtype().imm_4_1()) <<  1)
                    as i32;

    let imm: i32 = sign_extend(imm as u32, 13);

    let rs1: u64 = vm.register_read(inst.sbtype().rs1() as u32);
    let rs2: u64 = vm.register_read(inst.sbtype().rs2() as u32);

    let condition: bool;
    match bits_to_branch(inst.sbtype().funct3()) {
        Branch::BEQ     => {
            condition = rs1 == rs2;
        },
        Branch::BNE     => {
            condition = rs1 != rs2;
        },
        Branch::BLT     => {
            condition = (rs1 as i64) < (rs2 as i64);
        },
        Branch::BGE     => {
            condition = (rs1 as i64) > (rs2 as i64);
        },
        Branch::BLTU    => {
            condition = rs1 < rs2;
        },
        Branch::BGEU    => {
            condition = rs1 >= rs2;
        },
        _               => {return;},
    }

    unsafe {

        if condition {
            vm.pc = vm.pc.add(imm as usize);
        } else {
            vm.pc = vm.pc.add( size_of::<u32>() );
        }
    }
    dispatch!(vm);
}
