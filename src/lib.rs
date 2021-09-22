use std::io::{self, Write};

mod float;
mod int;

pub use float::{assemble_float_immediate, FLOAT_REGISTERS};
pub use int::{assemble_int_immediate, INT_REGISTERS};

fn assemble_immediate(value: u32, reg: &str, out: &mut impl Write) -> io::Result<()> {
    const TWENTY_BITS: u32 = 0b1111_1111_1111_1111_1111;
    const ELEVEN_BITS: u32 = 0b0111_1111_1111;
    const BIT_ELEVEN: u32 = 0b1000_0000_0000;

    let upper_bits = value >> 12;
    let lower_bits = value & ELEVEN_BITS;
    let bit_eleven = value & BIT_ELEVEN > 0;

    if bit_eleven {
        let upper_imm = (upper_bits + 1) & TWENTY_BITS;
        let lower_imm = ((lower_bits ^ ELEVEN_BITS) + 1) & ELEVEN_BITS;
        match (upper_imm > 0, lower_imm > 0) {
            (true, true) => writeln!(
                out,
                r"	lui {reg}, {upper_imm}
	addi {reg}, {reg}, -{lower_imm}",
                reg = reg,
                upper_imm = upper_imm,
                lower_imm = lower_imm,
            ),
            (true, false) => writeln!(
                out,
                r"	lui {reg}, {upper_imm}
	addi {reg}, {reg}, -2048",
                reg = reg,
                upper_imm = upper_imm,
            ),
            (false, true) => writeln!(
                out,
                r"	addi {reg}, zero, -{lower_imm}",
                reg = reg,
                lower_imm = lower_imm,
            ),
            (false, false) => writeln!(
                out,
                r"	addi {reg}, zero, -2048",
                reg = reg,
            ),
        }
    } else {
        match (upper_bits > 0, lower_bits > 0) {
            (true, true) => writeln!(
                out,
                r"	lui {reg}, {upper_bits}
	addi {reg}, {reg}, {lower_bits}",
                reg = reg,
                upper_bits = upper_bits,
                lower_bits = lower_bits,
            ),
            (true, false) => writeln!(
                out,
                r"	lui {reg}, {upper_bits}",
                reg = reg,
                upper_bits = upper_bits,
            ),
            (false, true) => writeln!(
                out,
                r"	addi {reg}, zero, {lower_bits}",
                reg = reg,
                lower_bits = lower_bits,
            ),
            (false, false) => writeln!(
                out,
                r"	add {reg}, zero, zero",
                reg = reg,
            ),
        }
    }
}

#[cfg(test)]
fn check_slice_sorted<T: Ord + std::fmt::Debug>(name: &str, slice: &[T]) {
    for window in slice.windows(2) {
        if window[0] > window[1] {
            let mut sorted: Vec<&T> = slice.iter().collect();
            sorted.sort_unstable();
            panic!("{} is not sorted, should be {:?}", name, sorted);
        }
    }
}
