# X+ Programming Language (XP)

**Complex by design. Powerful by nature.**

X+ is a production-oriented programming language combining:
- The low-level power of **C**
- The systems/programming power of **C++**
- The high-level features and developer ergonomics of **C#**
- Compact, expression-oriented syntax
- Fast compilation
- One-line programs as a first-class feature
- Strong metaprogramming
- Native interoperability
- Manual memory control when required
- Modern concurrency
- Self-hosting / bootstrapping

## Project Status

**Phase 1: Core Language & Compiler Infrastructure**
- [ ] Project structure
- [ ] Lexer
- [ ] Parser
- [ ] AST
- [ ] Basic expressions
- [ ] Variables & functions
- [ ] Control flow (if, loops)
- [ ] Basic compiler

## Quick Start

```bash
# Build the compiler
cargo build --release

# Run an XP program
xp run hello.xp

# Check syntax
xp check program.xp

# Format code
xp fmt program.xp
```

## Language Examples

### One-liner
```xp
#require <io>; fn main()=>print("Hello XP")
```

### Factorial
```xp
fn factorial(n:Int)=>n<=1?1:n*factorial(n-1)
```

### With explicit types and blocks
```xp
fn calculate(x:Int):Int => {
    let y => x * 2
    let z => y + 10
    return z
}
```

## Project Structure

```
X+/
├── src/
│   ├── lexer/           # Tokenization
│   ├── parser/          # Syntax analysis
│   ├── ast/             # Abstract syntax tree
│   ├── resolver/        # Name resolution
│   ├── types/           # Type system
│   ├── checker/         # Type checking
│   ├── ir/              # Intermediate representation
│   ├── codegen/         # Code generation
│   ├── compiler.rs      # Main compiler driver
│   └── main.rs          # CLI interface
│
├── stdlib/              # Standard library
├── tests/               # Comprehensive test suite
├── docs/                # Language documentation
└── Cargo.toml           # Rust dependencies
```

## Development Phases

1. **Phase 1**: Lexer, parser, basic expressions, functions, control flow
2. **Phase 2**: Type system, structs, arrays, enums, modules
3. **Phase 3**: Generics, classes, interfaces, lambdas, pipelines
4. **Phase 4**: Pointers, unsafe, manual memory, native ABI
5. **Phase 5**: Async, threads, channels, parallel execution
6. **Phase 6**: `#require`, `#add`, `#pick`, Shipment package manager
7. **Phase 7**: Compiler caching, incremental compilation, parallel builds
8. **Phase 8**: C/C++ interoperability, inline assembly
9. **Phase 9**: Macros, compile-time programming, optimization
10. **Phase 10**: Self-hosting bootstrap

## Design Principles

- **Compact & Readable**: High information density without sacrificing logic
- **Silent Errors**: Errors don't print by default—explicit handling required
- **Expression-Oriented**: Everything is an expression when possible
- **One-Line Support**: Multiple declarations separated by `;` on one line
- **Fast Compilation**: Incremental, parallel, cached compilation via `.xpc`
- **Self-Hosting**: Compiler written in XP, compiles itself

## Documentation

- [Language Specification](./docs/SPEC.md) — Complete language design
- [Compiler Architecture](./docs/ARCHITECTURE.md) — Implementation details
- [Error System](./docs/ERRORS.md) — Error handling and diagnostics

## Contributing

This is an active compiler engineering project. Contributions welcome for:
- Lexer/parser improvements
- Type system features
- Standard library implementation
- Test coverage
- Documentation

## License

MIT

---

**Lead Compiler Engineer**: zoDevLang

**Project Start**: 2026

**Target**: Production-ready X+ compiler with self-hosting capability
