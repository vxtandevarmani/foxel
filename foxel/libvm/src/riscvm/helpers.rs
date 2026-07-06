use crate::{
    riscvm::{
        enums::*,
        types::*,
    },
};

use misc::utils::{
    KeyWrapper,
    aes_ctr_xcryption,
};

use super::handlers::*;

#[allow(unused)]


pub const unsafe fn memcpy(dst: *mut u8, src: *const u8, len: usize) -> () {
    let mut i = 0;
    while i < len {
        unsafe {
            *dst.add(i) = *src.add(i);
        }
        i += 1;
    }
    return;
}

/**
 * bits_to_opcode() =>  A helper function used to convert 
 *                      opcodes into their enum represenation
 *                      for match statements in a switch-loop
 *                      based dispatch subroutine however, a more
 *                      efficient method of dispatch was used for
 *                      performance [ref1] enhancements and making
 *                      reverse engineering a tad bit tedious
 * 
 * [ref1]
 * ==>  So the performance increase is due to leveraging the cpu pipeline
 *      to handle instructions since this method technically "unrolls" the
 *      loop making Out-Of-Order Execution (OoO) more viable and also removes
 *      conditional jumps which hinders Branch Prediction since the predictor
 *      cannot reliably predict which path to take and if it guesses wrong it
 *      is forced to flush the Branch Target Buffer (BTB)
*/

pub(crate) fn bits_to_opcode(bits: u32) -> Rv64IM {
    match bits {
        0b0110111 =>{return Rv64IM::Lui;    },
        0b0010111 =>{return Rv64IM::Auipc;  },
        0b1101111 =>{return Rv64IM::Jal;    },
        0b1100111 =>{return Rv64IM::Jalr;   },
        0b1100011 =>{return Rv64IM::Branch; },
        0b0000011 =>{return Rv64IM::Load;   },
        0b0100011 =>{return Rv64IM::Store;  },
        0b0010011 =>{return Rv64IM::ImmOp64;},
        0b0110011 =>{return Rv64IM::RgOp64; },
        0b0001111 =>{return Rv64IM::Fence;  },
        0b1110011 =>{return Rv64IM::Syscall;},
        0b0011011 =>{return Rv64IM::ImmOp32;},
        0b0111011 =>{return Rv64IM::RegOp32;},
        _         =>{return Rv64IM::Invalid;}
    }
}

pub(crate) fn bits_to_branch(bits: u32) -> Branch {
    match bits {
        0b000 => {return Branch::BEQ; },
        0b001 => {return Branch::BNE; },
        0b100 => {return Branch::BLT; },
        0b101 => {return Branch::BGE; },
        0b110 => {return Branch::BLTU;},
        0b111 => {return Branch::BGEU;},
        _     => {return Branch::BINV;},
    }
}

pub(crate) fn bits_to_load(bits: u32) -> Load {
    match bits {
        0b000   => {return Load::LB;  },
        0b001   => {return Load::LH;  },
        0b010   => {return Load::LW;  },
        0b100   => {return Load::LBU; },
        0b101   => {return Load::LHU; },
        0b110   => {return Load::LWU; },
        0b011   => {return Load::LD;  },
        _       => {return Load::LINV;},
    }
}

pub(crate) fn bits_to_store(bits: u32) -> Store {
    match bits {
        0b000   => {return Store::SB;  },
        0b001   => {return Store::SH;  },
        0b010   => {return Store::SW;  },
        0b011   => {return Store::SD;  },
        _       => {return Store::SINV;}, 
    }
}

pub(crate) fn bits_to_immop64(bits: u32) -> ImmOp64 {
    match bits {
        0b000   => {return ImmOp64::ADDI; },
        0b010   => {return ImmOp64::SLTI; },
        0b011   => {return ImmOp64::SLTIU;},
        0b100   => {return ImmOp64::XORI; },
        0b110   => {return ImmOp64::ORI;  },
        0b111   => {return ImmOp64::ANDI; },
        0b001   => {return ImmOp64::SLLI; },
        0b101   => {return ImmOp64::SRXI; },
        _       => {return ImmOp64::INVI; },
    }
}

pub(crate) fn bits_to_rgop64(bits: u32) -> RgOp64 {
    match bits {
        0b000 => {return RgOp64::ADDSUB;},
        0b001 => {return RgOp64::SLL;   },
        0b010 => {return RgOp64::SLT;   },
        0b011 => {return RgOp64::SLTU;  },
        0b100 => {return RgOp64::XOR;   },
        0b101 => {return RgOp64::SRX;   },
        0b110 => {return RgOp64::OR;    },
        0b111 => {return RgOp64::AND;   },
        _     => {return RgOp64::INV;   },
    }
}

pub(crate) fn bits_to_immop32(bits: u32) -> ImmOp32 {
    match bits {
        0b000 => {return ImmOp32::ADDIW;},
        0b001 => {return ImmOp32::SLLIW;},
        0b101 => {return ImmOp32::SRXIW;},
        _     => {return ImmOp32::INVIW;},
    }
}

pub(crate) fn bits_to_rgop32(bits: u32) -> RgOp32 {
    match bits {
        0b000 => {return RgOp32::ADDSUBW;},
        0b001 => {return RgOp32::SLLW;   },
        0b101 => {return RgOp32::SRXW;   },
        _     => {return RgOp32::INVW;   },
    }
}

pub(crate) fn bits_to_syscall(bits: u32) -> Syscall {
    match bits {
        0b000000000000 => {return Syscall::Ecall;  },
        0b000000000001 => {return Syscall::EBreak; },
        _              => {return Syscall::INVALID;},
    }
}

pub(crate) fn bits_to_m32(bits: u32) -> M32 {
    match bits {
        0b000   => {return M32::MUL;    },
        0b001   => {return M32::MULH;   },
        0b010   => {return M32::MULHSU; },
        0b011   => {return M32::MULHU;  },
        0b100   => {return M32::DIV;    },
        0b101   => {return M32::DIVU;   },
        0b110   => {return M32::REM;    },
        0b111   => {return M32::REMU;   },
        _       => {return M32::INV;    },
    }
}

pub(crate) fn bits_to_m64(bits: u32) -> M64 {
    match bits {
        0b000   => {return M64::MULW;   },
        0b100   => {return M64::DIVW;   },
        0b101   => {return M64::DIVUW;  },
        0b110   => {return M64::REMW;   },
        0b111   => {return M64::REMUW;  },
        _       => {return M64::INV;    },
    }
}

pub(crate) fn bits_to_ssn(bits: u64) -> ServiceNumber {
    match bits {
        0xff => {return ServiceNumber::HOST;},
        0x45 => {return ServiceNumber::GPEB;},
        _    => {return ServiceNumber::INV ;},
    }
}

pub(crate) fn xcrypt_instr( base: *const u8,
                            keys: &mut Option<KeyWrapper>,
                            offset: usize,
) -> u32 {

    unsafe {

        let mut le_instr:[u8;4] = [0;4];
        memcpy(le_instr.as_mut_ptr(), base, size_of::<u32>());

        if let Some(k) = keys {
            aes_ctr_xcryption(&mut le_instr, size_of::<u32>(), k, offset);
        }

        return u32::from_le_bytes(le_instr);
    }
}

pub const fn build_table() -> [Handler ; 32] {
    let mut table: [Handler;32           ] = [invalid_handler;32];
    table[Rv64IM::Lui       as usize >> 2] = lui_handler;
    table[Rv64IM::Auipc     as usize >> 2] = auipc_handler;
    table[Rv64IM::Jal       as usize >> 2] = jal_handler;
    table[Rv64IM::Jalr      as usize >> 2] = jalr_handler;
    table[Rv64IM::Branch    as usize >> 2] = branch_handler;
    table[Rv64IM::Load      as usize >> 2] = load_handler;
    table[Rv64IM::Store     as usize >> 2] = store_handler;
    table[Rv64IM::ImmOp64   as usize >> 2] = immop64_handler;
    table[Rv64IM::RgOp64    as usize >> 2] = rgop64_handler;
    table[Rv64IM::Fence     as usize >> 2] = fence_handler;
    table[Rv64IM::Syscall   as usize >> 2] = syscall_handler;
    table[Rv64IM::ImmOp32   as usize >> 2] = immop32_handler;
    table[Rv64IM::RegOp32   as usize >> 2] = regop32_handler;

    return table;
}

pub const fn sign_extend(imm: u32, size: usize) -> i32 {
    if imm & (1 << (size - 1)) != 0 {
        return (imm | (0xFFFFFFFF << size)) as i32;
    }
    return imm as i32;
}
