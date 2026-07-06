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

fn immop64_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    let mut value: i64 = vm.register_read(inst.itype().rs1()) as i64;
    let imm      : i64 = sign_extend(inst.itype().imm(), 12)  as i64;

    match bits_to_immop64(inst.itype().funct3()) {
        ImmOp64::ADDI => {
            value = value + imm;
        },
        ImmOp64::SLTI => {
            if value < imm {
                value = 1;
            } else {
                value = 0;
            }
        },
        ImmOp64::SLTIU => {
            if (value as u64) < (imm as u64) {
                value = 1;
            } else {
                value = 0;
            }
        },
        ImmOp64::XORI => {
            value = value ^ imm;
        }
        ImmOp64::ORI => {
            value = value | imm;
        },
        ImmOp64::ANDI => {
            value = value & imm;
        },
        ImmOp64::SLLI => {
            value = value << extract_shammt64!(imm);
        },
        ImmOp64::SRXI => {
            if (imm >> 10) & 1 == 1 {
                value = value >> extract_shammt64!(imm);
            } else {
                value = ( (value as u64) >> extract_shammt64!(imm) ) as i64;
            }
        }
        _ => {
            //#[cfg(debug_assertions)]{dbg!(inst.itype().funct3());}
            return;
        },
    }

    vm.register_write(inst.itype().rd(), value as u64);

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }

    dispatch!(vm);
}
