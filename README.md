# JSON Serialization in Rust Using Serde

A simple Rust program demonstrating how to create custom data structures and serialize them into JSON using the `serde` and `serde_json` crates.

## Overview

This project defines two structs, `Article` and `Paragraph`, to represent an article with multiple paragraphs. It then uses Serde to convert the Rust data into a JSON string and prints the result to the terminal.

## Example Output

```json
{
  "article": "how to work with json in rust",
  "author": "arjun",
  "paragraph": [
    {
      "name": "starting sentence"
    },
    {
      "name": "body of the article"
    },
    {
      "name": "end of the article"
    }
  ]
}
```

*Note: The actual program prints the JSON as a single line without indentation.*

## Usage

1. Clone this repository or download the source code.
2. Navigate to the project directory.
3. Run the program:

   ```bash
   cargo run
   ```

## Limitations

- The program uses hardcoded article data rather than accepting user input.
- The JSON output is serialized into a compact, single-line string.
