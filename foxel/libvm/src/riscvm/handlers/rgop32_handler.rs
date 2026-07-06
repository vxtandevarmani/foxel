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

fn regop32_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    let value: i64;
    let rs1: i64 = vm.register_read(inst.rtype().rs1()) as i64;
    let rs2: i64 = vm.register_read(inst.rtype().rs2()) as i64;

    if !check_bit!(inst.rtype().funct7(), 0)  {
        match bits_to_rgop32(inst.rtype().funct3()) {
            RgOp32::ADDSUBW => {
                if check_bit!(inst.rtype().funct7(), 6) {
                    value = rs1 - rs2;
                } else {
                    value = rs1 + rs2;
                }
            },
            RgOp32::SLLW => {
                value = rs1 << extract_shammt32!(rs2);
            },
            RgOp32::SRXW => {
                if check_bit!(inst.rtype().funct7(), 6) {
                    value = rs1 >> extract_shammt32!(rs2);
                } else {
                    value = (rs1 as u32 >> extract_shammt32!(rs2) as u32) as i64;
                }
            }
            _ => {return;},
        }
    } else {
        match bits_to_m64(inst.rtype().funct3()) {
            M64::MULW => {
                value = (rs1 as i32).wrapping_mul(rs2 as i32) as i64; //(rs1 as i32) * (rs2 as i32);
            },
            M64::DIVW => {
                if rs2 == 0 {
                    value = -1;
                } else if rs1 == i64::MIN && rs2 == -1 {
                    value = i64::MIN;
                } else {
                    value = rs1 / rs2;
                }
            },
            M64::DIVUW => {
                if rs2 == 0 {
                    value = -1;
                } else {
                    value = ( (rs1 as u32) / (rs2 as u32) ) as i64;
                }
            },
            M64::REMW => {
                if rs2 == 0 {
                    value = rs1;
                } else if rs1 == i64::MIN && rs2 == -1 {
                    value = 0;
                } else {
                    value = rs1 % rs2;
                }
            },
            M64::REMUW => {
                if rs2 == 0 {
                    value = rs1;
                } else if rs1 == i64::MIN && rs2 == -1 {
                    value = 0;
                } else {
                    value = ( (rs1 as u32) % (rs2 as u32) ) as i64;
                }
            },
            _ => {return;},
        }
    }

    vm.register_write(inst.rtype().rd(), value as i64 as u64);

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }

    dispatch!(vm);
}
