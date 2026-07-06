use crate::{
    riscvm::{
        api::{
            Packet,
        },
        types::*,
        enums::*,
        macros::*,
        helpers::*,
        instruction::*,
    },
};

use misc::utils::{
    key_expansion,
    KeyWrapper
};

const REGISTER_CNT: usize = 32;

pub struct VmState {
    pub pc          : *mut u8,
    pub registers   : [u64;REGISTER_CNT],
    pub stack       : Box<[u8]>,
    pub offset      : u64,
    pub keys        : Option<KeyWrapper>,
    pub pckt        : Packet,
    pub handler     : [Handler;32],
    pub vm_exit     : u32,
}

static HANDLERS: [Handler; 32] = build_table();

impl VmState {
    #[inline(always)]
    pub(crate) fn register_write(&mut self, idx: u32, value: u64) -> () {
        if idx != Registers::Zero as u32 {
            self.registers[idx as usize] = value;
        }
    }

    #[inline(always)]
    pub(crate) fn register_read(&mut self, idx: u32) -> u64 {
        return self.registers[idx as usize];
    }

    pub fn new(code: &mut [u8], keys: Option<KeyWrapper>, stack_size: usize) -> Self {

        let mut ctx: Self = Self { 
            pc          : code.as_mut_ptr(),
            registers   : [0u64; 32],
            stack       : vec![0u8 ; stack_size].into_boxed_slice(),   // heap
            offset      : 0u64,
            keys        : keys,
            pckt        : Packet {
                            base: code.as_mut_ptr() as u64,
                            length: code.len() as u64,
                        },
            handler     : HANDLERS,
            vm_exit     : ((Syscall::EBreak as u32) << 20) | (Rv64IM::Syscall as u32) // ebreak instruction
        };


        let tmp: *const u32 = &ctx.vm_exit as *const u32;
        let vm_exit_loc: *mut u8 = tmp as *mut u8;

        let delta: usize = (vm_exit_loc as isize - ctx.pckt.base as isize).unsigned_abs();

        ctx.registers[Registers::Sp as usize] = ctx.stack.as_mut_ptr().wrapping_add(ctx.stack.len()) as u64;
        ctx.registers[Registers::A0 as usize] = (&ctx.pckt) as *const _ as u64;
        ctx.registers[Registers::Ra as usize] = vm_exit_loc as u64;

        if let Some(ref mut k) = ctx.keys {
            key_expansion(&mut k.key, &mut k.round_key);
        }

        ctx.vm_exit = xcrypt_instr(vm_exit_loc, &mut ctx.keys, delta);

        ctx.handler = HANDLERS;
        return ctx;
    }

    pub fn is_virtual_fn(&mut self, address: u64) -> bool {
        return 
            address > self.pckt.base &&
            address < self.pckt.base + self.pckt.length
        ;
    }

    #[inline(never)]
    pub unsafe fn run(&mut self, args: *const ()) -> () {

        self.registers[Registers::A1 as usize] = if args.is_null() {0}
                                                else {args as u64}; 

        unsafe {
            let inst: Instruction = Instruction(0);
            self.thread(inst);
        }
    }

    #[inline(always)]
    unsafe extern "C" fn thread(&mut self, _inst: Instruction) -> () {
        dispatch!(self);
    }

    pub fn return_value(&mut self) -> u64 {
        return self.register_read(Registers::A0 as u32);
    }

}
