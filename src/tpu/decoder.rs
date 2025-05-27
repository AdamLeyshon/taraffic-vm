use crate::shared::{DecodeResult, Instruction};
use crate::tpu::flow::decode;
use crate::tpu::{TPU, mmu};
use crate::tpu::{alu, io_matrix};
use std::rc::Rc;
use tracing::trace;

pub fn decode(instruction: &Rc<Instruction>) -> DecodeResult {
    trace!("DECODE: {instruction:?}");

    match &**instruction {
        // Stack operations
        Instruction::PUSH(operand) => mmu::decode::decode_op_push(operand),
        Instruction::POP(_) => mmu::decode::decode_op_pop(),
        Instruction::PEEK(_, index) => mmu::decode::decode_op_peek(index),
        Instruction::SCR => mmu::decode::decode_op_scr(),
        Instruction::RSP(_) => mmu::decode::decode_op_rsp(),

        // Networking
        Instruction::XMIT(_, _) => io_matrix::decode::decode_op_xmit(),
        Instruction::RECV => io_matrix::decode::decode_op_recv(),
        Instruction::TXBS => io_matrix::decode::decode_op_txbs(),
        Instruction::RXBS => io_matrix::decode::decode_op_rxbs(),

        // Arithmetic
        Instruction::ADD(_, _) => alu::decode::decode_op_add(),
        Instruction::SUB(_, _) => alu::decode::decode_op_sub(),
        Instruction::MUL(_, _) => alu::decode::decode_op_mul(),
        Instruction::DIV(_, _) => alu::decode::decode_op_div(),
        Instruction::MOD(_, _) => alu::decode::decode_op_mod(),
        Instruction::AND(_, _) => alu::decode::decode_op_and(),
        Instruction::OR(_, _) => alu::decode::decode_op_or(),
        Instruction::XOR(_, _) => alu::decode::decode_op_xor(),
        Instruction::NOT(_) => alu::decode::decode_op_not(),
        Instruction::INC(_) => alu::decode::decode_op_inc(),
        Instruction::DEC(_) => alu::decode::decode_op_dec(),

        // Bitwise
        Instruction::SLL(_, _, shift) => alu::decode::decode_op_sll(shift),
        Instruction::SLC(_, _, shift) => alu::decode::decode_op_slc(shift),
        Instruction::SLR(_, _, shift) => alu::decode::decode_op_slr(shift),
        Instruction::SRC(_, _, shift) => alu::decode::decode_op_src(shift),
        Instruction::ROL(_, _, shift) => alu::decode::decode_op_rol(shift),
        Instruction::ROR(_, _, shift) => alu::decode::decode_op_ror(shift),

        // Memory/Register Data movement
        Instruction::RCY(_, _) => mmu::decode::decode_op_rcy(),
        Instruction::RMV(_, _) => mmu::decode::decode_op_rmv(),
        Instruction::LDR(target, source) => mmu::decode::decode_op_ldr(target, source),
        Instruction::LDO(_, source, _) => mmu::decode::decode_op_ldo(source),
        Instruction::LDOI(_, source, _) => mmu::decode::decode_op_ldoi(source),
        Instruction::STM(_, source) => mmu::decode::decode_op_stm(source),
        Instruction::STMO(_, source, _) => mmu::decode::decode_op_stmo(source),
        Instruction::SMOI(_, source, _) => mmu::decode::decode_op_smoi(source),

        // Digital I/O
        Instruction::DPW(target, value) => io_matrix::decode::decode_op_dpw(target, value),
        // Instruction::DPWH => io_matrix::decode::decode_op_dpwh(operands),
        Instruction::DPR(_, source) => io_matrix::decode::decode_op_dpr(source),
        Instruction::DPWW(value) => io_matrix::decode::decode_op_dpww(value),
        Instruction::DPRW(_) => io_matrix::decode::decode_op_dprw(),

        // Analog I/O
        Instruction::APW(target, source) => io_matrix::decode::decode_op_apw(target, source),
        // Instruction::APWH => io_matrix::decode::decode_op_apwh(operands),
        Instruction::APR(_, source) => io_matrix::decode::decode_op_apr(source),

        // Misc
        Instruction::NOP => TPU::decode_op_nop(),
        Instruction::SLP(_) => TPU::decode_op_slp(),
        Instruction::WRX => TPU::decode_op_wrx(),
        Instruction::HLT => TPU::decode_op_hlt(),

        // Branching - Absolute
        Instruction::JMP(target) => decode::decode_op_jmp(target),
        Instruction::BEZ(_, _) => decode::decode_op_bez(),
        Instruction::BNZ(_, _) => decode::decode_op_bnz(),
        Instruction::BEQ(_, _, _) => decode::decode_op_beq(),
        Instruction::BNE(_, _, _) => decode::decode_op_bne(),
        Instruction::BGE(_, _, _) => decode::decode_op_bge(),
        Instruction::BLE(_, _, _) => decode::decode_op_ble(),
        Instruction::BGT(_, _, _) => decode::decode_op_bgt(),
        Instruction::BLT(_, _, _) => decode::decode_op_blt(),

        // Branching - Relative
        Instruction::JPR(target) => decode::decode_op_jpr(target),
        Instruction::BREZ(_, _) => decode::decode_op_brez(),
        Instruction::BRNZ(_, _) => decode::decode_op_brnz(),
        Instruction::BREQ(_, _, _) => decode::decode_op_breq(),
        Instruction::BRNE(_, _, _) => decode::decode_op_brne(),
        Instruction::BRGE(_, _, _) => decode::decode_op_brge(),
        Instruction::BRLE(_, _, _) => decode::decode_op_brle(),
        Instruction::BRGT(_, _, _) => decode::decode_op_brgt(),
        Instruction::BRLT(_, _, _) => decode::decode_op_brlt(),

        // Subroutines
        Instruction::JSR(target) => decode::decode_op_jsr(target),
        Instruction::RTS => decode::decode_op_rts(),
    }
}
