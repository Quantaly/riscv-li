use riscv_li::{self, FLOAT_REGISTERS, INT_REGISTERS};
use std::io::{self, Write};
use std::process;
use std::str::FromStr;

#[paw::main]
fn main(mut args: paw::Args) -> io::Result<()> {
    let executable_name = args.next().unwrap_or(String::from("riscv-li"));
    if let (Some(register), Some(value)) = (args.next(), args.next()) {
        let register = register.as_str();
        match register {
            "help" | "-h" | "--help" => {
                print_help(&executable_name, io::stdout())?;
                Ok(())
            }
            _ if INT_REGISTERS.binary_search(&register).is_ok() => {
                if let Ok(value) = i32::from_str(&value) {
                    riscv_li::assemble_int_immediate(value, register, io::stdout())
                } else if let Ok(value) = u32::from_str(&value) {
                    riscv_li::assemble_int_immediate(value, register, io::stdout())
                } else {
                    eprintln!(
                        "Invalid value '{}' for integer register {}",
                        value, register
                    );
                    if f32::from_str(&value).is_ok() {
                        eprintln!("hint: use a floating-point register for floating-point values");
                    }
                    process::exit(exitcode::USAGE);
                }
            }
            _ if FLOAT_REGISTERS.binary_search(&register).is_ok() => {
                let temp_register = args.next().unwrap_or(String::from("t6"));
                let temp_register = temp_register.as_str();
                match temp_register {
                    "x0" | "zero" => {
                        eprintln!("The zero register cannot be used as a temp register");
                        eprintln!("hint: use a different integer register (defaults to t6 if unspecified)");
                        process::exit(exitcode::USAGE);
                    }
                    _ if INT_REGISTERS.binary_search(&temp_register).is_ok() => {
                        if let Ok(value) = f32::from_str(&value) {
                            riscv_li::assemble_float_immediate(
                                value,
                                register,
                                temp_register,
                                io::stdout(),
                            )
                        } else {
                            eprintln!(
                                "Invalid value '{}' for floating-point register {}",
                                value, register
                            );
                            process::exit(exitcode::USAGE);
                        }
                    }
                    _ => {
                        eprintln!("Invalid temp register {}", temp_register);
                        if FLOAT_REGISTERS.binary_search(&temp_register).is_ok() {
                            eprintln!("hint: use an integer register for the temporary");
                        }
                        process::exit(exitcode::USAGE);
                    }
                }
            }
            _ => {
                eprintln!("Invalid register '{}'", register);
                process::exit(exitcode::USAGE);
            }
        }
    } else {
        print_help(&executable_name, io::stderr())?;
        process::exit(exitcode::USAGE);
    }
}

fn print_help(executable_name: &str, mut out: impl Write) -> io::Result<()> {
    write!(
        out,
        "usage: {} <register> <value> [temp_register]\n",
        executable_name
    )?;
    write!(out, "temp_register defaults to t6\n")?;
    Ok(())
}
