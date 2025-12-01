# ro-crate-rust

A Rust implementation of the [RO-Crate](https://www.researchobject.org/ro-crate/) metadata specification for Research Objects. This project provides both a library interface for managing RO-Crate metadata within Rust applications and a CLI tool for command-line interaction with RO-Crate files.

## Features

- Create, validate, and manipulate RO-Crate metadata files and directories
- Parse and generate RO-Crate metadata in JSON-LD format
- Easy integration as a Rust library or use as a CLI tool
- Strongly typed models for RO-Crate entities

---

## Library Usage

To use `ro-crate-rust` as a library, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
ro-crate-rust = "0.1.0"
# Replace with the latest version
```

### Main Types

- `RoCrate`: The primary type representing a RO-Crate package.
- `Entity`: Represents an object in the RO-Crate metadata graph (such as files, people, organizations).
- `Result`: Alias for the crate's common result type.

### Load an Existing RO-Crate

```rust
use ro_crate_rust::{RoCrate, Result};

fn main() -> Result<()> {
    // Load an existing RO-Crate from a directory
    let crate_path = "path/to/ro-crate";
    let ro_crate = RoCrate::load(crate_path)?;

    // Access metadata (for example)
    println!("Name: {}", ro_crate.metadata().name);

    // Iterate files
    for file in ro_crate.files() {
        println!("File: {:?}", file.path);
    }

    Ok(())
}
```

### Create a New RO-Crate

```rust
use ro_crate_rust::{RoCrate, Entity, Result};

fn main() -> Result<()> {
    // Start a new RO-Crate with a name
    let mut crate = RoCrate::new("Example Crate");

    // Add a file
    crate.add_file("README.md", "Project description");

    // Add an author entity
    let author = Entity::Person {
        id: "https://orcid.org/0000-0002-1825-0097".to_string(),
        name: "Jane Doe".to_string(),
        // fill other fields as needed
    };
    crate.add_entity(author);

    // Save crate to a directory
    crate.save("./output_crate")?;
    Ok(())
}
```

### Add a Custom Metadata Entity

You can add custom entities to the metadata graph:

```rust
use ro_crate_rust::{RoCrate, Entity};

let mut crate = RoCrate::new("Custom Crate");
let dataset = Entity::Dataset {
    id: "data.csv".to_string(),
    name: "Experiment Data".to_string(),
    // any other relevant fields
};
crate.add_entity(dataset);
```

### Validate an RO-Crate

Check if the crate is structurally valid:

```rust
let is_valid = ro_crate.is_valid();
if is_valid {
    println!("RO-Crate is valid!");
} else {
    println!("RO-Crate is invalid!");
}
```

### Common Methods

| Method               | Description                                                |
|----------------------|-----------------------------------------------------------|
| `RoCrate::load(path)`            | Load crate from directory or zip file                  |
| `RoCrate::new(name)`             | Create a new empty crate                              |
| `crate.add_file(path, descr)`    | Add a file with path and description                  |
| `crate.add_entity(entity)`       | Add a metadata entity (Person, Dataset, etc)          |
| `crate.files()`                  | List files in the crate                               |
| `crate.metadata()`               | Access root crate metadata                            |
| `crate.save(path)`               | Save crate to the specified directory                 |
| `crate.is_valid()`               | Validate the crate structure                          |

### Example Entities

Some typical entities:

```rust
Entity::Person {
    id: String,         // e.g. ORCID or URL
    name: String,       // person's name
    // additional fields...
}

Entity::Dataset {
    id: String,         // file path or URI
    name: String,       // dataset name
}
```

For more details, review the source code for available entity types and options.

---

## CLI Usage

After installing (see Installation below), you can use the CLI:

### Initialize a RO-Crate

```bash
ro-crate-rust --crate-dir path/to/output init
```

### Add a File to a Crate

```bash
ro-crate-rust --crate-dir path/to/crate add -P "description=Lorem Ipsum" -p path/to/file
```

### Export ro-crate as zip

```bash
ro-crate-rust --crate-dir path/to/crate write-zip -p path/to/destination
```
---

## Installation

### Library

Add to `Cargo.toml` as shown above.

### CLI

Install via Cargo:

```bash
cargo install --git https://github.com/arunaengine/ro-crate-rust
```

Or build manually:

```bash
git clone https://github.com/arunaengine/ro-crate-rust.git
cd ro-crate-rust
cargo build --release
```

The binary will be at `target/release/ro-crate-rust`.

---

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) before opening issues or PRs.

## License

See [LICENSE](LICENSE) for license details.

---

## Reference

- [RO-Crate spec](https://www.researchobject.org/ro-crate/)
- [ro-crate-rust GitHub](https://github.com/arunaengine/ro-crate-rust)
