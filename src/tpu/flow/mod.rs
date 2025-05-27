pub mod decode;
#[cfg(test)]
mod flow_test;

use crate::shared::Register;
use crate::shared::{ExecuteResult, HaltReason, OperandValueType};
use crate::tpu::TPU;

pub fn op_jmp(tpu: &mut TPU, target: &OperandValueType) -> ExecuteResult {
    let address = tpu.get_operand_value(target) as usize;
    set_program_counter_conditionally(tpu, true, address)
}

#[inline]
fn set_program_counter_conditionally(
    tpu: &mut TPU,
    condition: bool,
    address: usize,
) -> ExecuteResult {
    // Check if the value is zero
    let target = if condition {
        address
    } else {
        tpu.tpu_state.program_counter + 1
    };

    // Check if the address is valid
    if target > (tpu.tpu_state.rom.len() - 1) {
        return ExecuteResult::Halt(HaltReason::InvalidPC);
    }

    tpu.tpu_state.program_counter = target;
    ExecuteResult::PCModified
}

pub fn op_bez(tpu: &mut TPU, target: &OperandValueType, source: &Register) -> ExecuteResult {
    // Get the branch address and value
    let address = tpu.get_operand_value(target) as usize;
    let value = tpu.read_register(*source);
    set_program_counter_conditionally(tpu, value == 0, address)
}

pub fn op_bnz(tpu: &mut TPU, target: &OperandValueType, source: &Register) -> ExecuteResult {
    // Get the branch address and value
    let address = tpu.get_operand_value(target) as usize;
    let value = tpu.read_register(*source);

    set_program_counter_conditionally(tpu, value != 0, address)
}

pub fn op_beq(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    // Get the branch address and values
    let address = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);
    set_program_counter_conditionally(tpu, a == b, address)
}

pub fn op_bne(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    // Get the branch address and values
    let address = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);
    set_program_counter_conditionally(tpu, a != b, address)
}

pub fn op_bge(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    // Get the branch address and values
    let address = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);
    set_program_counter_conditionally(tpu, a >= b, address)
}

pub fn op_ble(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    // Get the branch address and values
    let address = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);
    set_program_counter_conditionally(tpu, a <= b, address)
}

pub fn op_bgt(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    // Get the branch address and values
    let address = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);
    set_program_counter_conditionally(tpu, a > b, address)
}

pub fn op_blt(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    // Get the branch address and values
    let address = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);
    set_program_counter_conditionally(tpu, a < b, address)
}

// Relative Branches
pub fn op_jpr(tpu: &mut TPU, target: &OperandValueType) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, true, new_pc)
}

pub fn op_brez(tpu: &mut TPU, target: &OperandValueType, source: &Register) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let value = tpu.read_register(*source);
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, value == 0, new_pc)
}

pub fn op_brnz(tpu: &mut TPU, target: &OperandValueType, source: &Register) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let value = tpu.read_register(*source);
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, value != 0, new_pc)
}

pub fn op_breq(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);

    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a == b, new_pc)
}

pub fn op_brne(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);

    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a != b, new_pc)
}

pub fn op_brge(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);

    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a >= b, new_pc)
}

pub fn op_brle(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);

    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a <= b, new_pc)
}

pub fn op_brgt(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);

    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a > b, new_pc)
}

pub fn op_brlt(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &Register,
    value: &OperandValueType,
) -> ExecuteResult {
    let offset = tpu.get_operand_value(target) as usize;
    let a = tpu.read_register(*source);
    let b = tpu.get_operand_value(value);

    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a < b, new_pc)
}

// Subroutines
pub fn op_jsr(tpu: &mut TPU, target: &OperandValueType) -> ExecuteResult {
    // Get the subroutine address
    let address = tpu.get_operand_value(target);

    // Validate we have enough space on the stack
    if tpu.tpu_state.stack.len() == TPU::STACK_SIZE {
        return ExecuteResult::Halt(HaltReason::StackOverflow);
    }

    let old_pc = tpu.tpu_state.program_counter;

    let result = set_program_counter_conditionally(tpu, true, address as usize);

    if matches!(result, ExecuteResult::PCModified) {
        // Only push the return address if we've validated the landing address
        // And modified the program counter
        tpu.push(old_pc as u16);
    }
    result
}

pub fn op_rts(tpu: &mut TPU) -> ExecuteResult {
    // Pop the return address from the stack
    let address = tpu.pop() as usize;
    set_program_counter_conditionally(tpu, true, address)
}
