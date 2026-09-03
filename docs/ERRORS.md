# X+ Error System

## Philosophy

**Errors are silent by default.**

Unlike most languages that print errors automatically, X+ requires explicit error handling. This gives developers:
- Complete control over error behavior
- No surprise output in production code
- Clean separation of logic and diagnostics

## Default Behavior

### Without Error Handling

```xp
open("missing.txt")  // File not found - ERROR OCCURS
// Nothing is printed
// Program continues or errors internally
```

### With Explicit Handling

```xp
try {
    data => open("missing.txt")
    process(data)
} catch err {
    print("Error: ", err)  // Error is printed HERE
}
```

## Error Representation

### Internal Error Types

XP uses internal error/result mechanisms:

```xp
// Option<T> - a value or nothing
let maybe_value:Option<Int> => find_item()

// Result<T, E> - a value or an error
let result:Result<Data, Error> => load_file()
```

### Catching Errors

```xp
match result {
    Ok(data) => process(data)
    Err(e) => print("Failed: ", e)
}
```

## Diagnostics Mode

Enable diagnostics for detailed information:

```xp
diagnostics on
```

When enabled, the compiler/runtime may print:
- Type information
- Stack traces
- Variable states
- Compilation details

Diagnostics are kept **separate from normal output**.

## Compiler Errors

Compilation errors are **always reported** and prevent execution:

```bash
$ xp run bad.xp
error: Undefined variable 'x' at 5:10
```

Compilation must succeed before execution.

## Runtime Error Handling

### Silent Errors (Default)

```xp
let result => 10 / 0  // Division by zero
// No output - error handled internally
// result may be null or an error value
```

### Explicit Error Handling

```xp
try {
    result => 10 / 0
} catch err {
    print("Math error: ", err)
}
```

## Error Types

- **Compilation Errors**: Syntax, type, and name resolution errors
- **Runtime Errors**: Division by zero, null access, bounds violations
- **Async Errors**: Task failures, cancellation, timeouts
- **Custom Errors**: User-defined error types

## Best Practices

1. **Explicit Handling**: Always handle errors that matter
2. **Fail Fast**: Check preconditions early
3. **Clear Messages**: Provide context in error handling
4. **Logging**: Use diagnostics mode for debugging
5. **Production Code**: Ensure no unhandled errors leak

## Future: Result Type

```xp
fn safe_divide(a:Int, b:Int):Result<Int, String> => {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

match safe_divide(10, 2) {
    Ok(result) => print(result)
    Err(msg) => print("Error: ", msg)
}
```
