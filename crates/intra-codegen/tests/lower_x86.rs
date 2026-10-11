use intra_codegen::{arch::{backend::x86::{X86Label, X86Operand, instr::X86Instr, reg::{X86Reg, X86RegClass}}, lower_ir_x86}, ir::{Block, CallingConv, Function, Module, Symbol, data::{Instruction, Operand, RegisterClass, ScalarType, Terminator, VReg}, opcodes::Opcode}};

#[test]
fn lowers_single_add_function() {
    let regs = [RegisterClass::Scalar(ScalarType::I32); 3];

    let a = VReg(0);
    let b = VReg(1);
    let result = VReg(2);

    let func = Function {
        sym: Symbol::new("add"),
        params: Box::new([regs[a.0], regs[b.0]]),
        out: Some(RegisterClass::Scalar(ScalarType::I32)),
        calling_conv: CallingConv::SysV64,

        blocks: Box::new([
            Block {
                instrs: Box::new([
                    Instruction {
                        opcode: Opcode::CoreAddI,
                        inputs: Box::new([
                            Operand::Reg(a),
                            Operand::Reg(b),
                        ]),
                        output: Some(result),
                    },
                ]),
                term: Terminator::Return(Some(result)),
            },
        ]),
    };

    let module = Module {
        name: Symbol::new("test"),
        funcs: Box::new([func]),
        data: Box::new([]),
        regs: regs.into(),
    };

    let x86 = lower_ir_x86(&module).unwrap();

    assert_eq!(x86.funcs.len(), 1);
    assert_eq!(x86.funcs[0].sym, Symbol::new("add"));

    let blocks = &x86.funcs[0].blocks;
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].label, X86Label(0));

    assert_eq!(
        blocks[0].instrs.as_ref(),
        &[
            X86Instr::Mov {
                dst: X86Reg {
                    id: 0,
                    class: X86RegClass::Gpr32,
                },
                src: X86Operand::Reg(X86Reg {
                    id: 2,
                    class: X86RegClass::Gpr32,
                }),
            },
            X86Instr::Add {
                dst: X86Reg {
                    id: 0,
                    class: X86RegClass::Gpr32,
                },
                src: X86Operand::Reg(X86Reg {
                    id: 3,
                    class: X86RegClass::Gpr32,
                }),
            },
            X86Instr::Ret {
                value: Some(X86Reg {
                    id: 0,
                    class: X86RegClass::Gpr32,
                }),
            },
        ],
    );
}
