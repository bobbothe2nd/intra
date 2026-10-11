use crate::{arch::backend::x86::{X86Lowerer, X86Operand, reg::{X86Reg, X86RegClass}}, error::Error, ir::data::{Constant, Instruction, Operand, VReg}};

impl X86Lowerer<'_> {
    fn alloc_physical(
        &mut self,
        class: X86RegClass,
    ) -> Result<X86Reg, Error> {
        const ALLOCATABLE_GPRS: &[u32] = &[
            0,    2,  3,  4,  5,
            8, 9, 10, 11, 12, 13, 14, 15,
        ];

        let used = self
            .allocs
            .iter()
            .flatten()
            .filter(|reg| reg.class == class)
            .count();

        let id = match class {
            X86RegClass::Gpr64
            | X86RegClass::Gpr32
            | X86RegClass::Gpr16 => {
                *ALLOCATABLE_GPRS
                    .get(used)
                    .ok_or(lower_error!(@RegisterExhaustion "GPR"))?
            }

            _ => {
                used as u32
            }
        };

        Ok(X86Reg { id, class })
    }

    pub fn alloc(&mut self, reg: VReg) -> Result<X86Reg, Error> {
        if let Some(reg) = self.allocs[reg.0] {
            return Ok(reg);
        }

        let class = self.module.regs[reg.0].try_into()?;

        let physical = self.alloc_physical(class)?;

        self.allocs[reg.0] = Some(physical);

        Ok(physical)
    }

    pub fn alloc_out(&mut self, instr: &Instruction) -> Result<X86Reg, Error> {
        match instr.output {
            Some(reg) => self.alloc(reg),
            None => Err(lower_error!(@MissingOutput)),
        }
    }

    pub fn alloc_opt(&mut self, reg: Option<VReg>) -> Result<Option<X86Reg>, Error> {
        Ok(match reg {
            Some(reg) => Some(self.alloc(reg)?),
            None => None,
        })
    }

    pub fn alloc_operand(&mut self, operand: &Operand) -> Result<X86Operand, Error> {
        match operand {
            Operand::Reg(reg) => Ok(X86Operand::Reg(self.alloc(*reg)?)),

            Operand::Imm(Constant::Int { val, .. }) => {
                Ok(X86Operand::Imm(*val))
            }

            _ => Err(lower_error!(@InvalidOperands)),
        }
    }
}
