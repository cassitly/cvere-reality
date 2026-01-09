# CVERE - Computer Virtualization and Execution Reality Engine

A complete 16-bit hexadecimal instruction set architecture with VM, assembler, and visualization tools for educational purposes.

## Project Structure

```
cvere-reality/
├── desktop/
│   ├── rust/                      # High-performance VM
│   │   ├── src/
│   │   │   ├── lib.rs            # Library exports
│   │   │   ├── main.rs           # CLI entry point
│   │   │   ├── vm.rs             # VM implementation
│   │   │   ├── memory.rs         # Memory subsystem
│   │   │   ├── registers.rs      # Register file
│   │   │   └── decoder.rs        # Instruction decoder
│   │   └── Cargo.toml
│   │
│   └── python/                    # ISA tools
│       ├── assembler.py          # Assembly → Machine code
│       ├── disassembler.py       # Machine code → Assembly
│       └── isa_designer.py       # ISA specification tool
│
├── backend/
│   └── go/                        # API server
│       ├── main.go               # HTTP server
│       └── simulator/
│           ├── vm.go             # VM in Go
│           ├── assembler.go      # Assembler in Go
│           └── disassembler.go   # Disassembler in Go
│
├── docs/
│   └── Cvere ISA Specifications/
│       └── v1.0.md               # Complete ISA specification
│
└── examples/                      # Example programs
    ├── simple_add.hex
    ├── loop_counter.hex
    └── array_sum.hex
```

## Quick Start

### Building the Rust VM

```bash
cd desktop/rust
cargo build --release
cargo test
```

### Running Programs

```bash
# Run a program from hex file
./target/release/cvere run examples/simple_add.hex

# Run with execution tracing
./target/release/cvere trace examples/loop_counter.hex

# Start interactive REPL
./target/release/cvere repl

# Run tests
./target/release/cvere test
```

### Using Python Tools

```bash
cd desktop/python

# Assemble a program
python3 assembler.py < program.asm > program.hex

# Disassemble machine code
python3 disassembler.py < program.hex

# ISA design tool
python3 isa_designer.py
```

### Starting Go Backend

```bash
cd backend/go
go run main.go

# API will be available at http://localhost:8080
# Endpoints:
#   POST /api/assemble      - Assemble source code
#   POST /api/disassemble   - Disassemble machine code
#   POST /api/simulate      - Run simulation
#   GET  /api/health        - Health check
```

## ISA Overview

CVERE uses a 16-bit instruction format with hexadecimal encoding.

### Instruction Formats

**R-Type (Register operations)**
```
┌────┬────┬────┬────┐
│ Op │ Rd │ Rs │ Rt │
└────┴────┴────┴────┘
 4bit 4bit 4bit 4bit
```

**I-Type (Immediate operations)**
```
┌────┬────┬────────┐
│ Op │ Rd │  Imm8  │
└────┴────┴────────┘
 4bit 4bit   8bit
```

**M-Type (Memory operations)**
```
┌────┬────┬────┬────┐
│ Op │ Rd │ Rs │Off │
└────┴────┴────┴────┘
 4bit 4bit 4bit 4bit
```

### Instruction Set Summary

| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| 0x0    | NOP      | No operation |
| 0x1    | ADD      | Add registers |
| 0x2    | ADDI     | Add immediate |
| 0x3    | SUB      | Subtract |
| 0x4    | AND      | Bitwise AND |
| 0x5    | OR       | Bitwise OR |
| 0x6    | XOR      | Bitwise XOR |
| 0x7    | NOT      | Bitwise NOT |
| 0x8    | SHL      | Shift left |
| 0x9    | SHR      | Shift right |
| 0xA    | LOAD     | Load from memory |
| 0xB    | STORE    | Store to memory |
| 0xC    | LOADI    | Load immediate |
| 0xD    | JMP      | Jump |
| 0xE    | BEQ      | Branch if equal |
| 0xF    | BNE      | Branch if not equal |
| 0xFFFF | HALT     | Stop execution |

### Register File

- **R0-RF**: 16 general-purpose registers (R0 hardwired to 0)
- **PC**: Program Counter
- **SP**: Stack Pointer (starts at 0xFFFE)
- **LR**: Link Register (for function calls)
- **SR**: Status Register (flags: Z, N, C, V)

## Example Programs

### Simple Addition

```assembly
; Add two numbers
start:
    LOADI R1, 0x05      ; R1 = 5
    LOADI R2, 0x03      ; R2 = 3
    ADD   R3, R1, R2    ; R3 = R1 + R2
    HALT
```

**Machine Code:**
```hex
0xC105
0xC203
0x1312
0xFFFF
```

### Loop Counter

```assembly
; Count from 0 to 10
start:
    LOADI R1, 0x00      ; Counter = 0
    LOADI R2, 0x0A      ; Limit = 10
loop:
    ADDI  R1, 0x01      ; Counter++
    SUB   R3, R2, R1    ; R3 = Limit - Counter
    BNE   R3, loop      ; If R3 != 0, continue
    HALT
```

**Machine Code:**
```hex
0xC100
0xC20A
0x2101
0x3321
0xF3FD
0xFFFF
```

## Development

### Rust Module Structure

```rust
// lib.rs
pub mod memory;
pub mod registers;
pub mod decoder;
pub mod vm;

pub use vm::CVEREVM;
pub use memory::Memory;
pub use registers::{RegisterFile, StatusFlags};
pub use decoder::InstructionDecoder;
```

### Key APIs

**Rust VM**
```rust
let mut vm = CVEREVM::new();
vm.load_program(&program, 0)?;
vm.run(1000)?;  // Run for max 1000 cycles
```

**Go API**
```go
vm := simulator.NewVM()
vm.LoadProgram(program, 0)
vm.Step()  // Execute one instruction
```

**Python Assembler**
```python
assembler = CVEREAssembler()
machine_code = assembler.assemble(source)
hex_output = assembler.assemble_to_hex(source)
```

## Testing

### Rust Tests

```bash
cargo test                    # Run all tests
cargo test --verbose          # Verbose output
cargo test test_loop          # Run specific test
```

### Go Tests

```bash
cd backend/go
go test ./...
```

### Integration Tests

Create test programs in `examples/` and verify with:

```bash
./target/release/cvere run examples/test_program.hex
```

## API Reference

### Backend Endpoints

**POST /api/assemble**
```json
Request:
{
  "source": "LOADI R1, 0x05\nHALT"
}

Response:
{
  "machineCode": [50437, 65535],
  "labels": {}
}
```

**POST /api/simulate**
```json
Request:
{
  "machineCode": [50437, 65535],
  "maxCycles": 1000
}

Response:
{
  "history": [...],
  "finalState": {
    "registers": [...],
    "pc": 4,
    "halted": true
  },
  "cycleCount": 2
}
```

## Architecture Details

### Pipeline Stages

1. **IF** - Instruction Fetch
2. **ID** - Instruction Decode
3. **EX** - Execute
4. **MEM** - Memory Access
5. **WB** - Write Back

### Memory Layout

```
0x0000 - 0x00FF: Program code
0x0100 - 0xEFFF: General memory
0xF000 - 0xFFFD: Stack
0xFFFE - 0xFFFF: Stack pointer initial value
```

### Status Flags

- **Z (Zero)**: Set when result is 0
- **N (Negative)**: Set when bit 15 is 1
- **C (Carry)**: Set when arithmetic overflow occurs
- **V (Overflow)**: Set when signed overflow occurs

## Building for Production

```bash
# Rust VM
cd desktop/rust
cargo build --release
strip target/release/cvere

# Go Backend
cd backend/go
CGO_ENABLED=0 go build -ldflags="-s -w" -o cvere-server

# Python tools (no build needed)
```

## Performance Notes

- Rust VM: ~10M instructions/second
- Go VM: ~5M instructions/second
- Python tools: Best for development/prototyping

## Contributing

1. Follow the ISA specification in `docs/Cvere ISA Specifications/v1.0.md`
2. Add tests for new features
3. Update documentation
4. Ensure all tests pass before submitting

## License

Educational use only.

## Resources

- ISA Specification: `docs/Cvere ISA Specifications/v1.0.md`
- Example Programs: `examples/`
- API Documentation: See backend README
- Language Choices: `docs/Language Usage.md`

## Future Roadmap

- [ ] Extended instructions (CALL, RET, PUSH, POP)
- [ ] Interrupt handling
- [ ] Memory-mapped I/O
- [ ] Cache simulation
- [ ] Multi-core support
- [ ] Floating-point operations
- [ ] Visual debugger/profiler
- [ ] Hardware visualization frontend

---

**CVERE v1.0** - A hexadecimal architecture for education and exploration
