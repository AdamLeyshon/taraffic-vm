use crate::shared::{DecodedOpcode, Operand};
use crate::tpu::TPU;

pub fn op_jmp(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the jump address and delay
    let address = tpu.get_operand_value(&operands[0]) as usize;

    // Check if the address is valid
    if address > (tpu.tpu_state.rom.len() - 1) {
        return true; // Return true to indicate an error
    }

    // Set the program counter
    tpu.tpu_state.program_counter = address;

    // Return false to indicate no error
    false
}

#[inline]
fn set_program_counter_conditionally(tpu: &mut TPU, condition: bool, address: usize) -> bool {
    // Check if the value is zero
    let target = if condition {
        address
    } else {
        tpu.tpu_state.program_counter + 1
    };

    // Check if the address is valid
    if target > (tpu.tpu_state.rom.len() - 1) {
        return true;
    }

    tpu.tpu_state.program_counter = target;
    false
}

pub fn decode_op_jmp(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Determine the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_jmp,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_bez(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and value
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let value = tpu.get_operand_value(&operands[1]);
    set_program_counter_conditionally(tpu, value == 0, address)
}

pub fn decode_op_bez(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_bez,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_bnz(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and value
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let value = tpu.get_operand_value(&operands[1]);

    set_program_counter_conditionally(tpu, value != 0, address)
}

pub fn decode_op_bnz(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_bnz,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_beq(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and values
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    set_program_counter_conditionally(tpu, a == b, address)
}

pub fn decode_op_beq(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_beq,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_bne(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and values
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    set_program_counter_conditionally(tpu, a != b, address)
}

pub fn decode_op_bne(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_bne,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_bge(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and values
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    set_program_counter_conditionally(tpu, a >= b, address)
}

pub fn decode_op_bge(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_bge,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_ble(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and values
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    set_program_counter_conditionally(tpu, a <= b, address)
}

pub fn decode_op_ble(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_ble,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_bgt(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and values
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    set_program_counter_conditionally(tpu, a > b, address)
}

pub fn decode_op_bgt(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_bgt,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_blt(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch address and values
    let address = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    set_program_counter_conditionally(tpu, a < b, address)
}

pub fn decode_op_blt(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_blt,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

// Relative Branches
pub fn op_jpr(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the jump offset
    let offset = tpu.get_operand_value(&operands[0]) as usize;

    // Calculate the new program counter
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, true, new_pc)
}

pub fn decode_op_jpr(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_jpr,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_brez(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(reg) => {
            // Get the branch offset and value
            let offset = tpu.get_operand_value(&operands[0]) as usize;
            let value = tpu.read_register(reg);
            // Calculate the new program counter
            let new_pc = tpu.tpu_state.program_counter + offset;
            set_program_counter_conditionally(tpu, value == 0, new_pc)
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_brez(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(_) => {
            // Calculate the number of clock cycles
            let mut cycles = 3; // Base cycles for register operand
            if let Operand::Register(_) = operands[0] {
                cycles += 1;
            }

            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_brez,
                cycles,
                pc_modified: true,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

pub fn op_brnz(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the second operand is a register
    match operands[1] {
        Operand::Register(reg) => {
            // Get the branch offset and value
            let offset = tpu.get_operand_value(&operands[0]) as usize;
            let value = tpu.read_register(reg);

            // Calculate the new program counter
            let new_pc = tpu.tpu_state.program_counter + offset;
            set_program_counter_conditionally(tpu, value != 0, new_pc)
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_brnz(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_brnz,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_breq(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch offset and values
    let offset = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    // Calculate the new program counter
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a == b, new_pc)
}

pub fn decode_op_breq(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_breq,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_brne(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch offset and values
    let offset = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    // Calculate the new program counter
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a != b, new_pc)
}

pub fn decode_op_brne(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_brne,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_brge(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch offset and values
    let offset = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    // Calculate the new program counter
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a >= b, new_pc)
}

pub fn decode_op_brge(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_brge,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_brle(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch offset and values
    let offset = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    // Calculate the new program counter
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a <= b, new_pc)
}

pub fn decode_op_brle(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_brle,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_brgt(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch offset and values
    let offset = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    // Calculate the new program counter
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a > b, new_pc)
}

pub fn decode_op_brgt(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_brgt,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_brlt(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the branch offset and values
    let offset = tpu.get_operand_value(&operands[0]) as usize;
    let a = tpu.get_operand_value(&operands[1]);
    let b = tpu.get_operand_value(&operands[2]);
    // Calculate the new program counter
    let new_pc = tpu.tpu_state.program_counter + offset;
    set_program_counter_conditionally(tpu, a < b, new_pc)
}

pub fn decode_op_brlt(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 3 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_brlt,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

// Subroutines
pub fn op_gsub(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the subroutine address
    let address = tpu.get_operand_value(&operands[0]);

    // Validate we have enough space on the stack
    if tpu.tpu_state.stack.len() == TPU::STACK_SIZE {
        return true;
    }

    let old_pc = tpu.tpu_state.program_counter;

    if set_program_counter_conditionally(tpu, true, address as usize) {
        return true;
    }

    // Only push the return address if we've validated the landing address
    // And modified the program counter
    tpu.push(old_pc as u16);

    false
}

pub fn decode_op_gsub(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(operands) + 1;

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_gsub,
        cycles,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_rsub(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Pop the return address from the stack
    let address = tpu.pop() as usize;
    set_program_counter_conditionally(tpu, true, address)
}

pub fn decode_op_rsub(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_rsub,
        cycles: 2,
        pc_modified: true,
    };

    // Return the decoded instruction
    Ok(inst)
}
