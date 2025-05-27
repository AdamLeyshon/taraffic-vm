use crate::shared::{AnalogPin, DigitalPin, ExecuteResult, HaltReason, NetPacket, OperandValueType, Register};
use crate::tpu::io_matrix::*;
use crate::tpu::{TPU, TpuState, create_basic_tpu_config};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tpu::ExecutionState;
    use std::collections::VecDeque;
    use strum::{EnumCount, IntoEnumIterator};

    // Helper function to create a TPU with specific register values
    fn create_tpu_with_registers(a: u16, x: u16, y: u16) -> TPU {
        let mut tpu_state = TpuState {
            stack: Vec::new(),
            analog_pins: [0; AnalogPin::COUNT],
            digital_pins: [false; DigitalPin::COUNT],
            analog_pin_config: [false; AnalogPin::COUNT],
            digital_pin_config: [false; DigitalPin::COUNT],

            ram: [0; TPU::RAM_SIZE],
            rom: vec![],
            network_address: 0x1,
            incoming_packets: std::collections::VecDeque::new(),
            outgoing_packets: std::collections::VecDeque::new(),
            registers: [0; Register::COUNT],

            program_counter: 0,
            halted: false,
            execution_state: ExecutionState::default(),
        };

        // Set register values
        tpu_state.registers[Register::A as usize] = a;
        tpu_state.registers[Register::X as usize] = x;
        tpu_state.registers[Register::Y as usize] = y;

        TPU::new_from_state(tpu_state)
    }

    // Helper function to create a TPU with specific digital pin values
    fn create_tpu_with_digital_pins(pin_values: &[(DigitalPin, bool)]) -> TPU {
        let mut tpu = create_tpu_with_registers(0, 0, 0);

        // Set digital pin values
        for (pin, value) in pin_values {
            tpu.set_digital_pin(*pin, *value);
        }

        tpu
    }

    // Helper function to create a TPU with specific analog pin values
    fn create_tpu_with_analog_pins(pin_values: &[(AnalogPin, u16)]) -> TPU {
        let mut tpu = create_tpu_with_registers(0, 0, 0);

        // Set analog pin values
        for (pin, value) in pin_values {
            tpu.set_analog_pin(*pin, *value);
        }

        tpu
    }

    // Helper function to create a TPU with specific network packets
    fn create_tpu_with_network_packets(incoming: &[NetPacket]) -> TPU {
        let mut tpu = create_tpu_with_registers(0, 0, 0);

        // Add incoming packets
        for packet in incoming {
            tpu.tpu_state.incoming_packets.push_back(packet.clone());
        }

        tpu
    }

    #[test]
    fn test_op_dpw() {
        // Test case 1: Set digital pin to HIGH
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = OperandValueType::Immediate(0); // Pin 0
        let source = OperandValueType::Immediate(1); // HIGH
        let result = op_dpw(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_digital_pin(DigitalPin::Digital0), true);
    
        // Test case 2: Set digital pin to LOW
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = OperandValueType::Immediate(1); // Pin 1
        let source = OperandValueType::Immediate(0); // LOW
        let result = op_dpw(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_digital_pin(DigitalPin::Digital1), false);
    
        // Test case 3: Set digital pin with register values
        let mut tpu = create_tpu_with_registers(0, 2, 1);
        let target = OperandValueType::Register(Register::X); // Pin 2 from X
        let source = OperandValueType::Register(Register::Y); // HIGH from Y
        let result = op_dpw(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_digital_pin(DigitalPin::Digital2), true);
    
        // Test case 4: Error case - invalid pin number
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = OperandValueType::Immediate(100); // Invalid pin
        let source = OperandValueType::Immediate(1);
        let result = op_dpw(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::IndexOutOfRange)); // Error
    }

    // test_op_dpwh removed as the opcode no longer exists

    #[test]
    fn test_op_dpr() {
        // Test case 1: Read digital pin (HIGH)
        let pin_values = [(DigitalPin::Digital0, true)];
        let mut tpu = create_tpu_with_digital_pins(&pin_values);
        let target = Register::A;
        let source = OperandValueType::Immediate(0); // Pin 0
        let result = op_dpr(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 1); // HIGH = 1
    
        // Test case 2: Read digital pin (LOW)
        let pin_values = [(DigitalPin::Digital1, false)];
        let mut tpu = create_tpu_with_digital_pins(&pin_values);
        let target = Register::A;
        let source = OperandValueType::Immediate(1); // Pin 1
        let result = op_dpr(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0); // LOW = 0
    
        // Test case 3: Read digital pin with register value
        let pin_values = [(DigitalPin::Digital2, true)];
        let mut tpu = create_tpu_with_digital_pins(&pin_values);
        tpu.write_register(Register::X, 2);
        let target = Register::A;
        let source = OperandValueType::Register(Register::X); // Pin 2 from X
        let result = op_dpr(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 1); // HIGH = 1
    
        // Test case 4: Error case - invalid pin number
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = Register::A;
        let source = OperandValueType::Immediate(100); // Invalid pin
        let result = op_dpr(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::IndexOutOfRange)); // Error
    }

    #[test]
    fn test_op_apw() {
        // Test case 1: Set analog pin to a value
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = OperandValueType::Immediate(0);  // Pin 0
        let source = OperandValueType::Immediate(42); // Value
        let result = op_apw(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_analog_pin(AnalogPin::Analog0), 42);
    
        // Test case 2: Set analog pin with register values
        let mut tpu = create_tpu_with_registers(0, 1, 255);
        let target = OperandValueType::Register(Register::X); // Pin 1 from X
        let source = OperandValueType::Register(Register::Y); // Value from Y
        let result = op_apw(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_analog_pin(AnalogPin::Analog1), 255);
    
        // Test case 3: Error case - invalid pin number
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = OperandValueType::Immediate(100); // Invalid pin
        let source = OperandValueType::Immediate(42);
        let result = op_apw(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::IndexOutOfRange)); // Error
    }

    // test_op_apwh removed as the opcode no longer exists

    #[test]
    fn test_op_apr() {
        // Test case 1: Read analog pin
        let pin_values = [(AnalogPin::Analog0, 42)];
        let mut tpu = create_tpu_with_analog_pins(&pin_values);
        let target = Register::A;
        let source = OperandValueType::Immediate(0); // Pin 0
        let result = op_apr(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 42);
    
        // Test case 2: Read analog pin with register value
        let pin_values = [(AnalogPin::Analog1, 255)];
        let mut tpu = create_tpu_with_analog_pins(&pin_values);
        tpu.write_register(Register::X, 1);
        let target = Register::A;
        let source = OperandValueType::Register(Register::X); // Pin 1 from X
        let result = op_apr(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 255);
    
        // Test case 3: Error case - invalid pin number
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = Register::A;
        let source = OperandValueType::Immediate(100); // Invalid pin
        let result = op_apr(&mut tpu, &target, &source);
        assert_eq!(result, ExecuteResult::Halt(HaltReason::IndexOutOfRange)); // Error
    }

    #[test]
    fn test_op_xmit() {
        // Test case 1: Send a packet
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let target = Register::A;
        tpu.write_register(Register::A, 0x2); // Target address
        let data = OperandValueType::Immediate(42);  // Data
        let result = op_xmit(&mut tpu, &target, &data);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.tpu_state.outgoing_packets.len(), 1);
        let packet = &tpu.tpu_state.outgoing_packets[0];
        assert_eq!(packet.sender, 0x1); // From our network address
        assert_eq!(packet.target, 0x2); // To the target address
        assert_eq!(packet.data, 42); // With the data
    
        // Test case 2: Send a packet with register values
        let mut tpu = create_tpu_with_registers(0, 0x3, 24);
        let target = Register::X; // Target address from X
        let data = OperandValueType::Register(Register::Y); // Data from Y
        let result = op_xmit(&mut tpu, &target, &data);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.tpu_state.outgoing_packets.len(), 1);
        let packet = &tpu.tpu_state.outgoing_packets[0];
        assert_eq!(packet.sender, 0x1); // From our network address
        assert_eq!(packet.target, 0x3); // To the target address
        assert_eq!(packet.data, 24); // With the data
    }

    #[test]
    fn test_op_recv() {
        // Test case 1: Receive a packet
        let incoming = [NetPacket {
            sender: 0x2,
            target: 0x1,
            data: 42,
        }];
        let mut tpu = create_tpu_with_network_packets(&incoming);
        let result = op_recv(&mut tpu);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 0x2); // Sender in X
        assert_eq!(tpu.read_register(Register::Y), 42); // Data in Y
        assert_eq!(tpu.tpu_state.incoming_packets.len(), 0); // Packet removed from queue
    
        // Test case 2: Receive when no packets are available
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let result = op_recv(&mut tpu);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 0); // Default sender
        assert_eq!(tpu.read_register(Register::Y), 0); // Default data
    }

    #[test]
    fn test_op_txbs() {
        // Test case 1: Get transmit buffer size (empty)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let result = op_txbs(&mut tpu);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 0); // Empty buffer
    
        // Test case 2: Get transmit buffer size (with packets)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        // Add some outgoing packets
        tpu.send_packet(0x2, 42);
        tpu.send_packet(0x3, 24);
        let result = op_txbs(&mut tpu);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 2); // Two packets in buffer
    }

    #[test]
    fn test_op_rxbs() {
        // Test case 1: Get receive buffer size (empty)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let result = op_rxbs(&mut tpu);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 0); // Empty buffer
    
        // Test case 2: Get receive buffer size (with packets)
        let incoming = [
            NetPacket {
                sender: 0x2,
                target: 0x1,
                data: 42,
            },
            NetPacket {
                sender: 0x3,
                target: 0x1,
                data: 24,
            },
        ];
        let mut tpu = create_tpu_with_network_packets(&incoming);
        let result = op_rxbs(&mut tpu);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::X), 2); // Two packets in buffer
    }

    #[test]
    fn test_with_basic_tpu_config() {
        // Test using create_basic_tpu_config
        let program = vec![
            // No instructions needed for this test
        ];
        let mut tpu = create_basic_tpu_config(program);

        // Test DPW operation
        let target = OperandValueType::Immediate(0); // Pin 0
        let source = OperandValueType::Immediate(1); // HIGH
        op_dpw(&mut tpu, &target, &source);
        // Expect it to be true because by default the pin is configured as an output
        // So you can't write to it
        assert_eq!(tpu.get_digital_pin(DigitalPin::Digital0), true);
    
        // Test APW operation
        let target = OperandValueType::Immediate(0);  // Pin 0
        let source = OperandValueType::Immediate(42); // Value
        op_apw(&mut tpu, &target, &source);
        assert_eq!(tpu.get_analog_pin(AnalogPin::Analog0), 42);
    
        // Test XMIT and TXBS operations
        tpu.write_register(Register::A, 0x2); // Target address
        let target = Register::A;
        let data = OperandValueType::Immediate(42);  // Data
        op_xmit(&mut tpu, &target, &data);
    
        op_txbs(&mut tpu);
        assert_eq!(tpu.read_register(Register::X), 1); // One packet in buffer
    }

    #[test]
    fn test_set_digital_pins_with_u16() {
        // Create a TPU with all digital pins configured as outputs
        let mut tpu = create_basic_tpu_config(vec![]);

        // Test case 1: Set all pins to 0
        tpu.set_digital_pins(0);
        for pin in DigitalPin::iter() {
            assert_eq!(tpu.get_digital_pin(pin), false);
        }

        // Test case 2: Set all pins to 1
        let all_pins_mask = (1 << DigitalPin::COUNT) - 1;
        tpu.set_digital_pins(all_pins_mask);
        for pin in DigitalPin::iter() {
            assert_eq!(tpu.get_digital_pin(pin), true);
        }

        // Test case 3: Set alternating pins
        let alternating_mask = 0b01010101 & all_pins_mask; // Only use valid pins
        tpu.set_digital_pins(alternating_mask);
        for pin in DigitalPin::iter() {
            let expected = (alternating_mask & (1 << (pin as u16))) != 0;
            println!(
                "Pin {:?}: {:5}, expect: {}",
                pin,
                tpu.get_digital_pin(pin),
                expected
            );
            assert_eq!(tpu.get_digital_pin(pin), expected);
        }
    }

    #[test]
    fn test_get_digital_pins_with_u16() {
        // Create a TPU with all digital pins configured as outputs
        let mut tpu = create_basic_tpu_config(vec![]);

        // Test case 1: All pins low
        for pin in DigitalPin::iter() {
            tpu.set_digital_pin(pin, false);
        }
        assert_eq!(tpu.get_digital_pins(), 0);

        // Test case 2: All pins high
        for pin in DigitalPin::iter() {
            tpu.set_digital_pin(pin, true);
        }
        let all_pins_mask = (1 << DigitalPin::COUNT) - 1;
        assert_eq!(tpu.get_digital_pins(), all_pins_mask);

        // Test case 3: Alternating pins
        let alternating_mask = 0b01010101 & all_pins_mask; // Only use valid pins
        for pin in DigitalPin::iter() {
            let value = (alternating_mask & (1 << (pin as u16))) != 0;
            tpu.set_digital_pin(pin, value);
        }
        assert_eq!(tpu.get_digital_pins(), alternating_mask);
    }

    #[test]
    fn test_op_dpww() {
        // Create a TPU with all digital pins configured as outputs
        let mut tpu = create_basic_tpu_config(vec![]);

        // Test case 1: Set all pins to 0
        let value = OperandValueType::Immediate(0);
        let result = op_dpww(&mut tpu, &value);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_digital_pins(), 0);
    
        // Test case 2: Set all pins to 1
        let all_pins_mask = (1 << DigitalPin::COUNT) - 1;
        let value = OperandValueType::Immediate(all_pins_mask);
        let result = op_dpww(&mut tpu, &value);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_digital_pins(), all_pins_mask);
    
        // Test case 3: Set alternating pins
        let alternating_mask = 0b01010101 & all_pins_mask; // Only use valid pins
        let value = OperandValueType::Immediate(alternating_mask);
        let result = op_dpww(&mut tpu, &value);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_digital_pins(), alternating_mask);
    
        // Test case 4: Set pins using register value
        let mut tpu = create_tpu_with_registers(0, alternating_mask, 0);
        let value = OperandValueType::Register(Register::X);
        let result = op_dpww(&mut tpu, &value);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.get_digital_pins(), alternating_mask);
    }

    #[test]
    fn test_op_dprw() {
        // Test case 1: Read all pins (all LOW)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        for pin in DigitalPin::iter() {
            tpu.set_digital_pin(pin, false);
        }
        let target = Register::A;
        let result = op_dprw(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), 0);
    
        // Test case 2: Read all pins (all HIGH)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        for pin in DigitalPin::iter() {
            tpu.set_digital_pin(pin, true);
        }
        let all_pins_mask = (1 << DigitalPin::COUNT) - 1;
        let target = Register::A;
        let result = op_dprw(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), all_pins_mask);
    
        // Test case 3: Read all pins (alternating)
        let mut tpu = create_tpu_with_registers(0, 0, 0);
        let all_pins_mask = (1 << DigitalPin::COUNT) - 1;
        let alternating_mask = 0b01010101 & all_pins_mask; // Only use valid pins
        for pin in DigitalPin::iter() {
            let value = (alternating_mask & (1 << (pin as u16))) != 0;
            tpu.set_digital_pin(pin, value);
        }
        let target = Register::A;
        let result = op_dprw(&mut tpu, &target);
        assert_eq!(result, ExecuteResult::PCAdvance); // No error
        assert_eq!(tpu.read_register(Register::A), alternating_mask);
    }
}
