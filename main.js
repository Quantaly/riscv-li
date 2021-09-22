import init, { int_registers, float_registers, li_int, li_uint, li_float } from "./pkg/riscv_li_wasm.js";

const MAX_U32 = Math.pow(2, 32) - 1;
const MIN_I32 = -Math.pow(2, 31)

init().then(() => {
    const regSelect = document.querySelector("#reg-select");
    const valueInput = document.querySelector("#value-input");
    const tempRegSelect = document.querySelector("#temp-reg-select");
    const output = document.querySelector("#output");
    const copyButton = document.querySelector("#copy-button");

    const intRegisters = int_registers();
    const floatRegisters = float_registers();

    function update() {
        try {
            const reg = regSelect.value;
            const value = valueInput.valueAsNumber;
            if (intRegisters.includes(reg)) {
                tempRegSelect.disabled = true;
                if (0 <= value && value <= MAX_U32) {
                    output.value = li_uint(value, reg);
                } else if (MIN_I32 <= value && value < 0) {
                    output.value = li_int(value, reg);
                } else {
                    output.value = "ERROR: Integer value out of range!";
                }
            } else if (floatRegisters.includes(reg)) {
                tempRegSelect.disabled = false;
                output.value = li_float(value, reg, tempRegSelect.value);
            } else {
                output.value = "ERROR: Invalid register. This is probably a bug.";
            }
        } catch (e) {
            output.value = "ERROR: Check the browser console.";
            console.error(e);
        }
    }

    regSelect.addEventListener("input", update);
    valueInput.addEventListener("input", update);
    tempRegSelect.addEventListener("input", update);

    update();

    copyButton.addEventListener("click", () => {
        navigator.clipboard.writeText(output.value);
        copyButton.disabled = true;
        copyButton.textContent = "Copied!";
        setTimeout(() => {
            copyButton.disabled = false;
            copyButton.textContent = "Copy";
        }, 750);
    });
});