use crate::shared::{DecodedOpcode, Operand, Register};
use crate::tpu::TPU;

// Math operators

// Increment operations
pub fn op_inca(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the current value of the accumulator
    let a = tpu.read_register(Register::A);

    // Increment the value (wrapping on overflow)
    let result = a.wrapping_add(1);

    // Store the result back in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_inca(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_inca,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_incx(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the current value of X
    let x = tpu.read_register(Register::X);

    // Increment the value (wrapping on overflow)
    let result = x.wrapping_add(1);

    // Store the result back in X
    tpu.write_register(Register::X, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_incx(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_incx,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_incy(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the current value of Y
    let y = tpu.read_register(Register::Y);

    // Increment the value (wrapping on overflow)
    let result = y.wrapping_add(1);

    // Store the result back in Y
    tpu.write_register(Register::Y, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_incy(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_incy,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

// Decrement operations
pub fn op_deca(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the current value of the accumulator
    let a = tpu.read_register(Register::A);

    // Decrement the value (wrapping on underflow)
    let result = a.wrapping_sub(1);

    // Store the result back in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_deca(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_deca,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_decx(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the current value of X
    let x = tpu.read_register(Register::X);

    // Decrement the value (wrapping on underflow)
    let result = x.wrapping_sub(1);

    // Store the result back in X
    tpu.write_register(Register::X, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_decx(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_decx,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_decy(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the current value of Y
    let y = tpu.read_register(Register::Y);

    // Decrement the value (wrapping on underflow)
    let result = y.wrapping_sub(1);

    // Store the result back in Y
    tpu.write_register(Register::Y, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_decy(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_decy,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_add(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Add the operands (wrapping on overflow)
    let result = a.wrapping_add(b);

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_add(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_add,
        cycles,
        pc_modified: false,
    };

    // Return the number of clock cycles
    Ok(inst)
}

pub fn op_sub(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Subtract the operands (wrapping on underflow)
    let result = a.wrapping_sub(b);

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_sub(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_sub,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_mul(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Multiply the operands (wrapping on overflow)
    let result = a.wrapping_mul(b);

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_mul(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 3;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_mul,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_div(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Check for division by zero
    if b == 0 {
        return true; // Return true to indicate an error
    }

    // Divide the operands
    let quotient = a / b;
    let remainder = a % b;

    // Store the quotient in the accumulator and the remainder in X
    tpu.write_register(Register::A, quotient);
    tpu.write_register(Register::X, remainder);

    // Return false to indicate no error
    false
}

pub fn decode_op_div(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 4;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_div,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_mod(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Check for division by zero
    if b == 0 {
        return true; // Return true to indicate an error
    }

    // Calculate the modulo
    let result = a % b;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_mod(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 4;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_mod,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_and(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Perform bitwise AND
    let result = a & b;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_and(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_and,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_or(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Perform bitwise OR
    let result = a | b;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_or(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_or,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_xor(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operands
    let a = tpu.get_operand_value(&operands[0]);
    let b = tpu.get_operand_value(&operands[1]);

    // Perform bitwise XOR
    let result = a ^ b;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_xor(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_xor,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_not(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the operand
    let a = tpu.get_operand_value(&operands[0]);

    // Perform bitwise NOT
    let result = !a;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_not(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_not,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

// Bitshifting operations
pub fn op_shlr(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the value to shift and the shift amount
            let value = tpu.get_operand_value(&operands[1]);
            let shift = tpu.get_operand_value(&operands[2]);

            // Shift the value left
            let result = value << shift;

            // Store the result in the register
            tpu.write_register(reg, result);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_shlr(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_shlr,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_shlc(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the value to shift and the shift amount
            let value = tpu.get_operand_value(&operands[1]);
            let shift = tpu.get_operand_value(&operands[2]);

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
            tpu.write_register(reg, result);
            tpu.write_register(Register::A, carry);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_shlc(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_shlc,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_shla(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the value to shift and the shift amount
    let value = tpu.get_operand_value(&operands[0]);
    let shift = tpu.get_operand_value(&operands[1]);

    // Shift the value left
    let result = value << shift;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_shla(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_shla,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_shrr(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the value to shift and the shift amount
            let value = tpu.get_operand_value(&operands[1]);
            let shift = tpu.get_operand_value(&operands[2]);

            // Shift the value right
            let result = value >> shift;

            // Store the result in the register
            tpu.write_register(reg, result);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_shrr(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_shrr,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_shrc(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the value to shift and the shift amount
            let value = tpu.get_operand_value(&operands[1]);
            let shift = tpu.get_operand_value(&operands[2]);

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
            tpu.write_register(reg, result);
            tpu.write_register(Register::A, carry);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_shrc(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_shrc,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_shra(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the value to shift and the shift amount
    let value = tpu.get_operand_value(&operands[0]);
    let shift = tpu.get_operand_value(&operands[1]);

    // Shift the value right
    let result = value >> shift;

    // Store the result in the accumulator
    tpu.write_register(Register::A, result);

    // Return false to indicate no error
    false
}

pub fn decode_op_shra(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_shra,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

// Rotate operations
pub fn op_rol(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the value to rotate and the rotation amount
            let value = tpu.get_operand_value(&operands[1]);
            let rotate = tpu.get_operand_value(&operands[2]) % 16;

            // Rotate the value left
            let result = (value << rotate) | (value >> (16 - rotate));

            // Store the result in the register
            tpu.write_register(reg, result);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_rol(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_rol,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_ror(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the value to rotate and the rotation amount
            let value = tpu.get_operand_value(&operands[1]);
            let rotate = tpu.get_operand_value(&operands[2]) % 16;

            // Rotate the value right
            let result = (value >> rotate) | (value << (16 - rotate));

            // Store the result in the register
            tpu.write_register(reg, result);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_ror(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_ror,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}
