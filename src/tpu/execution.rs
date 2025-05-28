use crate::shared::{ExecuteResult, Instruction};
use crate::tpu::{TPU, alu, flow, io_matrix, mmu};

pub fn execute(tpu: &mut TPU, instruction: &Instruction, _: u16) -> ExecuteResult {
    let result = match instruction {
        // Stack operations
        Instruction::PUSH(source) => mmu::op_push(tpu, source),
        Instruction::POP(target) => mmu::op_pop(tpu, target),
        Instruction::PEEK(target, source) => mmu::op_peek(tpu, target, source),
        Instruction::SCR => mmu::op_scr(tpu),
        Instruction::RSP(target) => mmu::op_rsp(tpu, target),

        // Networking
        Instruction::XMIT(target, data) => io_matrix::op_xmit(tpu, target, data),
        Instruction::RECV => io_matrix::op_recv(tpu),
        Instruction::TXBS => io_matrix::op_txbs(tpu),
        Instruction::RXBS => io_matrix::op_rxbs(tpu),
        Instruction::WRX => TPU::op_wrx(tpu),

        // Arithmetic
        Instruction::ADD(left, right) => alu::op_add(tpu, left, right),
        Instruction::SUB(left, right) => alu::op_sub(tpu, left, right),
        Instruction::MUL(left, right) => alu::op_mul(tpu, left, right),
        Instruction::DIV(left, right) => alu::op_div(tpu, left, right),
        Instruction::MOD(left, right) => alu::op_mod(tpu, left, right),
        Instruction::AND(left, right) => alu::op_and(tpu, left, right),
        Instruction::OR(left, right) => alu::op_or(tpu, left, right),
        Instruction::XOR(left, right) => alu::op_xor(tpu, left, right),
        Instruction::NOT(value) => alu::op_not(tpu, value),
        Instruction::INC(target) => alu::op_inc(tpu, target),
        Instruction::DEC(target) => alu::op_dec(tpu, target),

        // Bitwise
        Instruction::SLL(target, source, shift) => alu::op_sll(tpu, target, source, shift),
        Instruction::SLR(target, source, shift) => alu::op_slr(tpu, target, source, shift),
        Instruction::SLC(target, source, shift) => alu::op_slc(tpu, target, source, shift),
        Instruction::SRC(target, source, shift) => alu::op_src(tpu, target, source, shift),
        Instruction::ROL(target, source, shift) => alu::op_rol(tpu, target, source, shift),
        Instruction::ROR(target, source, shift) => alu::op_ror(tpu, target, source, shift),

        // Memory/Register Data movement
        Instruction::RCY(target, source) => mmu::op_rcy(tpu, target, source),
        Instruction::RMV(target, source) => mmu::op_rmv(tpu, target, source),
        Instruction::LDR(target, source) => mmu::op_ldr(tpu, target, source),
        Instruction::LDM(target, source) => mmu::op_ldm(tpu, target, source),
        Instruction::LDO(target, source, offset) => mmu::op_ldo(tpu, target, source, offset),
        Instruction::LDOI(target, source, offset) => mmu::op_ldoi(tpu, target, source, offset),
        Instruction::STM(target, source) => mmu::op_stm(tpu, target, source),
        Instruction::STMO(target, source, offset) => mmu::op_stmo(tpu, target, source, offset),
        Instruction::SMOI(target, source, offset) => mmu::op_smoi(tpu, target, source, offset),

        // Digital I/O
        Instruction::DPW(target, source) => io_matrix::op_dpw(tpu, target, source),
        // Instruction::DPWH => io_matrix::op_dpwh(tpu, operands),
        Instruction::DPR(target, source) => io_matrix::op_dpr(tpu, target, source),
        Instruction::DPWW(value) => io_matrix::op_dpww(tpu, value),
        Instruction::DPRW(target) => io_matrix::op_dprw(tpu, target),

        // Analog I/O
        Instruction::APW(target, source) => io_matrix::op_apw(tpu, target, source),
        // Instruction::APWH => io_matrix::op_apwh(tpu, operands),
        Instruction::APR(target, source) => io_matrix::op_apr(tpu, target, source),

        // Misc
        Instruction::SLP(value) => tpu.op_slp(value),
        Instruction::NOP => TPU::op_nop(),
        Instruction::HLT => TPU::op_hlt(),

        // Branching - Absolute
        Instruction::JMP(target) => flow::op_jmp(tpu, target),
        Instruction::BEZ(target, source) => flow::op_bez(tpu, target, source),
        Instruction::BNZ(target, source) => flow::op_bnz(tpu, target, source),
        Instruction::BEQ(target, source, value) => flow::op_beq(tpu, target, source, value),
        Instruction::BNE(target, source, value) => flow::op_bne(tpu, target, source, value),
        Instruction::BGE(target, source, value) => flow::op_bge(tpu, target, source, value),
        Instruction::BLE(target, source, value) => flow::op_ble(tpu, target, source, value),
        Instruction::BGT(target, source, value) => flow::op_bgt(tpu, target, source, value),
        Instruction::BLT(target, source, value) => flow::op_blt(tpu, target, source, value),

        // Branching - Relative
        Instruction::JPR(target) => flow::op_jpr(tpu, target),
        Instruction::BREZ(target, source) => flow::op_brez(tpu, target, source),
        Instruction::BRNZ(target, source) => flow::op_brnz(tpu, target, source),
        Instruction::BREQ(target, source, value) => flow::op_breq(tpu, target, source, value),
        Instruction::BRNE(target, source, value) => flow::op_brne(tpu, target, source, value),
        Instruction::BRGE(target, source, value) => flow::op_brge(tpu, target, source, value),
        Instruction::BRLE(target, source, value) => flow::op_brle(tpu, target, source, value),
        Instruction::BRGT(target, source, value) => flow::op_brgt(tpu, target, source, value),
        Instruction::BRLT(target, source, value) => flow::op_brlt(tpu, target, source, value),

        // Subroutines
        Instruction::JSR(target) => flow::op_jsr(tpu, target),
        Instruction::RTS => flow::op_rts(tpu),
    };
    result
}
