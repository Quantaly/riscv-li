use super::assemble_immediate;
use std::fmt::Display;
use std::io::{self, Write};

/// Sorted alphabetically, to enable binary search
pub const INT_REGISTERS: &[&str] = &[
    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "fp", "gp", "ra", "s0", "s1", "s10", "s11",
    "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "sp", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "tp", "x0", "x1", "x10", "x11", "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x2",
    "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29", "x3", "x30", "x31", "x4",
    "x5", "x6", "x7", "x8", "x9", "zero",
];

#[test]
fn int_registers_sorted() {
    crate::check_slice_sorted("INT_REGISTERS", INT_REGISTERS);
}

mod private {
    pub trait Sealed {}
    impl Sealed for i32 {}
    impl Sealed for u32 {}
}

/// A signed or unsigned 32-bit integer.
///
/// This trait is sealed and only implemented for [i32] and [u32].
pub trait Integer32: private::Sealed + Display {
    fn into_u32(self) -> u32;
}

impl Integer32 for i32 {
    fn into_u32(self) -> u32 {
        u32::from_ne_bytes(self.to_ne_bytes())
    }
}

impl Integer32 for u32 {
    fn into_u32(self) -> u32 {
        self
    }
}

/// Generate instructions to load a 32-bit integer into `reg`.
///
/// `value` can be an [i32] or a [u32].
pub fn assemble_int_immediate(
    value: impl Integer32,
    reg: &str,
    mut out: impl Write,
) -> io::Result<()> {
    writeln!(
        &mut out,
        r"	# li {}, {}",
        reg, value
    )?;
    assemble_immediate(value.into_u32(), reg, &mut out)?;
    Ok(())
}
