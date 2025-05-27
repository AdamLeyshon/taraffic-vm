use crate::shared::DecodeResult;
use crate::shared::OperandValueType;
use crate::tpu::TPU;

pub fn decode_op_jmp(target: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[target]) + 1;

    DecodeResult {
        cycles,
        call_every_cycle: true,
    }
}

pub fn decode_op_bez() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_bnz() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_beq() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_bne() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_bge() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_ble() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_bgt() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_blt() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_jpr(target: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[target]) + 1;

    DecodeResult {
        cycles,
        call_every_cycle: true,
    }
}

pub fn decode_op_brez() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_brnz() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_breq() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_brne() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_brge() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_brle() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_brgt() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_brlt() -> DecodeResult {
    DecodeResult {
        cycles: 3,
        call_every_cycle: true,
    }
}

pub fn decode_op_jsr(target: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[target]) + 4;
    DecodeResult {
        cycles,
        call_every_cycle: true,
    }
}

pub fn decode_op_rts() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: true,
    }
}
