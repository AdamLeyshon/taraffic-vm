pub mod decode;
#[cfg(test)]
mod io_matrix_test;

use crate::shared::{AnalogPin, DigitalPin, ExecuteResult, HaltReason, OperandValueType, Register};
use crate::tpu::TPU;

// Digital Pin operations
pub fn op_dpw(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &OperandValueType,
) -> ExecuteResult {
    let pin_num = tpu.get_operand_value(target);
    let value = tpu.get_operand_value(source);

    let pin = match DigitalPin::from_repr(pin_num) {
        Some(p) => p,
        None => return ExecuteResult::Halt(HaltReason::IndexOutOfRange),
    };

    // Set the pin value (any non-zero value is considered true)
    tpu.set_digital_pin(pin, value != 0);

    ExecuteResult::PCAdvance
}

// pub fn op_dpwh(tpu: &mut TPU, operands: &[Operand]) -> ExecuteResult {
//     // Get the pin number
//     let pin_num = tpu.get_operand_value(&operands[0]);
//
//     // Convert pin number to DigitalPin
//     let pin = match DigitalPin::from_repr(pin_num) {
//         Some(p) => p,
//         None => return true, // Return true to indicate an error
//     };
//
//     // Set the pin to HIGH
//     tpu.set_digital_pin(pin, true);
//
//     // Return ExecuteResult::Continue to indicate no error
//     ExecuteResult::Continue
// }

pub fn op_dpr(tpu: &mut TPU, target: &Register, source: &OperandValueType) -> ExecuteResult {
    // Get the pin number
    let pin_num = tpu.get_operand_value(source);

    let pin = match DigitalPin::from_repr(pin_num) {
        Some(p) => p,
        None => return ExecuteResult::Halt(HaltReason::IndexOutOfRange),
    };

    let value = tpu.get_digital_pin(pin);

    tpu.write_register(*target, if value { 1 } else { 0 });

    ExecuteResult::PCAdvance
}

// Analog Pin operations
pub fn op_apw(
    tpu: &mut TPU,
    target: &OperandValueType,
    source: &OperandValueType,
) -> ExecuteResult {
    let pin_num = tpu.get_operand_value(target);
    let value = tpu.get_operand_value(source);

    // Convert pin number to AnalogPin
    let pin = match AnalogPin::from_repr(pin_num) {
        Some(p) => p,
        None => return ExecuteResult::Halt(HaltReason::IndexOutOfRange),
    };

    tpu.set_analog_pin(pin, value);

    ExecuteResult::PCAdvance
}

// pub fn op_apwh(tpu: &mut TPU, operands: &[Operand]) -> ExecuteResult {
//     // Get the pin number and value
//     let pin_num = tpu.get_operand_value(&operands[0]);
//     let value = tpu.get_operand_value(&operands[1]);
//
//     // Convert pin number to AnalogPin
//     let pin = match AnalogPin::from_repr(pin_num) {
//         Some(p) => p,
//         None => return true, // Return true to indicate an error
//     };
//
//     // Set the pin value
//     tpu.set_analog_pin(pin, value);
//
//     // Return ExecuteResult::Continue to indicate no error
//     ExecuteResult::Continue
// }

pub fn op_apr(tpu: &mut TPU, target: &Register, source: &OperandValueType) -> ExecuteResult {
    let pin_num = tpu.get_operand_value(source);

    let value = match AnalogPin::from_repr(pin_num) {
        Some(p) => tpu.get_analog_pin(p),
        None => return ExecuteResult::Halt(HaltReason::IndexOutOfRange),
    };

    tpu.write_register(*target, value);

    ExecuteResult::PCAdvance
}

// Network operations
pub fn op_xmit(tpu: &mut TPU, target: &Register, data: &OperandValueType) -> ExecuteResult {
    // Get the address and data
    let target = tpu.read_register(*target);
    let data = tpu.get_operand_value(data);

    // Send the packet if there's room in the buffer
    if tpu.tpu_state.outgoing_packets.len() < TPU::NET_BUFFER_SIZE {
        tpu.send_packet(target, data);
    }
    // else
    // {
    // Set overflow CPU flag?
    // }

    ExecuteResult::PCAdvance
}

pub fn op_recv(tpu: &mut TPU) -> ExecuteResult {
    let packet = tpu.receive_packet();

    // Store the sender in X and the data in Y
    tpu.write_register(Register::X, packet.sender);
    tpu.write_register(Register::Y, packet.data);

    ExecuteResult::PCAdvance
}

/// Get the number of packets waiting to be sent
pub fn op_txbs(tpu: &mut TPU) -> ExecuteResult {
    let tx_buffer_size = tpu.tpu_state.outgoing_packets.len() as u16;

    tpu.write_register(Register::X, tx_buffer_size);

    ExecuteResult::PCAdvance
}

/// Get the number of packets waiting to be received
pub fn op_rxbs(tpu: &mut TPU) -> ExecuteResult {
    let rx_buffer_size = tpu.tpu_state.incoming_packets.len() as u16;

    tpu.write_register(Register::X, rx_buffer_size);

    ExecuteResult::PCAdvance
}

/// Digital Pin Write Word operation
pub fn op_dpww(tpu: &mut TPU, value: &OperandValueType) -> ExecuteResult {
    // Get the bitmask value
    let bitmask = tpu.get_operand_value(value);

    // Set the digital pins based on the bitmask
    tpu.set_digital_pins(bitmask);

    ExecuteResult::PCAdvance
}

/// Digital Pin Read Word operation
pub fn op_dprw(tpu: &mut TPU, target: &Register) -> ExecuteResult {
    // Read all digital pins as a 16-bit value
    let value = tpu.get_digital_pins();

    // Store it in the register
    tpu.write_register(*target, value);

    // Return ExecuteResult::Continue to indicate no error
    ExecuteResult::PCAdvance
}
