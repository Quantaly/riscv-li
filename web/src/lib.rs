use riscv_li::{self, FLOAT_REGISTERS, INT_REGISTERS};
use wasm_bindgen::prelude::*;

fn js_error(e: impl ToString) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[wasm_bindgen]
pub fn float_registers() -> Vec<JsValue> {
    FLOAT_REGISTERS
        .iter()
        .map(|&s| JsValue::from_str(s))
        .collect()
}

#[wasm_bindgen]
pub fn int_registers() -> Vec<JsValue> {
    INT_REGISTERS
        .iter()
        .map(|&s| JsValue::from_str(s))
        .collect()
}

#[wasm_bindgen]
pub fn li_int(value: i32, reg: &str) -> Result<String, JsValue> {
    let mut out = Vec::new();
    riscv_li::assemble_int_immediate(value, reg, &mut out).map_err(js_error)?;
    String::from_utf8(out).map_err(js_error)
}

#[wasm_bindgen]
pub fn li_uint(value: u32, reg: &str) -> Result<String, JsValue> {
    let mut out = Vec::new();
    riscv_li::assemble_int_immediate(value, reg, &mut out).map_err(js_error)?;
    String::from_utf8(out).map_err(js_error)
}

#[wasm_bindgen]
pub fn li_float(value: f32, reg: &str, temp_reg: &str) -> Result<String, JsValue> {
    let mut out = Vec::new();
    riscv_li::assemble_float_immediate(value, reg, temp_reg, &mut out).map_err(js_error)?;
    String::from_utf8(out).map_err(js_error)
}
