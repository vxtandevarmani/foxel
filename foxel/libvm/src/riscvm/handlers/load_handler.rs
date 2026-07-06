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

fn load_handler(
    vm: &mut VmState,
    inst: Instruction
) -> () {

    #[cfg(debug_assertions)]
    {
        vm.trace_state(&inst);
    }

    let addr: u64 = vm.register_read(inst.itype().rs1()) + 
                    sign_extend(inst.itype().imm(), 12)
            as u64;

    let value: u64;
    
    match bits_to_load(inst.itype().funct3()) {
        Load::LB => {
            let tmp: i8 = unsafe {
                (addr as *const i8).read_unaligned()
            };
            value = tmp as i64 as u64;
        },
        Load::LH => {
            let tmp: i16 = unsafe {
                (addr as *const i16).read_unaligned()
            };
            value = tmp as i64 as u64;
        },
        Load::LW => {
            let tmp: i32 = unsafe {
                (addr as *const i32).read_unaligned()
            };
            value = tmp as i64 as u64;
        },
        Load::LBU => {
            let tmp: u8 = unsafe {
                (addr as *const u8).read_unaligned()
            };
            value = tmp as i64 as u64;
        }
        Load::LHU => {
            let tmp: u16 = unsafe {
                (addr as *const u16).read_unaligned()
            };
            value = tmp as i64 as u64;
        },
        Load::LWU => {
            let tmp: u32 = unsafe {
                (addr as *const u32).read_unaligned()
            };
            value = tmp as i64 as u64;
        }
        Load::LD => {
            let tmp: i64 = unsafe {
                (addr as *const i64).read_unaligned()
            };
            value = tmp as i64 as u64;
        }
        _ => {return;},
    }

    vm.register_write(inst.itype().rd(), value);

    unsafe {
        vm.pc = vm.pc.add( size_of::<u32>() );
    }

    dispatch!(vm);
}
