# X+ Compiler Architecture

## Overview

The X+ compiler pipeline follows a traditional architecture optimized for:
- Fast incremental compilation
- Parallel module compilation
- Efficient caching
- Self-hosting capability

## Pipeline

```
Source Code (.xp)
    ↓
Lexer (Tokenization)
    ↓
Parser (Syntax Analysis)
    ↓
AST (Abstract Syntax Tree)
    ↓
Name Resolution
    ↓
Type Checking
    ↓
IR (Intermediate Representation)
    ↓
Optimization
    ↓
Code Generation
    ↓
Native Machine Code
```

## Compilation Phases

### Phase 1: Lexer
**Status**: ✓ Complete

- Tokenizes source into `Token` stream
- Handles comments (line `//` and block `/* */`)
- Recognizes all XP keywords and operators
- Tracks source positions for error reporting

**Key Files**:
- `src/lexer/mod.rs` - Lexer implementation
- `src/lexer/token.rs` - Token types and definitions
- `src/lexer/position.rs` - Source position tracking

### Phase 2: Parser
**Status**: ✓ Complete

- Recursive descent parser
- Builds Abstract Syntax Tree (AST)
- Supports:
  - Functions, structs, classes, enums, interfaces
  - Expressions with proper precedence
  - Statements and blocks
  - Type annotations
  - One-line and multi-line code

**Key Files**:
- `src/parser/mod.rs` - Parser implementation

### Phase 3: AST
**Status**: ✓ Complete

- Complete AST node definitions
- Formatted output for debugging
- Support for all language constructs

**Key Files**:
- `src/ast/mod.rs` - AST definitions

### Phase 4: Evaluation/Runtime
**Status**: ✓ Basic implementation

- Tree-walking interpreter
- Runtime value representation
- Built-in functions (print)
- Variable scoping
- Function calls
- Binary and unary operations

**Future Optimization**:
- Bytecode VM
- JIT compilation
- Native code generation

**Key Files**:
- `src/compiler.rs` - Runtime compiler and evaluator

### Phase 5: Type Checking
**Status**: Planned

- Type inference
- Type checking
- Generic instantiation
- Error diagnostics

### Phase 6: Optimization
**Status**: Planned

- Constant folding
- Dead code elimination
- Inlining
- Loop optimizations

### Phase 7: Code Generation
**Status**: Planned

- LLVM backend (primary)
- Native code generation
- Inline assembly support

## Module Structure

```
src/
├── main.rs              # CLI entry point
├── lexer/
│   ├── mod.rs          # Main lexer
│   ├── token.rs        # Token definitions
│   └── position.rs     # Position tracking
├── parser/
│   └── mod.rs          # Parser implementation
├── ast/
│   └── mod.rs          # AST definitions
└── compiler.rs         # Runtime evaluation
```

## Dependency Graph

```
main.rs
├── lexer/
│   ├── token.rs
│   └── position.rs
├── parser/
│   ├── ast/
│   ├── lexer/
│   └── position.rs
├── ast/
└── compiler/
    └── ast/
```

## Future Work

1. **Type System** (Phase 2)
   - Type checking and inference
   - Generic constraints
   - Trait system

2. **Advanced Features** (Phase 3-5)
   - Async/await
   - Concurrency primitives
   - Pipelines
   - Macros

3. **Optimization** (Phase 6-7)
   - IR optimization passes
   - LLVM code generation
   - Machine code output

4. **Self-Hosting** (Phase 10)
   - Rewrite compiler in XP
   - Bootstrap process
   - Full self-compilation

## Error Handling Strategy

- **Silent by Default**: Errors don't print unless explicitly handled
- **Try-Catch**: Explicit error handling with `try { } catch err { }`
- **Diagnostics Mode**: Optional verbose error reporting
- **Compilation Errors**: Always reported to prevent execution

## Performance Considerations

- **Incremental Compilation**: Only recompile changed modules
- **Parallel Compilation**: Independent modules compile in parallel
- **Caching**: `.xpc` cache format for build artifacts
- **Fast Startup**: Minimal initialization overhead
- **Deterministic Builds**: Reproducible compilation output
