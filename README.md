# FEN2PDF

FEN2PDF is a professional-quality PDF generator that creates multi-page chess position diagrams from Lichess studies. This cross-platform Rust application downloads Lichess study PGN files and generates high-quality A4 PDFs with 9 positions per page in a 3x3 grid layout.

## Features

- **Cross-platform support**: Available for Linux and Windows
- **Lichess integration**: Downloads studies directly by study ID
- **Professional layout**: 75mm chess boards in 3x3 grid on A4 pages
- **High-quality output**: 600px resolution boards for crisp PDF rendering
- **Apple PDF compatibility**: Optimized for viewing on all PDF readers
- **Self-contained**: No external dependencies, all chess piece graphics embedded
- **Smart formatting**: Automatic board flipping for black-to-move positions
- **Text formatting**: Colon (:) in descriptions triggers line breaks

## Installation & Build

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- For Windows cross-compilation: mingw-w64

### Build
```bash
./build.sh                # Builds both Linux and Windows executables
```

This creates:
- `fen2pdf` - Linux executable
- `fen2pdf.exe` - Windows executable

### Alternative build commands
```bash
cargo build --release                           # Linux only
cargo build --release --target x86_64-pc-windows-gnu  # Windows cross-compile
./clean.sh                                     # Clean build artifacts
```

## Usage

```bash
# Linux
./fen2pdf <study-id>

# Windows  
fen2pdf.exe <study-id>

# Examples
./fen2pdf hVLtgoSL        # Downloads and converts study to "WM25.pdf"
./fen2pdf ABC123          # Creates "StudyName.pdf" based on actual study name
```

### Input
- **Study ID**: Lichess study identifier (e.g., `hVLtgoSL` from `https://lichess.org/study/hVLtgoSL`)
- The application automatically constructs the Lichess URL and downloads the PGN

### Output
- PDF named after the study (spaces replaced with underscores)
- 3x3 grid layout with 9 chess positions per page
- High-quality embedded chess piece graphics
- Board coordinates (a-h, 1-8) and position descriptions
- Study title header and page numbering

## Technical Details

### Architecture
- **Single Rust file**: All logic in `src/main.rs`
- **Embedded assets**: Chess pieces stored as PNG data in `src/pieces.rs`
- **Cross-platform**: Builds for Linux and Windows using mingw-w64

### Key Libraries
- `printpdf` - PDF generation with embedded images
- `reqwest` - HTTP client for Lichess downloads
- `tiny-skia` - 2D graphics rendering for chess boards
- `regex` - PGN parsing and text processing

### File Processing Flow
1. Parse command line for study ID
2. Download Lichess study PGN
3. Validate study contains chess positions
4. Parse FEN positions and study metadata
5. Generate PDF with RGB image data for maximum compatibility

### Layout Specifications
- **Page size**: A4 (210×297mm)
- **Board size**: 75×75mm per position
- **Grid**: 3×3 layout (9 positions per page)
- **Resolution**: 600px boards for crisp rendering
- **Margins**: Optimized for printing

## Recent Updates

### v0.1.1
- ✅ Fixed Windows temporary file path issue (cross-platform temp directory support)
- ✅ Apple PDF viewer compatibility fix (RGB image format)
- ✅ Cross-platform Windows build support
- ✅ Embedded chess piece graphics (no external files)
- ✅ Professional A4 layout with proper spacing
- ✅ Dynamic PDF naming from study titles
- ✅ Automatic board orientation for black-to-move

## Project Structure

```
fen2pdf/
├── src/
│   ├── main.rs          # Main application logic
│   └── pieces.rs        # Embedded chess piece PNG data
├── assets/png/          # Source chess piece images
├── build.sh             # Cross-platform build script
├── clean.sh             # Cleanup script
├── Cargo.toml           # Rust dependencies
├── README.md            # This file
└── CLAUDE.md            # Development documentation
```

## License

Open source project for chess study documentation and analysis.
