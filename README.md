# Rust Minesweeper

A Rust implementation for adding mine counts to empty squares in a completed Minesweeper board. This project takes a minefield represented by a 2D array of strings and annotates each empty square with the count of adjacent mines.

## Features

- Counts adjacent mines for each empty square.
- Handles various board configurations, including edges and corners.
- Implements thorough test cases to ensure accuracy.

## Usage

To use this library, include the function in your project and call `annotate` with a reference to your minefield:

```rust
// Add the following to your main.rs or another file
use rust_minesweeper::annotate;

fn main() {
    let minefield = vec![
        " * ",
        " * ",
        "   ",
    ];

    let annotated_board = annotate(&minefield);

    for row in annotated_board {
        println!("{}", row);
    }
}
```

### Example Output
For the input above, the output will be:

```text
2*2
2*2
111
```

## Running Tests
To run the tests, execute the following command:

```cmd
cargo test
```

