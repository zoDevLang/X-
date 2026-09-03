# X+ (XP) Language Specification

## Overview

X+ (XP) is a production-oriented systems programming language combining:
- Low-level power of C
- Systems capabilities of C++
- High-level ergonomics of C#
- Compact, expression-oriented syntax
- Fast compilation
- One-line program support

## Syntax Examples

### One-Liner Programs

```xp
#require <io>; fn main()=>print("Hello XP")
```

```xp
fn factorial(n:Int)=>n<=1?1:n*factorial(n-1); fn main()=>print(factorial(10))
```

### Variable Declaration

```xp
let x => 10
let name => "XP"
mut count => 0
count = 5
const PI:Float64 => 3.14159
```

### Functions

```xp
fn add(a:Int, b:Int) => a + b

fn calculate(x:Int):Int => {
    let y => x * 2
    let z => y + 10
    return z
}

fn factorial(n:Int) => n <= 1 ? 1 : n * factorial(n-1)
```

### Control Flow

```xp
if x > 10 {
    print("large")
} else {
    print("small")
}

while x < 100 {
    x++
}

for i in 0..10 {
    print(i)
}

match state {
    State.Ready => print("ready")
    State.Running => print("running")
    _ => print("unknown")
}
```

### Arrays

```xp
let nums => [1, 2, 3, 4, 5]
let nums:Int[] => [1, 2, 3]
let first => nums[0]
```

### Structs

```xp
struct User {
    name:String
    age:Int
}

let user => User{
    name:"Alex"
    age:20
}

user.name
```

### Classes

```xp
class Player {
    let name:String
    mut health:Int

    fn damage(amount:Int) => {
        health -= amount
    }
}
```

### Generics

```xp
fn max<T:Comparable>(a:T, b:T) => a > b ? a : b

struct Box<T> {
    value:T
}
```

## Type System

### Primitive Types

- `Int`, `UInt` (default integer types)
- `Int8`, `Int16`, `Int32`, `Int64`
- `UInt8`, `UInt16`, `UInt32`, `UInt64`
- `Float`, `Float32`, `Float64`
- `Bool`, `Char`, `String`, `Byte`
- `Void` (no return), `Any` (any type), `Never` (never returns)

### Composite Types

- Arrays: `Type[]`
- Pointers: `*Type`
- References: `&Type`
- Function types: `fn(ParamTypes):ReturnType`

## Operators

### Arithmetic
```
+ - * / % ++ --
```

### Comparison
```
== != < > <= >=
```

### Logical
```
&& || !
```

### Bitwise
```
& | ^ ~ << >>
```

### Assignment
```
= += -= *= /= %= &= |= ^= <<= >>=
```

## Dependencies

### Standard Library (`#require`)

```xp
#require <io>
#require <math>
#require <memory>
#require <io,math>  // Multiple
```

### Syntax Extensions (`#add`)

```xp
#add <async>
#add <pattern>
```

### External Packages (`#pick`)

```xp
#pick <http>
#pick <sqlite@3.4>
#pick <graphics> as gfx
```

## Error Handling

### Silent by Default

Errors do NOT print automatically. Errors must be explicitly handled:

```xp
try {
    result => open("file.txt")
} catch err {
    print(err)
}
```

### Diagnostics

Enable compiler diagnostics:

```xp
diagnostics on
```

## Async & Concurrency

### Async Functions

```xp
async fn download(url:String) => {
    // ...
}

let data => await download(url)
```

### Threads

```xp
thread {
    worker()
}

let t => thread worker()
t.join()
```

### Channels

```xp
let channel => Channel<Int>()
channel.send(42)
let x => channel.receive()
```

## Memory Management

### Pointers

```xp
let x:Int => 10
let p:*Int => &x
*p
*p = 20
```

### Manual Allocation

```xp
let p => alloc<Int>()
*p = 42
free(p)

let buffer => alloc<Int>(1024)
free(buffer)
```

### Unsafe Code

```xp
unsafe {
    // Low-level operations
}
```

## Compiler Commands

```bash
xp run main.xp          # Run a program
xp check main.xp        # Check syntax
xp build main.xp        # Build executable
xp fmt main.xp          # Format code
xp test                 # Run tests
xp clean                # Clean build artifacts
xp version              # Show version
```

## Package Manager (Shipment)

```bash
shm init                # Initialize new project
shm install http        # Install package
shm remove http         # Remove package
shm update              # Update dependencies
shm search database     # Search packages
shm publish             # Publish package
shm build               # Build project
```

## Project File (xp.pkg)

```
name: MyApp
version: 1.0.0

pick:
    http@2.1
    sqlite@3.4
```
