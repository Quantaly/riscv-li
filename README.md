# RISC-V Load Immediate

Computer Organization at Colorado School of Mines uses the RISC-V architecture, but doesn't let us use most "pseudo-instructions," such as `li` for loading an arbitrary value into a register by composing the 20-bit `lui` and 12-bit `addi` instructions. So I made this to do the bit math for me.

Also, this can load floating-point immediates, which there isn't a pseudo-instruction for. The representation of the number is constructed in an integer register, then moved to a float register via the `fmv.s.x` instruction (the newer mnemonic, `fmv.w.x`, seems not to be well supported by [RARS](https://github.com/TheThirdOne/rars), which is what we are using).

I realized after making this that storing the value somewhere in the binary, then loading it by its address (we _are_ allowed to use `la`, thankfully), would be easier and wouldn't require an external calculator tool. However, using `la` and `lw`/`flw` is always three instructions (perhaps two, given a serendipitous birary layout and a smart preprocessor) and requires an additional word somewhere in the binary, while the output of this tool can be as small as a single instruction and is never more than three words.

## Command-line usage

```
usage: riscv-li <register> <value> [temp_register]
temp_register defaults to t6, is only used for loading floats
```

## Web app

https://quantaly.github.io/riscv-li
