# rpx2json

RPX to JSON converter for Across Report (ACR)

---

## Overview

`rpx2json` is a command-line tool designed to convert RPX (ActiveReports XML format) into a structured JSON format used by ACR (Across Report).

This repository provides the **public interface and CLI entry point** for the conversion process.

---

## Purpose

* Enable migration from legacy RPX templates
* Provide a standardized JSON format for ACR
* Support cross-platform report processing

---

## Features

* Command-line interface (CLI)
* RPX file input
* JSON output (ACR-compatible structure)
* Cross-platform (Windows / Linux / macOS)

---

## Status

This is the **public edition** of the tool.

* Core conversion logic is **not included**
* Intended for interface, usage, and integration reference

---

## Usage

```bash
rpx2json <input.rpx>
```

Example:

```bash
rpx2json sample.rpx > output.json
```

---

## Output Format

The generated JSON follows the ACR template specification.

Example structure:

```json
{
  "page": {
    "width": 2100,
    "height": 2970
  },
  "sections": [
    {
      "type": "detail",
      "controls": []
    }
  ]
}
```

---

## Project Structure

```
rpx2json/
├── src/
│   └── main.rs
├── Cargo.toml
├── README.md
```

---

## Build

```bash
cargo build --release
```

---

## Run

```bash
cargo run -- <input.rpx>
```

---

## Notes

* This repository does **not contain the full conversion engine**
* Internal parsing, layout processing, and rendering logic are maintained separately
* This design prevents exposure of proprietary implementation details

---

## Internal Version

The complete implementation is maintained in a private repository:

```
rpx2json-internal
```

Access is restricted to authorized members.

---

## Security / Design Policy

* Core algorithms are not exposed publicly
* Public repository is limited to interface-level functionality
* Internal repository contains full processing pipeline

---

## Related Projects

* acr-engine (rendering engine)
* acr-spec (template specification)
* acrossreport-site (documentation)

---

## Future Plans

* Expand CLI options
* Add validation tools
* Provide schema definitions for JSON output
* Improve integration with ACR runtime

---

## License

Currently not specified.

Please contact the author for usage permissions.

---

## Author

Across Report (ACR)
