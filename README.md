# Xenoverse 2 Save Converter

A Rust-based tool for converting Xenoverse 2 save files between PS4 and PC formats.

## Command Line Tool

**Windows**: Drag and drop your save file (SDATA000.DAT or EditorReady.sav) onto `xv2_converter.exe`

**Linux/Mac**:

```bash
./xv2_converter <save_file>
```

The tool automatically detects the format and converts appropriately.

## Features

- Convert PS4 save files to PC-ready format
- Convert PC-ready save files to PS4 format
- Automatic format detection and conversion
- Support for handling leftover data
- Available as Rust, Python, and C libraries

## Building the Libraries

### Rust Library

The core functionality is implemented in Rust. To build the library:

```bash
# Build the default library
cargo build --release

# Build with Python bindings
cargo build --release --features python

# Build with C bindings
cargo build --release --features c
```

### Python Library

To build and install the Python library, you need `maturin`:

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop --features python

# Or build a wheel
maturin build --features python
```

### C Library

The C library is built as a shared object file:

```bash
# Build the C library
cargo build --release --features c

# The library will be at target/release/libxv2_converter_lib.so (Linux)
# or target/release/libxv2_converter_lib.dylib (macOS)
# or target/release/xv2_converter_lib.dll (Windows)
```

## Using the Libraries

### Rust Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
xenoverse2-save-converter = { path = "./" }
```

Then use in your code:

```rust
use xv2_converter_lib::{ps4_to_pcready, pcready_to_ps4, convert_auto};

// Convert PS4 to PC-ready
let pc_ready_data = ps4_to_pcready(&ps4_data, "input.dat", ".")?;

// Convert PC-ready to PS4
let ps4_data = pcready_to_ps4(&pc_ready_data, "input.sav", ".", false)?;

// Auto-detect and convert
let converted_data = convert_auto(&data, "input.dat", ".")?;
```

### Python Usage

After installing the Python library:

```python
from xv2_converter_lib import PyXenoverse2Converter

# Create converter instance
converter = PyXenoverse2Converter()

# Convert PS4 to PC-ready
pc_ready_data = converter.ps4_to_pcready(ps4_data, 'input.dat', '.')

# Convert PC-ready to PS4
ps4_data = converter.pcready_to_ps4(pc_ready_data, 'input.sav', '.', False)

# Auto-detect and convert
converted_data = converter.convert_auto(data, 'input.dat', '.')
```

### C Usage

Include the header and link against the library:

```c
#include "xenoverse2_converter.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    // Load your save data
    FILE* file = fopen("input.dat", "rb");
    fseek(file, 0, SEEK_END);
    size_t input_size = ftell(file);
    fseek(file, 0, SEEK_SET);

    uint8_t* input_data = malloc(input_size);
    fread(input_data, 1, input_size, file);
    fclose(file);

    // Convert PS4 to PC-ready
    size_t output_len;
    uint8_t* result = ps4_to_pcready_c(input_data, input_size, "input.dat", ".", &output_len);

    if (result != NULL) {
        // Process the result
        // ...

        // Free allocated memory
        free_buffer(result);
    }

    free(input_data);
    return 0;
}
```

To compile with the C library:

```bash
# On Linux
gcc -I. your_program.c target/release/libxv2_converter_lib.so -o your_program -ldl

# On macOS
gcc -I. your_program.c target/release/libxv2_converter_lib.dylib -o your_program

# On Windows
gcc -I. your_program.c target/release/xv2_converter_lib.dll -o your_program.exe
```

## Credits

Based on extensive research by Gabrieluto on Xenoverse 2 save formats, his original save conversion tool, and shared information about the save format structure.

## License

This software is licensed under the Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License (CC BY-NC-SA 4.0). You are free to use, modify, and share this software for non-commercial purposes only. Commercial use and distribution for profit are strictly prohibited.

For commercial use, redistribution, or incorporation into products sold commercially, a separate commercial license is required. Contact jelaxxa on Discord for commercial licensing inquiries.

See the license deed at: https://creativecommons.org/licenses/by-nc-sa/4.0/
