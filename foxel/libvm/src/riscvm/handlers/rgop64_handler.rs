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

fn rgop64_handler(
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
    
    if !check_bit!(inst.rtype().funct7(), 0) {
        match bits_to_rgop64(inst.rtype().funct3()) {
            RgOp64::ADDSUB => {
                if check_bit!(inst.rtype().funct7(), 6) {       
                    // inst.rtype().funct7()  != 0
                    value = rs1 - rs2;
                } else {
                    value = rs1 + rs2;
                }
            },
            RgOp64::SLL => {
                value = rs1 << rs2;
            },
            RgOp64::SLT => {
                if rs1 < rs2 {
                    value = 1;
                } else {
                    value = 0;
                }
            },
            RgOp64::SLTU => {
                if (rs1 as u64) < (rs2 as u64) {
                    value = 1;
                } else {
                    value = 0;
                }
            },
            RgOp64::XOR => {
                value = rs1 ^ rs2;
            },
            RgOp64::SRX => {
                if check_bit!(inst.rtype().funct7(), 6) {
                    value = rs1 >> rs2;
                } else {
                    value = ( rs1 as u64 >> rs2 as u64 ) as i64;
                }
            },
            RgOp64::OR => {
                value = rs1 | rs2;
            },
            RgOp64::AND => {
                value = rs1 & rs2;
            },
            _ => {return;},
        }
    } 
    else {
        match bits_to_m32(inst.rtype().funct3()) {
            M32::MUL => {
                value = rs1 * rs2;
            },
            M32::MULH => {
                let tmp: i128 = (rs1 as i128) * (rs2 as i128);
                value = (tmp >> 64) as i64;
            },
            M32::MULHSU => {
                let tmp: i128 = (rs1 as i128) * (rs2 as u128 as i128);
                value = (tmp >> 64) as i64;
            },
            M32::MULHU => {
                let tmp: i128 = (rs1 as u128 as i128) * (rs2 as u128 as i128);
                value = (tmp >> 64) as i64;
            },
            M32::DIV => {
                if rs2 == 0 {
                    value = -1;
                } else if rs1 == i64::MIN && rs2 == -1 {
                    value = i64::MIN;
                } else {
                    value = rs1 / rs2;
                }
            },
            M32::DIVU => {
                if rs2 == 0 {
                    value = -1;
                } else {
                    value = ( (rs1 as u64) / (rs2 as u64) ) as i64;
                }
            },
            M32::REM => {
                if rs2 == 0 {
                    value = rs1;
                } else if rs1 == i64::MIN && rs2 == -1 {
                    value = 0;
                } else {
                    value = rs1 % rs2;
                }
            },
            M32::REMU => {
                if rs2 == 0 {
                    value = rs1;
                } else {
                    value = ( (rs1 as u64) % (rs2 as u64) ) as i64;
                }
            },
            _ => {return;},
        }
    }

    vm.register_write(inst.rtype().rd(), value as u64);

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }

    dispatch!(vm);
}
