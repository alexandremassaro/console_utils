# Console Utils

`console_utils` is a Rust library that provides utility functions for handling terminal interactions. It is designed to simplify tasks such as moving the cursor, clearing lines, and getting the current cursor position.

## Features

- Move the cursor to a specific line
- Clear the current line
- Get the current cursor position

## Dependencies

- `crossterm` for terminal interactions

## Usage

### Adding to Your Project

To use `console_utils` in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
console_utils = { path = "../console_utils" }
```

### Functions

#### `move_cursor_to`

Moves the cursor to the specified line.

```rust
use console_utils::move_cursor_to;

move_cursor_to(5)?;
```

#### `clear_current_line`

Clears the current line in the terminal.

```rust
use console_utils::clear_current_line;

clear_current_line()?;
```

#### `get_cursor_position`

Gets the current cursor position.

```rust
use console_utils::get_cursor_position;

let (x, y) = get_cursor_position()?;
```

## License

This project is licensed under the MIT License.

