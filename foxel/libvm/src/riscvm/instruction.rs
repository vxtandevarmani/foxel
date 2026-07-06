
pub struct RType(u32);
impl RType {
    #[inline(always)]
    pub fn rd(&self) -> u32 {
        return (self.0 >> 7) & 0x1f;
    }
    #[inline(always)]
    pub fn funct3(&self) -> u32 {
        return (self.0 >> 12) & 0x7;
    }
    #[inline(always)]
    pub fn rs1(&self) -> u32 {
        return (self.0 >> 15) & 0x1f;
    }
    #[inline(always)]
    pub fn rs2(&self) -> u32 {
        return (self.0 >> 20) & 0x1f;
    }
    #[inline(always)]
    pub fn funct7(&self) -> u32 {
        return (self.0 >> 25) & 0x7f
    }
}

pub struct IType(u32);
impl IType {
    #[inline(always)]
    pub fn rd(&self) -> u32 {
        return (self.0 >> 7) & 0x1f;
    }
    #[inline(always)]
    pub fn funct3(&self) -> u32 {
        return (self.0 >> 12) & 0x7;
    }
    #[inline(always)]
    pub fn rs1(&self) -> u32 {
        return (self.0 >> 15) & 0x1f;
    }
    #[inline(always)]
    pub fn imm(&self) -> u32 {
        return (self.0 >> 20) & 0xfff;
    }
}

pub struct SType(u32);
impl SType {
    #[inline(always)]
    pub fn imm_4_0(&self) -> u32 {
        return (self.0 >> 7) & 0x1f;
    }
    #[inline(always)]
    pub fn funct3(&self) -> u32 {
        return (self.0 >> 12) & 0x7;
    }
    #[inline(always)]
    pub fn rs1(&self) -> u32 {
        return (self.0 >> 15) & 0x1f;
    }
    #[inline(always)]
    pub fn rs2(&self) -> u32 {
        return (self.0 >> 20) & 0x1f;
    }
    #[inline(always)]
    pub fn imm_11_5(&self) -> u32 {
        return (self.0 >> 25) & 0x7f;
    }
}
pub struct UType(u32);
impl UType {
    #[inline(always)]
    pub fn rd(&self) -> u32 {
        return (self.0 >> 7) & 0x1f;
    }
    #[inline(always)]
    pub fn imm(&self) ->u32 {
        return (self.0 >> 12) & 0xfffff;
    }
}
pub struct SBType(u32);
impl SBType {
    #[inline(always)]
    pub fn imm_11(&self) -> u32 {
        return (self.0 >> 7) & 0x1;
    }
    #[inline(always)]
    pub fn imm_4_1(&self) -> u32 {
        return (self.0 >> 8) & 0xf;
    }
    #[inline(always)]
    pub fn funct3(&self) -> u32 {
        return (self.0 >> 12) & 0x7;
    }
    #[inline(always)]
    pub fn rs1(&self) -> u32 {
        return (self.0 >> 15) & 0x1f;
    }
    #[inline(always)]
    pub fn rs2(&self) -> u32 {
        return (self.0 >> 20) & 0x1f;
    }
    #[inline(always)]
    pub fn imm_10_5(&self) -> u32 {
        return (self.0 >> 25) & 0x3f;
    }
    #[inline(always)]
    pub fn imm_12(&self) -> u32 {
        return (self.0 >> 31) & 0x1;
    }
}
pub struct UJType(u32);
impl UJType {
    #[inline(always)]
    pub fn rd(&self) -> u32 {
        return (self.0 >> 7) & 0x1f;
    }
    #[inline(always)]
    pub fn imm_19_12(&self) -> u32 {
        return (self.0 >> 12) & 0xff;
    }
    #[inline(always)]
    pub fn imm_11(&self) -> u32 {
        return (self.0 >> 20) & 0x1;
    }
    #[inline(always)]
    pub fn imm_4_1(&self) -> u32 {
        return (self.0 >> 21) & 0x3ff;
    }
    #[inline(always)]
    pub fn imm_20(&self) -> u32 {
        return (self.0 >> 31) & 0x1;
    }
}
#[repr(C)]
pub struct Instruction(pub u32);

impl Instruction {

    #[inline(always)]
    pub fn raw(&self) -> u32 {
        return self.0;
    }

    #[inline(always)]
    pub fn opcode(&self) -> u32 {
        return self.0 & 0x7f;
    }

    #[inline(always)]
    pub fn rtype(&self) -> RType {
        return RType(self.0);
    }

    #[inline(always)]
    pub fn itype(&self) -> IType {
        return IType(self.0);
    }

    #[inline(always)]
    pub fn stype(&self) -> SType {
        return SType(self.0);
    }

    #[inline(always)]
    pub fn utype(&self) -> UType {
        return UType(self.0);
    }

    #[inline(always)]
    pub fn sbtype(&self) -> SBType {
        return SBType(self.0);
    }

    #[inline(always)]
    pub fn ujtype(&self) -> UJType {
        return UJType(self.0);
    }

}