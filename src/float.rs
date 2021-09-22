use super::assemble_immediate;
use std::io::{self, Write};

/// Sorted alphabetically, to enable binary search
pub const FLOAT_REGISTERS: &[&str] = &[
    "f0", "f1", "f10", "f11", "f12", "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f2", "f20",
    "f21", "f22", "f23", "f24", "f25", "f26", "f27", "f28", "f29", "f3", "f30", "f31", "f4", "f5",
    "f6", "f7", "f8", "f9", "fa0", "fa1", "fa2", "fa3", "fa4", "fa5", "fa6", "fa7", "fs0", "fs1",
    "fs10", "fs11", "fs2", "fs3", "fs4", "fs5", "fs6", "fs7", "fs8", "fs9", "ft0", "ft1", "ft10",
    "ft11", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "ft8", "ft9",
];

#[test]
fn float_registers_sorted() {
    crate::check_slice_sorted("FLOAT_REGISTERS", FLOAT_REGISTERS);
}

/// Generate instructions to load a single-precision floating-point number into `reg`.
///
/// `temp_reg` should be an integer register and may be clobbered by the operation.
pub fn assemble_float_immediate(
    value: f32,
    reg: &str,
    temp_reg: &str,
    mut out: impl Write,
) -> io::Result<()> {
    if value == 0.0 {
        // 0.0 is represented by all zeroes, so we can pull directly from the zero register
        writeln!(
            &mut out,
            r"	# li {reg}, 0
	fmv.s.x {reg}, zero",
            reg = reg
        )?;
    } else {
        writeln!(
            &mut out,
            r"	# li {}, {} (uses {})",
            reg, value, temp_reg
        )?;
        assemble_immediate(u32::from_ne_bytes(value.to_ne_bytes()), temp_reg, &mut out)?;
        write!(
            &mut out,
            r"	fmv.s.x {}, {}",
            reg, temp_reg
        )?;
    }
    Ok(())
}
