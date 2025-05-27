pub mod decode;
#[cfg(test)]
mod mmu_test;

use crate::shared::Register;
use crate::shared::{ExecuteResult, HaltReason, OperandValueType};
use crate::tpu::TPU;

// Stack operations
pub fn op_push(tpu: &mut TPU, operand: &OperandValueType) -> ExecuteResult {
    // Stack overflow
    if tpu.tpu_state.stack.len() == TPU::STACK_SIZE {
        return ExecuteResult::Halt(HaltReason::StackOverflow);
    }

    // Get the value to push
    let value = tpu.get_operand_value(operand);

    // Push the value onto the stack
    tpu.push(value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Pop a value from the Stack and store it in a register
pub fn op_pop(tpu: &mut TPU, operand: &Register) -> ExecuteResult {
    // Pop a value from the stack
    let value = tpu.pop();

    // Store it in the register
    tpu.write_register(*operand, value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Peek at a value on the stack without removing it and store in the register
pub fn op_peek(tpu: &mut TPU, operand_1: &Register, operand_2: &OperandValueType) -> ExecuteResult {
    let index = tpu.get_operand_value(operand_2) as usize;

    if index > TPU::STACK_SIZE || index > tpu.tpu_state.stack.len() {
        // Out of bounds
        return ExecuteResult::Halt(HaltReason::IndexOutOfRange);
    }

    // Peek at value in the stack
    let value = tpu.tpu_state.stack[index];

    // Store it in the register
    tpu.write_register(*operand_1, value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Clear the stack
pub fn op_scr(tpu: &mut TPU) -> ExecuteResult {
    // Clear the stack
    tpu.tpu_state.stack.clear();

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Read Stack Pointer
pub fn op_rsp(tpu: &mut TPU, operand: &Register) -> ExecuteResult {
    tpu.write_register(*operand, tpu.stack_pointer());
    ExecuteResult::PCAdvance
}

// Memory operations
/// Copy the value from the source register to the destination register
pub fn op_rcy(tpu: &mut TPU, operand_1: &Register, operand_2: &Register) -> ExecuteResult {
    let value = tpu.read_register(*operand_2);
    tpu.write_register(*operand_1, value);

    ExecuteResult::PCAdvance
}

/// Move the value from the source register to the destination register
pub fn op_rmv(tpu: &mut TPU, operand_1: &Register, operand_2: &Register) -> ExecuteResult {
    let value = tpu.read_register(*operand_2);
    tpu.write_register(*operand_1, value);
    tpu.write_register(*operand_2, 0);

    ExecuteResult::PCAdvance
}

pub fn op_str(tpu: &mut TPU, target: &OperandValueType, source: Register) -> ExecuteResult {
    // Get the address and value
    let address = tpu.get_operand_value(target) as usize;
    let value = tpu.read_register(source);

    // Store the value in memory
    tpu.write_ram(address, value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Load a value into a register
pub fn op_ldr(tpu: &mut TPU, target: &Register, source: &OperandValueType) -> ExecuteResult {
    // Get the value
    let value = tpu.get_operand_value(source);

    // Store the value in the register
    tpu.write_register(*target, value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Load Register with Offset
pub fn op_ldo(
    tpu: &mut TPU,
    target: &Register,
    source: &OperandValueType,
    offset: &Register,
) -> ExecuteResult {
    // Get the address and offset
    let address = tpu.get_operand_value(source) as usize;
    let offset_amount = tpu.read_register(*offset) as usize;

    // Load the value from memory
    let value = tpu.read_ram(address + offset_amount);

    // Store the value in the register
    tpu.write_register(*target, value);

    ExecuteResult::PCAdvance
}

/// Load Register With Offset and Increment
pub fn op_ldoi(
    tpu: &mut TPU,
    target: &Register,
    source: &OperandValueType,
    offset: &Register,
) -> ExecuteResult {
    op_ldo(tpu, target, source, offset);
    tpu.write_register(*offset, tpu.read_register(*offset).wrapping_add(1));
    ExecuteResult::PCAdvance
}

/// Store To Memory
pub fn op_stm(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &OperandValueType,
) -> ExecuteResult {
    // Get the address and value
    let address = tpu.get_operand_value(target) as usize;
    let value = tpu.get_operand_value(source);

    // Store the value in memory
    tpu.write_ram(address, value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Store To Memory With Offset
pub fn op_stmo(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &OperandValueType,
    offset: &Register,
) -> ExecuteResult {
    // Get the address and value
    let address = tpu.get_operand_value(target) as usize;
    let value = tpu.get_operand_value(source);
    let offset_amount = tpu.read_register(*offset) as usize;

    // Store the value in memory
    tpu.write_ram(address + offset_amount, value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

/// Store To Memory With Offset and Increment
pub fn op_smoi(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &OperandValueType,
    offset: &Register,
) -> ExecuteResult {
    op_stmo(tpu, target, source, offset);
    tpu.write_register(*offset, tpu.read_register(*offset).wrapping_add(1));
    ExecuteResult::PCAdvance
}
