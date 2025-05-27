use crate::shared::DecodeResult;
use crate::shared::OperandValueType;
use crate::tpu::TPU;

pub fn decode_op_dpw(target: &OperandValueType, value: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[target, value]) + 4;

    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

// pub fn decode_op_dpwh() -> DecodeResult {
//     let cycles = match operands[0] {
//         Operand::Constant(_) => 6,
//         Operand::Register(_) => 7,
//     };
//
//     DecodeResult {
//         cycles,
//         pc_modified: false,
//     }
// }

pub fn decode_op_dpr(source: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[source]) + 2;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_apw(target: &OperandValueType, value: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[target, value]) + 4;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

// pub fn decode_op_apwh() -> DecodeResult {
//     let mut cycles = 6;
//
//     DecodeResult {
//         cycles,
//         pc_modified: false,
//     }
// }

pub fn decode_op_apr(source: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[source]) + 4;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_xmit() -> DecodeResult {
    DecodeResult {
        cycles: 10,
        call_every_cycle: false,
    }
}

pub fn decode_op_recv() -> DecodeResult {
    DecodeResult {
        cycles: 10,
        call_every_cycle: false,
    }
}

pub fn decode_op_txbs() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_rxbs() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}

pub fn decode_op_dpww(value: &OperandValueType) -> DecodeResult {
    let cycles = TPU::check_operand_cost(&[value]) + 4;
    DecodeResult {
        cycles,
        call_every_cycle: false,
    }
}

pub fn decode_op_dprw() -> DecodeResult {
    DecodeResult {
        cycles: 2,
        call_every_cycle: false,
    }
}
