use crate::shared::{DecodeResult, OperandValueType, Register};
use crate::tpu::TPU;

pub fn decode_op_push(operand: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[operand]) + 1;

    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_pop() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_peek(index: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[index]) + 1;

    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_scr() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_rsp() -> DecodeResult {
    DecodeResult {
        cycles: 1,
        call_every_cycle: false,
    }
}

pub fn decode_op_rcy() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_rmv() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: false,
    }
}

pub fn decode_op_str(_: &Register, source: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[source]) + 1;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_ldr(_: &Register, source: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[source]) + 1;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_ldo(source: &OperandValueType) -> DecodeResult {
    // Two cycles needed minimum
    // * One to perform the Addition
    // * One to write the value into the target register
    // 1 cycle penalty if the source is a memory address, not another register
    let cycles = TPU::check_operand_cost(&[source]) + 2;

    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_ldoi(source: &OperandValueType) -> DecodeResult {
    // Three cycles needed minimum
    // * One to perform the Addition
    // * One to write the value into the target register
    // * One to increment X
    // 1 cycle penalty if the source is a memory address, not another register
    let cycles = TPU::check_operand_cost(&[source]) + 3;

    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_stm(source: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[source]) + 1;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_stmo(source: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[source]) + 4;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_smoi(source: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[source]) + 5;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}
