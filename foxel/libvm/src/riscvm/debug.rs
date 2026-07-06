use crate::riscvm::{
    vm::VmState,
    instruction::*,
    helpers::*,
    enums::*,
};

impl VmState {
    pub(crate) fn trace_state(&mut self, inst: &Instruction) {
        eprint!(
        "────────────────────────────────────────────────────────────────────────────────────────────────\n\
         PC: {:p}  │  INST#: {:x}  │  opcode: 0x{:08x}  │  {:?}\n\
         ────────────────────────────────────────────────────────────────────────────────────────────────\n",
        self.pc,
        unsafe { self.pc.offset_from(self.pckt.base as *const u8) },
        inst.raw(),
        bits_to_opcode(inst.opcode())
    );

        eprint!("[special]\n");
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::Zero, self.registers[Registers::Zero as usize],
            Registers::Ra,   self.registers[Registers::Ra   as usize],
            Registers::Sp,   self.registers[Registers::Sp   as usize],
            Registers::Gp,   self.registers[Registers::Gp   as usize],
        );

        eprint!("[temp]\n");
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::Tp, self.registers[Registers::Tp as usize],
            Registers::T0, self.registers[Registers::T0 as usize],
            Registers::T1, self.registers[Registers::T1 as usize],
            Registers::T2, self.registers[Registers::T2 as usize],
        );
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::T3, self.registers[Registers::T3 as usize],
            Registers::T4, self.registers[Registers::T4 as usize],
            Registers::T5, self.registers[Registers::T5 as usize],
            Registers::T6, self.registers[Registers::T6 as usize],
        );

        eprint!("[saved]\n");
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::S0FP, self.registers[Registers::S0FP as usize],
            Registers::S1,   self.registers[Registers::S1   as usize],
            Registers::S2,   self.registers[Registers::S2   as usize],
            Registers::S3,   self.registers[Registers::S3   as usize],
        );
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::S4, self.registers[Registers::S4 as usize],
            Registers::S5, self.registers[Registers::S5 as usize],
            Registers::S6, self.registers[Registers::S6 as usize],
            Registers::S7, self.registers[Registers::S7 as usize],
        );
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::S8,  self.registers[Registers::S8  as usize],
            Registers::S9,  self.registers[Registers::S9  as usize],
            Registers::S10, self.registers[Registers::S10 as usize],
            Registers::S11, self.registers[Registers::S11 as usize],
        );

        eprint!("[args]\n");
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::A0, self.registers[Registers::A0 as usize],
            Registers::A1, self.registers[Registers::A1 as usize],
            Registers::A2, self.registers[Registers::A2 as usize],
            Registers::A3, self.registers[Registers::A3 as usize],
        );
        eprint!("{:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x} │ {:?}\t{:016x}\n",
            Registers::A4, self.registers[Registers::A4 as usize],
            Registers::A5, self.registers[Registers::A5 as usize],
            Registers::A6, self.registers[Registers::A6 as usize],
            Registers::A7, self.registers[Registers::A7 as usize],
        );

        self.offset += size_of::<u32>() as u64;
    }
}