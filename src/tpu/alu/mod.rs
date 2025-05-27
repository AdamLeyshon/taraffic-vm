pub mod decode;
#[cfg(test)]
mod alu_test;

use crate::shared::{ExecuteResult, HaltReason, OperandValueType, Register};
use crate::tpu::TPU;

// Math operators

pub fn op_inc(tpu: &mut TPU, target: &Register) -> ExecuteResult {
    tpu.write_register(*target, tpu.read_register(*target).wrapping_add(1));
    ExecuteResult::PCAdvance
}

pub fn op_dec(tpu: &mut TPU, target: &Register) -> ExecuteResult {
    tpu.write_register(*target, tpu.read_register(*target).wrapping_sub(1));
    ExecuteResult::PCAdvance
}

pub fn op_add(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    // Get the operands
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);

    // Add the operands (wrapping on overflow)
    let result = a.wrapping_add(b);

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

pub fn op_sub(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);
    let result = a.wrapping_sub(b);
    tpu.write_register(Register::A, result);
    ExecuteResult::PCAdvance
}

pub fn op_mul(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);
    let result = a.wrapping_sub(b);
    tpu.write_register(Register::A, result);
    ExecuteResult::PCAdvance
}

pub fn op_div(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);
    // Divide by zero error
    if b == 0 {
        return ExecuteResult::Halt(HaltReason::Div0);
    }
    let result = a.wrapping_div(b);
    tpu.write_register(Register::A, result);
    ExecuteResult::PCAdvance
}

pub fn op_mod(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);
    if b == 0 {
        return ExecuteResult::Halt(HaltReason::Div0);
    }
    let result = a % b;
    tpu.write_register(Register::A, result);
    ExecuteResult::PCAdvance
}

pub fn op_and(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);
    let result = a & b;
    tpu.write_register(Register::A, result);
    ExecuteResult::PCAdvance
}

pub fn op_or(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);
    let result = a | b;
    tpu.write_register(Register::A, result);
    ExecuteResult::PCAdvance
}

pub fn op_xor(tpu: &mut TPU, left: &Register, right: &Register) -> ExecuteResult {
    let a = tpu.read_register(*left);
    let b = tpu.read_register(*right);

    // Perform bitwise XOR
    let result = a ^ b;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

pub fn op_not(tpu: &mut TPU, left: &Register) -> ExecuteResult {
    // Get the operand
    let a = tpu.read_register(*left);

    // Perform bitwise NOT
    let result = !a;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}

// Bitshifting operations
pub fn op_sll(
    tpu: &mut TPU,
    target: &Register,
    value: &Register,
    shift: &OperandValueType,
) -> ExecuteResult {
    let value = tpu.read_register(*value);
    let shift = tpu.get_operand_value(shift);

    // Shift the value left
    let result = value << shift;

    tpu.write_register(*target, result);
    ExecuteResult::PCAdvance
}

pub fn op_slc(
    tpu: &mut TPU,
    target: &Register,
    value: &Register,
    shift: &OperandValueType,
) -> ExecuteResult {
    let value = tpu.read_register(*value);
    let shift = tpu.get_operand_value(shift);

    // Calculate the bits that will be shifted off
    let carry = if shift > 0 && shift < 16 {
        (value >> (16 - shift)) & ((1 << shift) - 1)
    } else if shift >= 16 {
        value
    } else {
        0
    };

    // Shift the value left
    let result = value << shift;

    // Store the result in the register and the carry in the accumulator
    tpu.write_register(*target, result);
    tpu.write_register(Register::A, carry);

    ExecuteResult::PCAdvance
}

pub fn op_slr(
    tpu: &mut TPU,
    target: &Register,
    value: &Register,
    shift: &OperandValueType,
) -> ExecuteResult {
    let value = tpu.read_register(*value);
    let shift = tpu.get_operand_value(shift);
    let result = value >> shift;
    tpu.write_register(*target, result);
    ExecuteResult::PCAdvance
}

pub fn op_src(
    tpu: &mut TPU,
    target: &Register,
    value: &Register,
    shift: &OperandValueType,
) -> ExecuteResult {
    let value = tpu.read_register(*value);
    let shift = tpu.get_operand_value(shift);

    // Calculate the bits that will be shifted off
    let carry = if shift > 0 && shift < 16 {
        (value & ((1 << shift) - 1)) << (16 - shift)
    } else if shift >= 16 {
        value
    } else {
        0
    };

    // Shift the value right
    let result = value >> shift;

    // Store the result in the register and the carry in the accumulator
    tpu.write_register(*target, result);
    tpu.write_register(Register::A, carry);
    ExecuteResult::PCAdvance
}

// Rotate operations
pub fn op_rol(
    tpu: &mut TPU,
    target: &Register,
    value: &Register,
    rotate: &OperandValueType,
) -> ExecuteResult {
    let value = tpu.read_register(*value);
    let rotate = tpu.get_operand_value(rotate) % 16;
    let result = (value << rotate) | (value >> (16 - rotate));
    tpu.write_register(*target, result);
    ExecuteResult::PCAdvance
}

pub fn op_ror(
    tpu: &mut TPU,
    target: &Register,
    value: &Register,
    rotate: &OperandValueType,
) -> ExecuteResult {
    let value = tpu.read_register(*value);
    let rotate = tpu.get_operand_value(rotate) % 16;
    let result = (value >> rotate) | (value << (16 - rotate));
    tpu.write_register(*target, result);
    ExecuteResult::PCAdvance
}
