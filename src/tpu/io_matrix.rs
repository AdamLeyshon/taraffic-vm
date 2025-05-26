use crate::shared::{AnalogPin, DecodedOpcode, DigitalPin, Operand, Register};
use crate::tpu::TPU;

// Digital Pin operations
pub fn op_dpw(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the pin number and value
    let pin_num = tpu.get_operand_value(&operands[0]);
    let value = tpu.get_operand_value(&operands[1]);

    // Convert pin number to DigitalPin
    let pin = match DigitalPin::from_repr(pin_num) {
        Some(p) => p,
        None => return true, // Return true to indicate an error
    };

    // Set the pin value (any non-zero value is considered true)
    tpu.set_digital_pin(pin, value != 0);

    // Return false to indicate no error
    false
}

pub fn decode_op_dpw(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    // 1 cycle for constant, +1 for each register operand
    let mut cycles = 1;
    if let Operand::Register(_) = operands[0] {
        cycles += 1;
    }
    if let Operand::Register(_) = operands[1] {
        cycles += 1;
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_dpw,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_dpwh(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the pin number
    let pin_num = tpu.get_operand_value(&operands[0]);

    // Convert pin number to DigitalPin
    let pin = match DigitalPin::from_repr(pin_num) {
        Some(p) => p,
        None => return true, // Return true to indicate an error
    };

    // Set the pin to HIGH
    tpu.set_digital_pin(pin, true);

    // Return false to indicate no error
    false
}

pub fn decode_op_dpwh(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Calculate the number of clock cycles (5 cycles + 1 if register operand)
    let cycles = match operands[0] {
        Operand::Constant(_) => 6,
        Operand::Register(_) => 7,
    };

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_dpwh,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_dpr(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the pin number
            let pin_num = tpu.get_operand_value(&operands[1]);

            // Convert pin number to DigitalPin
            let pin = match DigitalPin::from_repr(pin_num) {
                Some(p) => p,
                None => return true, // Return true to indicate an error
            };

            // Read the pin value
            let value = tpu.get_digital_pin(pin);

            // Store it in the register
            tpu.write_register(reg, if value { 1 } else { 0 });

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_dpr(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_dpr,
                cycles: 2,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

// Analog Pin operations
pub fn op_apw(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the pin number and value
    let pin_num = tpu.get_operand_value(&operands[0]);
    let value = tpu.get_operand_value(&operands[1]);

    // Convert pin number to AnalogPin
    let pin = match AnalogPin::from_repr(pin_num) {
        Some(p) => p,
        None => return true, // Return true to indicate an error
    };

    // Set the pin value
    tpu.set_analog_pin(pin, value);

    // Return false to indicate no error
    false
}

pub fn decode_op_apw(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles
    // 1 cycle for constant, +1 for each register operand
    let mut cycles = 1;
    if let Operand::Register(_) = operands[0] {
        cycles += 1;
    }
    if let Operand::Register(_) = operands[1] {
        cycles += 1;
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_apw,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_apwh(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the pin number and value
    let pin_num = tpu.get_operand_value(&operands[0]);
    let value = tpu.get_operand_value(&operands[1]);

    // Convert pin number to AnalogPin
    let pin = match AnalogPin::from_repr(pin_num) {
        Some(p) => p,
        None => return true, // Return true to indicate an error
    };

    // Set the pin value
    tpu.set_analog_pin(pin, value);

    // Return false to indicate no error
    false
}

pub fn decode_op_apwh(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Calculate the number of clock cycles (6 cycles + 1 for each register operand)
    let mut cycles = 6;
    if let Operand::Register(_) = operands[0] {
        cycles += 1;
    }
    if let Operand::Register(_) = operands[1] {
        cycles += 1;
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_apwh,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_apr(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Get the pin number
            let pin_num = tpu.get_operand_value(&operands[1]);

            // Convert pin number to AnalogPin
            let pin = match AnalogPin::from_repr(pin_num) {
                Some(p) => p,
                None => return true, // Return true to indicate an error
            };

            // Read the pin value
            let value = tpu.get_analog_pin(pin);

            // Store it in the register
            tpu.write_register(reg, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_apr(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_apr,
                cycles: 2,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}

// Network operations
pub fn op_xmit(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the address and data
    let address = tpu.get_operand_value(&operands[0]);
    let data = tpu.get_operand_value(&operands[1]);

    // Send the packet if there's room in the buffer
    if tpu.tpu_state.outgoing_packets.len() < TPU::NET_BUFFER_SIZE {
        tpu.send_packet(address, data);
    }

    // Return false to indicate no error
    false
}

pub fn decode_op_xmit(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 2 {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_xmit,
        cycles: 4,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_recv(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Receive a packet
    let packet = tpu.receive_packet();

    // Store the sender in X and the data in Y
    tpu.write_register(Register::X, packet.sender);
    tpu.write_register(Register::Y, packet.data);

    // Return false to indicate no error
    false
}

pub fn decode_op_recv(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_recv,
        cycles: 4,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_txbs(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the number of packets waiting to be sent
    let tx_buffer_size = tpu.tpu_state.outgoing_packets.len() as u16;

    // Store it in X
    tpu.write_register(Register::X, tx_buffer_size);

    // Return false to indicate no error
    false
}

pub fn decode_op_txbs(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_txbs,
        cycles: 2,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

pub fn op_rxbs(tpu: &mut TPU, _: &[Operand]) -> bool {
    // Get the number of packets waiting to be received
    let rx_buffer_size = tpu.tpu_state.incoming_packets.len() as u16;

    // Store it in X
    tpu.write_register(Register::X, rx_buffer_size);

    // Return false to indicate no error
    false
}

pub fn decode_op_rxbs(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if !operands.is_empty() {
        return Err(());
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_rxbs,
        cycles: 2,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

// Digital Pin Write Word operation
pub fn op_dpww(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Get the bitmask value
    let bitmask = tpu.get_operand_value(&operands[0]);

    // Set the digital pins based on the bitmask
    tpu.set_digital_pins(bitmask);

    // Return false to indicate no error
    false
}

pub fn decode_op_dpww(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Calculate the number of clock cycles
    // 1 cycle for constant, +1 for register operand
    let mut cycles = 1;
    if let Operand::Register(_) = operands[0] {
        cycles += 1;
    }

    let inst = DecodedOpcode {
        operands: operands.to_vec(),
        function: op_dpww,
        cycles,
        pc_modified: false,
    };

    // Return the decoded instruction
    Ok(inst)
}

// Digital Pin Read Word operation
pub fn op_dprw(tpu: &mut TPU, operands: &[Operand]) -> bool {
    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(reg) => {
            // Read all digital pins as a 16-bit value
            let value = tpu.get_digital_pins();

            // Store it in the register
            tpu.write_register(reg, value);

            // Return false to indicate no error
            false
        }
        _ => true, // Return true to indicate an error
    }
}

pub fn decode_op_dprw(operands: &[Operand]) -> Result<DecodedOpcode, ()> {
    // Check operand count
    if operands.len() != 1 {
        return Err(());
    }

    // Check if the first operand is a register
    match operands[0] {
        Operand::Register(_) => {
            let inst = DecodedOpcode {
                operands: operands.to_vec(),
                function: op_dprw,
                cycles: 1,
                pc_modified: false,
            };

            // Return the decoded instruction
            Ok(inst)
        }
        _ => Err(()),
    }
}
