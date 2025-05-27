# TSP, Traffic System Programming language

A language influenced by MIPS and 6502 assembly instructions.

TSP is a language defined using PEG grammar, with one instruction per line.

Designed specifically for programming the Traffic Processing Unit (TPU) virtual machine.

## What is the TPU

The TPU (Traffic Processing Unit) is a small but powerful Virtual Machine that can be programmed with a simple language.

The TPU was created specifically for a road traffic simulation engine, with goals to:

* Optimise traffic throughput
* Increase the safety of both the system and road users alike
* Minimise power consumption and manufacturing costs
* Be straightforward to program whilst remaining versatile

Since the TPU focuses on safety:

* Out-of-bounds errors cause a HLT
* Overflow and underflow math operations simply cause the value to wrap around from 0 to 65535 or 65535 back to 0.
* Divide by zero error causes a HLT.
* Sleep and I/O wait times are clamped to ensure infinite sleep does not occur.

When deployed in a live environment, if the CPU encounters a HLT, then a safe, fallback light sequence will be used
until an operator inspects and resets the system.

## TPU internal details

* All values are unsigned 16-bit integers
* The `SP` (Stack Pointer) and `PC` (Program Counter) are not exposed as registers but can be read with special
  instructions.
* Register `A` (the Accumulator) is a special register typically used for math operations but can be used for general
  purpose.
* `X` and `Y` are general purpose registers like `R0` to `R6` but are optimised for use with some instructions.
* The TPU has a stack which is FILO (First-In-Last-Out) and is 16 items in size.
    * Exceeding the stack size will cause the TPU to halt.
* A `HLT` instruction does not increase the PC, so you can see which line caused the error.

For instructions that expect booleans (Digital Pin instructions, for example), any non-zero value is considered true.

## Opcodes

As a general rule of thumb, the target of the operation is nearly always the first operand.

Using registers for an operand causes the instruction to consume one additional clock cycle delay.

If a Cycle Count is not provided for an instruction, it's because the value is not yet known since the TPU is currently
under development.

### Operand types

* `R`: Register operand can be any register (`R0`-`R6`, `X`, `Y` or `A`).
* `#`: Any value operand, the operand can be either a constant or a register.

## Opcode list

### Stack operations

PUSH or PEEK out-of-bounds causes a HLT instruction.

| Opcode | Operands | Name               | Description                                                                                            | Cycle Count |
|--------|----------|--------------------|--------------------------------------------------------------------------------------------------------|-------------|
| PUSH   | `#`      | Stack Push         | Pushes the value on to the stack and increases `SP`                                                    | 1-2         |
| POP    | `R`      | Stack Pop          | Pops a value off the stack into the register and decrements `SP`, if the stack is empty, returns `0`   | 1-2         |
| PUSHX  |          | Stack Push X       | Pushes the value in X on to the stack and increases `SP`                                               |             |
| POPX   |          | Stack Pop X        | Pops a value off the stack into the X register and decrements `SP`, if the stack is empty, returns `0` |             |
| PEEK   | `R`, `#` | Stack Peek         | Peek at a value on the stack without removing it and store in the register `R`                         |             |                        
| SCR    |          | Stack Clear        | Clears the stack and resets the stack pointer                                                          |             |                                                     
| RSP    | `R`      | Read Stack Pointer | Get the current stack pointer and store in register `R`                                                |             |                                               

### Flow Control

TPS programs lines are ZERO indexed. Line 0 is the first line of the program.

If the condition is met, the execution jumps to the line provided, otherwise execution continues to the next line.

For relative jumps/branches, the value of the operand is added to the current program counter.

For example:

```
0 ...
1 JPR 3 <- This relative jump will jump to line 4 because PC=1 and the operand is 3 (1+3 = 4).
2 ...
3 ...
4 JMP 0 <- This absolute jump will jump back to the start.
```

#### Absolute Branches

| Opcode | Operands      | Description                                                            | Cycle Count |
|--------|---------------|------------------------------------------------------------------------|-------------|
| JMP    | `#`           | Jump absolute                                                          | 1-2         |
| BEZ    | `#`, `R`      | Branch to operand 1 if register 2 is zero                               | 1-3         |
| BNZ    | `#`, `R`      | Branch to operand 1 if register 2 is not zero                           | 1-3         |         
| BEQ    | `#`, `R`, `#` | Branch to operand 1 if register 2 is equal to operand 3                 | 1-4         | 
| BNE    | `#`, `R`, `#` | Branch to operand 1 if register 2 is not equal to operand 3             | 1-4         |
| BGE    | `#`, `R`, `#` | Branch to operand 1 if register 2 is greater than or equal to operand 3 | 1-4         |
| BLE    | `#`, `R`, `#` | Branch to operand 1 if register 2 is less than or equal to operand 3    | 1-4         |
| BGT    | `#`, `R`, `#` | Branch to operand 1 if register is greater than operand 3              | 1-4         |
| BLT    | `#`, `R`, `#` | Branch to operand 1 if register 2 is less than operand 3                | 1-4         |

#### Relative Branches

| Opcode | Operands      | Description                                                             | Cycle Count |
|--------|---------------|-------------------------------------------------------------------------|-------------|
| JPR    | `#`           | Jump relative                                                           | 1-2         |
| BREZ   | `#`, `R`      | Branch relative by operand 1 if operand 2 is zero                       | 1-4         |
| BRNZ   | `#`, `R`      | Branch relative by operand 1 if operand 2 is not zero                   | 1-4         |
| BREQ   | `#`, `R`, `#` | Branch relative by operand 1 if operand 2 is equal to v                 | 1-4         |
| BRNE   | `#`, `R`, `#` | Branch relative by operand 1 if operand 2 is not equal to v             | 1-4         |
| BRGE   | `#`, `R`, `#` | Branch relative by operand 1 if operand 2 is greater than or equal to v | 1-4         |
| BRLE   | `#`, `R`, `#` | Branch relative by operand 1 if operand 2 is less than or equal to v    | 1-4         |
| BRGT   | `#`, `R`, `#` | Branch relative by operand 1 if operand 2 is greater than v             | 1-4         |
| BRLT   | `#`, `R`, `#` | Branch relative by operand 1 if operand 2 is less than v                | 1-4         |

#### Subroutines

Subroutines modify the stack, so pay close attention to stack usage.

The TPU will execute a `HLT` if it tries to jump to a non-existent line.

Trying to nest too many subroutines will cause a `HLT` due to a stack overflow.

With careful management and attention, you can pass parameters to subroutines by using registers or known RAM locations.

| Opcode | Operands | Description                                                                    | Cycle Count |
|--------|----------|--------------------------------------------------------------------------------|-------------|
| JSR    | `#`      | Pushes the current PC onto the stack and jumps absolute to the line specified. | 2           |
| RTS    |          | Pops the value off the stack and jumps absolute to the value.                  | 2           |

### Math operators

Any math operations that result in a value that cannot fit into a 16 bit word, the value to "wrap" around past zero.

e.g. if there is a value of 65,535 in a register, and you add 9, the result will be 8.

Unless otherwise specified, these instructions store their results in the accumulator (`A`).

| Opcode | Operands | Description                                                                   | Cycle Count |
|--------|----------|-------------------------------------------------------------------------------|-------------|
| ADD    | `R`, `R` | Adds the operands                                                             | 2           |
| SUB    | `R`, `R` | Subtracts operand 2 from operand 1                                            | 2           |
| MUL    | `R`, `R` | Multiplies the operands                                                       | 4           |
| DIV    | `R`, `R` | Divides operand 1 by operand 2, quotient goes to `A` and the remainder in `X` | 6           |
| MOD    | `R`, `R` | Modulo division of operand 1 by operand 2                                     | 6           |
| AND    | `R`, `R` | Performs a bitwise AND of the operands                                        | 3           |
| OR     | `R`, `R` | Performs a bitwise OR of the operands                                         | 3           |
| XOR    | `R`, `R` | Performs a bitwise XOR of the operands                                        | 3           |
| NOT    | `R`      | Performs a bitwise NOT of the operand                                         | 3           |           
| INC    | `R`      | Increments in the Register by 1 and stores the Result in `R`                  | 2           |           
| DEC    | `R`      | Decrements in the Register by 1 and stores the Result in `R`                  | 2           |

#### Bitshifting operations

When using operations that bitshift into the accumulator, the bits shifted off the ends of the operand are the bits
stored in the accumulator.

For example:

* r0 contains `01000001`
* r1 contains `00000010`
* `SHLC(r0, r0, r1)` will store `00000100` in r0 and `00000001` in the accumulator.

| Opcode | Operands      | Name                                         | Description                                                                                                | Cycle Count |
|--------|---------------|----------------------------------------------|------------------------------------------------------------------------------------------------------------|-------------|
| SLL    | `R`, `#`, `#` | Shift Left into Register                     | Shift the bits of operand 2 left by operand 3 places and store the result in operand 1                     |             |
| SLC    | `R`, `#`, `#` | Shift Left into Register, Accumulator Carry  | Shift the bits of operand 2 left by operand 3 places and store the result in operand 1, carry bits to `A`  |             |
| SLR    | `R`, `#`, `#` | Shift Right into Register                    | Shift the bits of operand 2 right by operand 3 places and store the result in operand 1                    |             |
| SRC    | `R`, `#`, `#` | Shift Right into Register, Accumulator Carry | Shift the bits of operand 2 right by operand 3 places and store the result in operand 1, carry bits to `A` |             |

#### Rotate operations

Rotating bits by more than seven places makes little sense since the register is only eight bits wide.

| Opcode | Operands      | Description                                                                              | Cycle Count |
|--------|---------------|------------------------------------------------------------------------------------------|-------------|
| ROL    | `R`, `#`, `#` | Rotate the bits of operand 2 left by operand 3 places and store the result in operand 1  |             |
| ROR    | `R`, `#`, `#` | Rotate the bits of operand 2 right by operand 3 places and store the result in operand 1 |             |

### Memory operations

| Opcode | Operands      | Name                                    | Description                                                                                           | Cycle Count |
|--------|---------------|-----------------------------------------|-------------------------------------------------------------------------------------------------------|-------------|
| RCY    | `R`, `R`      | Register Copy                           | Copy the value of operand 2 into operand 1,                                                           | 2           |
| RMV    | `R`, `R`      | Register Move                           | Move the value of operand 2 into operand 1, leaving the source register as zero                       | 3           |
| LDR    | `R`, `#`      | Load Register                           | Load value from operand into the register `R`                                                         |             |
| LDO    | `R`, `#`, `O` | Load Register with Offset               | Load value from address operand `#` plus offset `O` into register `R`                                 |             |
| LDOI   | `R`, `#`, `O` | Load Register With Offset and Increment | Load value from address operand `#` plus offset from register `O` into register `R` and increment `O` |             |
| STM    | `#`, `#`      | Store To Memory                         | Store value from operand 2 `#` into address operand 1                                                 |             |
| STMO   | `#`, `#`, `R` | Store To Memory With Offset             | Store value from operand 2 `#` into address operand 1                                                 |             |
| SMOI   | `#`, `#`, `R` | Store Memory With Offset and Increment  | Store value from operand 2 `#` into address operand 1 plus offset from register `R` and increment `R` |             |

Note 1: While `LDR` could be used for copying between registers, the microcode of `RCY` and `RMV` is optimised to
minimise the number of CPU cycles required.

### I/O Subsystem

#### Digital Pin operations

For the Read/Write word operations, The order is Least Significant Bit (LSB) first, i.e. Pin 0 is bit 0 of the word.

On implementations where there are fewer than 16 pins, the unused bit values will be zero.

| Opcode | Operands | Name                   | Description                                                                                | Cycle Count |
|--------|----------|------------------------|--------------------------------------------------------------------------------------------|-------------|
| DPW    | `#`, `#` | Digital Pin Write      | Sets the pin from operand 1 to the value of operand 2                                      | 1-3         |         
| DPWH   | `#`      | Digital Pin Write Hold | Sets the pin `#` to HIGH for 5 cycles, then back to LOW, equivalent to two DPW statements. | 6-7         |
| DPR    | `R`, `#` | Digital Pin Read       | Put the value of the pin from operand 1 into register `R`                                  | 2           |    
| DPWW   | `#`      | Digital Pin Write Word | Sets the output pin values based on the bitmask of the operand                             | 2           |
| DPRW   | `R`      | Digital Pin Read Word  | Read the value of all pins as a 16 bit value into Register R (Note 1)                      | 1           | 

Note 1: This also includes the current state of pins that are set to outputs.

#### Analog Pin operations

| Opcode | Operands | Name                  | Description                                                                              | Cycle Count |
|--------|----------|-----------------------|------------------------------------------------------------------------------------------|-------------|
| APW    | `#`, `#` | Analog Pin Write      | Sets the pin (operand 1) to the value of operand 2                                       |
| APWH   | `#`, `#` | Analog Pin Write Hold | Sets the pin (operand 1) to the value of operand 2 for 5 cycles before returning to zero | 6-8         |
| APR    | `R`, `#` | Analog Pin Read       | Put the value of pin `#` into register `R`                                               |

#### Network operations

When connected to the network, the TPU will only receive traffic that addresses it directly, or was broadcast on the
special address of 65,535 (0xFFFF).

The network layer guarantees that so long as the other device:

* Is powered on
* Not halted
* has space in its buffer

that the message will be received.

| Opcode | Operands | Name                 | Description                                                                                           | Cycle Count |
|--------|----------|----------------------|-------------------------------------------------------------------------------------------------------|-------------|
| XMIT   | `#`, `#` | Transmit             | Send operand 2 to a network device with address from operand 1 (Note 1)                               | 4           |
| RECV   |          | Receive              | Get a packet from the network, store the sender in register `X` and the data in register `Y` (Note 2) | 4           |
| TXBS   |          | Transmit Buffer Size | Get the number of network packets waiting to be sent and store in register `X`                        | 2           |
| RXBS   |          | Receive Buffer Size  | Get the number of network packets waiting to be received and store in register `X`                    | 2           |

Note 1: If the output buffer is full, the packet is dropped
Note 2: Both will be `0` if no packets are waiting.

### Misc operations

| Opcode | Operands | Name          | Description                                                           | Cycle Count |
|--------|----------|---------------|-----------------------------------------------------------------------|-------------|
| NOP    |          | No Operation  | Waits for exactly 2 cycles                                            | 2           |               
| SLP    | `#`      | Sleep         | Sleep for the specified number of cycles, Equivalent to multiple NOPs | 2+          | 
| WRX    |          | Wait Receive  | Wait for a packet to be received                                      | 1+          |                                                                               
| HLT    |          | Halt          | Stops the TPU, non-recoverable.                                       | 1           |                                                                                   
