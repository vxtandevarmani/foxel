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

fn immop32_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    let rs1: i32 = vm.register_read(inst.itype().rs1()) as i32;
    let imm: i32 = sign_extend(inst.itype().imm(), 12);
    
    let value: i32;

    match bits_to_immop32(inst.itype().funct3()) {
        ImmOp32::ADDIW => {
            value = rs1 + imm;
        },
        ImmOp32::SLLIW => {
            value = rs1 << extract_shammt32!(imm);
        },
        ImmOp32::SRXIW => {
            if (imm >> 10) & 1 != 0 {
                value = rs1 >> extract_shammt32!(imm);
            } else {
                value = ( rs1 as u32 >> extract_shammt32!(imm) as u32 )
                        as i32;
            }
        },
        _ => {return;}
    }

    vm.register_write(inst.itype().rd(), value as i64 as u64);

    unsafe {
        vm.pc = vm.pc.add(size_of::<u32>());
    }

    dispatch!(vm);
}
