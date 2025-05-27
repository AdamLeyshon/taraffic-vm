use crate::shared::{OperandValueType, DecodeResult};
use crate::tpu::{TPU};

// Increment operations
pub fn decode_op_inc() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_dec() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_add() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_sub() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_mul() -> DecodeResult {
    DecodeResult {
        cycles: 4,
        call_every_cycle: false,
    }
}

pub fn decode_op_div() -> DecodeResult {
    DecodeResult {
        cycles: 6,
        call_every_cycle: false,
    }
}

pub fn decode_op_mod() -> DecodeResult {
    DecodeResult {
        cycles: 6,
        call_every_cycle: false,
    }
}

pub fn decode_op_and() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: false,
    }
}

pub fn decode_op_or() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: false,
    }
}

pub fn decode_op_xor() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: false,
    }
}

pub fn decode_op_not() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_sll(shift: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[shift]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_slc(shift: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[shift]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_shla(shift: &OperandValueType) -> DecodeResult {
    // Calculate the number of clock cycles
    let cycles = TPU::check_operand_cost(&[shift]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_slr(shift: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[shift]) + 2;

    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_src(shift: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[shift]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_shra(shift: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[shift]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_rol(shift: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[shift]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_ror(shift: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[shift]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}
