use crate::shared::{DecodedOpcode, Operand, Register};
use crate::tpu::TPU;

// Stack operations
pub fn op_push(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Stack overflow
    if tpu.tpu_state.stack.len() == TPU::STACK_SIZE {
        return true;
    }

    // Get the value to push
    let value = tpu.get_operand_value(&operands[0]);

    // Push the value onto the stack
    tpu.push(value);

    // Return false to indicate no error
    false
}

pub fn decode_op_push(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_push,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_pop(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Pop a value from the stack
            let value = tpu.pop();

            // Store it in the register
            tpu.write_register(reg, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_pop(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Check if the operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_pop,
                cycles: 2,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_pushx(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Stack overflow
    if tpu.tpu_state.stack.len() == TPU::STACK_SIZE {
        return true;
    }

    // Push the value of X register onto the stack
    let x_value = tpu.read_register(Register::X);
    tpu.push(x_value);

    // Return false to indicate no error
    false
}

pub fn decode_op_pushx(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_pushx,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_popx(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Pop a value from the stack and store it in X
    let value = tpu.pop();
    tpu.write_register(Register::X, value);

    // Return false to indicate no error
    false
}

pub fn decode_op_popx(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_popx,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_peek(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            let index = tpu.get_operand_value(&operands[1]) as usize;

            if index > TPU::STACK_SIZE || index > tpu.tpu_state.stack.len() {
                // Out of bounds
                return true;
            }

            // Peek at value in the stack
            let value = tpu.tpu_state.stack[index];

            // Store it in the register
            tpu.write_register(reg, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_peek(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    // Check if the operand is a register
    match (operands[0], operands[1]) {
        (Operand::Register(_), Operand::Register(_) | Operand::Constant(_)) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_peek,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_scr(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Clear the stack
    tpu.tpu_state.stack.clear();

    // Return false to indicate no error
    false
}

pub fn decode_op_scr(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_scr,
        cycles: 2,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_rsp(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the stack pointer and store it in the register
            tpu.write_register(reg, tpu.stack_pointer());

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_rsp(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Check if the operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_rsp,
                cycles: 1,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

// Memory operations
pub fn op_rcy(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if both operands are registers
    match (&operands[0], &operands[1]) {
        (Operand::Register(dest), Operand::Register(src)) => {
            // Copy the value from the source register to the destination register
            let value = tpu.read_register(*src);
            tpu.write_register(*dest, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_rcy(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Check if both operands are registers
    match (&operands[0], &operands[1]) {
        (Operand::Register(_), Operand::Register(_)) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_rcy,
                cycles: 2,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_rmv(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if both operands are registers
    match (&operands[0], &operands[1]) {
        (Operand::Register(dest), Operand::Register(src)) => {
            // Move the value from the source register to the destination register
            let value = tpu.read_register(*src);
            tpu.write_register(*dest, value);
            tpu.write_register(*src, 0);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_rmv(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Check if both operands are registers
    match (&operands[0], &operands[1]) {
        (Operand::Register(_), Operand::Register(_)) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_rmv,
                cycles: 3,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_str(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(reg) => {
            // Get the address and value
            let address = tpu.get_operand_value(&operands[0]) as usize;
            let value = tpu.read_register(reg);

            // Store the value in memory
            tpu.write_ram(address, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_str(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_str,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_ldr(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the value
            let value = tpu.get_operand_value(&operands[1]);

            // Store the value in the register
            tpu.write_register(reg, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_ldr(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_ldr,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_ldm(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the address
            let address = tpu.get_operand_value(&operands[1]) as usize;

            // Load the value from memory
            let value = tpu.read_ram(address);

            // Store the value in the register
            tpu.write_register(reg, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_ldm(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_ldm,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_lda(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the value
    let value = tpu.get_operand_value(&operands[0]);

    // Store the value in the accumulator
    tpu.write_register(Register::A, value);

    // Return false to indicate no error
    false
}

pub fn decode_op_lda(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_lda,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_ldx(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the address and offset
            let address = tpu.get_operand_value(&operands[1]) as usize;
            let offset = tpu.read_register(Register::X) as usize;

            // Load the value from memory
            let value = tpu.read_ram(address + offset);

            // Store the value in the register
            tpu.write_register(reg, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_ldx(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_ldx,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_ldxi(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the address and offset
            let address = tpu.get_operand_value(&operands[1]) as usize;
            let offset = tpu.read_register(Register::X) as usize;

            // Load the value from memory
            let value = tpu.read_ram(address + offset);

            // Store the value in the register
            tpu.write_register(reg, value);

            // Increment X
            let x = tpu.read_register(Register::X);
            tpu.write_register(Register::X, x.wrapping_add(1));

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_ldxi(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 2;

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_ldxi,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_stm(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the address and value
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let value = tpu.get_operand_value(&operands[1]);

    // Store the value in memory
    tpu.write_ram(address, value);

    // Return false to indicate no error
    false
}

pub fn decode_op_stm(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_stm,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_sta(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the address
    let address = tpu.get_operand_value(&operands[0]) as usize;

    // Store the accumulator in memory
    let value = tpu.read_register(Register::A);
    tpu.write_ram(address, value);

    // Return false to indicate no error
    false
}

pub fn decode_op_sta(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_sta,
        cycles: 1,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_stx(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(reg) => {
            // Get the address and offset
            let address = tpu.get_operand_value(&operands[0]) as usize;
            let offset = tpu.read_register(Register::X) as usize;

            // Get the value from the register
            let value = tpu.read_register(reg);

            // Store the value in memory
            tpu.write_ram(address + offset, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_stx(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_stx,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_stxi(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(reg) => {
            // Get the address and offset
            let address = tpu.get_operand_value(&operands[0]) as usize;
            let offset = tpu.read_register(Register::X) as usize;

            // Get the value from the register
            let value = tpu.read_register(reg);

            // Store the value in memory
            tpu.write_ram(address + offset, value);

            // Increment X
            let x = tpu.read_register(Register::X);
            tpu.write_register(Register::X, x.wrapping_add(1));

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_stxi(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_stxi,
                cycles,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}
