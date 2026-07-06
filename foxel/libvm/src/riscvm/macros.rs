macro_rules! extract_shammt64 {
    ($imm:expr) => {
        $imm & 0b111111
    };
}

macro_rules! extract_shammt32 {
    ($imm:expr) => {
        $imm & 0b111111
    };
}

macro_rules! check_bit {
    ($imm:expr, $pos:expr) => {
        $imm & (1 << $pos) != 0
    };
}

macro_rules! fetch {
    ($vm:expr) => {
        $crate::riscvm::helpers::xcrypt_instr(
            $vm.pc,
            &mut $vm.keys,
            ($vm.pc as usize).wrapping_sub($vm.pckt.base as usize),
        )
    };
}

macro_rules! dispatch {
    ($vm:expr) => {{
        let raw : u32           = fetch!($vm);
        let inst: Instruction   = Instruction(raw);
        let addr: Handler       = $vm.handler[ (inst.opcode() >> 2) as usize ];
        unsafe{addr($vm, inst);}
  }};
}

pub(crate) use extract_shammt32;
pub(crate) use extract_shammt64;
pub(crate) use check_bit;
pub(crate) use fetch;
pub(crate) use dispatch;
