# asciicolor
A tiny library for coloring text in ASCII terminals

## Example

```rust
use asciicolor::Colorize;

fn main() {
    print!("Hello, color!".blue().underline().bold())
}

```