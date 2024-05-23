# Explanation

## Dependencies:

```toml
    [dependencies]
    byteorder = "1.4.3"
    clap = { version = "4.0", features = ["derive"] }
```
- byteorder for reading binary data with a specific endianness.
- clap for parsing command-line arguments.  

Add these to your Cargo.toml as shown above.

### Cli Struct:

- Defines the structure of the command-line arguments.
- Uses clap's Parser derive macro for automatic argument parsing.

### MRCHeader Struct:

- Represents the MRC header fields.
- Includes methods for reading the header from a file and displaying the header contents.

### read_from_file Method:

- Reads binary data from the specified MRC file.
- Parses and populates the MRCHeader struct fields.

### display Method:

- Prints the values of the header fields to the console.

### Main Function:

- Uses clap to parse the command-line arguments.
- Reads the MRC header from the specified file and displays its contents.

To build and run the tool:

```bash
    cargo build --release
```

./target/release/your_binary_name path_to_mrc_file.mrc
Replace path_to_mrc_file.mrc with the path to your MRC file.