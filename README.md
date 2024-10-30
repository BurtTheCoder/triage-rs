## WORK IN PROGRESS ##

# Forensic Triage Tool

A high-performance digital forensics tool for automated system triage and artifact collection, built in Rust using The Sleuth Kit (TSK) bindings.

## Features

- Fast parallel processing of forensic images
- Support for E01, RAW, and virtual disk formats
- Windows and Linux artifact collection
- Registry analysis and parsing
- Automated system information extraction
- Artifact collection and hashing
- Progress tracking and detailed logging

## Prerequisites

- Rust 1.70 or higher
- The Sleuth Kit 4.12 or higher
- For Windows: Visual Studio 2019+ with Windows SDK
- For Linux: Development tools and headers

## Installation


## Usage

```bash
triage <image_path> [options]

Options:
  -o, --output <dir>    Output directory (default: ./output)
  -t, --threads <num>   Number of threads to use
  -v, --verbose        Enable verbose logging
```

## Example

```bash
triage /path/to/image.E01 -o case_output -t 8
```

## Supported Artifacts

### Windows
- Registry hives
- Event logs
- Prefetch files
- User profiles
- Browser history
- System configuration

### Linux
- System logs
- User data
- Configuration files
- Package information

## Project Structure

```
forensic-triage/
├── src/
│   ├── image/        # Image handling
│   ├── artifacts/    # Artifact collection
│   ├── registry/     # Registry parsing
│   ├── filesystem/   # Filesystem operations
│   └── utils/        # Common utilities
```

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

