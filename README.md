# quick-timer

A simple rust timer macro library

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
quick-timer = "0.1.0"
```

## Example

```rust
use quick_timer::timer;

fn main() {
    timer! {
        println!("Hello, world!");
    }
}
```

## License

This project is licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.