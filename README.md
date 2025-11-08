# quick-timer

A simple Rust timer macro library for measuring execution time of code blocks.

## Features

- Easy to use macro syntax for timing code blocks
- Two types of timers:
  - `timer!` - Prints timing information to stdout (only in debug builds by default)
  - `timer_silent!` - Returns timing information without printing (always active)
- Multiple syntax options for flexibility
- Zero overhead in release builds (unless `release_also` feature is enabled)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
quick-timer = "0.1.0"
```

## Examples

### Basic Usage

```rust
use quick_timer::timer;

fn main() {
    timer! {
        println!("Hello, world!");
    }
}
```

### Using Tags

```rust
use quick_timer::timer;

fn main() {
    // With a tag
    timer!(# "My Function" {
        // Some expensive operation
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
    
    // Alternative syntax
    timer! {
        # "Another Function"
        println!("Doing some work...");
    }
}
```

### Getting Results

```rust
use quick_timer::timer;

fn main() {
    let result = timer! {
        1 + 1
    };
    assert_eq!(result, 2);
}
```

### Silent Timer

```rust
use quick_timer::timer_silent;

fn main() {
    let (result, duration) = timer_silent! {
        println!("This will be printed");
        42
    };
    
    assert_eq!(result, 42);
    println!("Execution took {} ms", duration.as_millis());
}
```

### Release Mode

By default, `timer!` macro only works in debug builds. To enable it in release builds as well, enable the `release_also` feature:

```toml
[dependencies]
quick-timer = { version = "0.1.0", features = ["release_also"] }
```

## Syntax Variants

The `timer!` macro supports multiple syntax variants:

```rust
use quick_timer::timer;

fn main() {
    // Basic form
    timer! { /* code */ }
    
    // With block syntax
    timer!(block: { /* code */ });
    
    // With tag
    timer!(# "Tag" { /* code */ });
    
    // With identifier tag
    timer!(# Tag { /* code */ });
    
    // Explicit tag syntax
    timer!(tag: "Tag", block: { /* code */ });
    
    // Braceless forms
    timer! { # "Tag" /* code */ }
}
```

## License

This project is licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.