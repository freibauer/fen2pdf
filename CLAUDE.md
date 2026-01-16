# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
FEN2PDF is a professional-quality PDF generator that creates multi-page chess position diagrams from Lichess studies. Single Rust binary that downloads Lichess study PGN files and generates A4 PDFs with 9 positions per page in a 3x3 grid layout.

## Common Commands

### Build and Run
```bash
./build.sh                      # Builds the application (checks dependencies, compiles Rust binary)
./clean.sh                      # Cleans build artifacts and temporary files
./clean.sh --all               # Clean build artifacts and Cargo cache
```

### Development
```bash
cargo build --release          # Build optimized binary
cargo run -- <study-id>        # Build and run directly with study ID
cargo test                     # Run tests
cargo clean                    # Clean build artifacts
```

### Usage
```bash
./fen2pdf <study-id>           # Direct binary execution (study-id required)
cargo run -- <study-id>       # Build and run directly with study ID
```

## Code Architecture

### Main Application (`src/main.rs`)
Single Rust file containing all logic:
- **Command Line Parsing**: Accepts study ID and constructs Lichess URL
- **Study Validation**: Validates study exists and contains chess positions
- **Lichess Study Parser**: Downloads PGN files via HTTP and extracts FEN positions  
- **PDF Generator**: Creates multi-page A4 layouts using printpdf library
- **Board Renderer**: Handles coordinate systems and piece placement with embedded PNGs

### Piece Assets (`src/pieces.rs`)
Contains embedded PNG data for all 12 chess pieces using `include_bytes!` macro:
- **White pieces**: WK, WQ, WR, WB, WN, WP
- **Black pieces**: BK, BQ, BR, BB, BN, BP
- **Access function**: `get_piece_png_data(piece: char) -> Option<&'static [u8]>`

### Key Data Structures
```rust
#[derive(Debug, Clone)]
struct ChessPosition {
    number: i32,
    description: String,
    fen: String,
    black_to_move: bool,  // Auto-inverts board perspective
}

#[derive(Debug, Clone)]  
struct StudyData {
    name: String,
    positions: Vec<ChessPosition>,
}
```

### Critical Implementation Details
1. **Study ID Input**: Takes Lichess study ID (e.g., `hVLtgoSL`) and constructs URL automatically
2. **Error Handling**: Validates study exists, contains positions, prevents PDF creation on failure
3. **Dynamic PDF Naming**: PDF filename uses StudyName with spaces replaced by underscores
4. **Board Orientation**: Automatically flips board for black-to-move positions
5. **Text Formatting**: Colon (:) in position descriptions triggers line feed in PDF
6. **PDF Layout**: 3x3 grid (9 positions/page), 75mm boards with proper spacing
7. **High Resolution**: 600px board images for crisp PDF rendering

### File Processing Flow
1. Parses command line for study ID (required)
2. Downloads Lichess study PGN → `lichess_study.pgn`
3. Validates study contains chess positions
4. Parses PGN to extract FEN positions and study name
5. Generates PDF with embedded PNG boards → `<StudyName>.pdf`
6. Self-contained executable with no external dependencies

## Dependencies

### Rust Crates
```toml
[dependencies]
resvg = "0.38"          # SVG rendering capability (for future asset processing)
usvg = "0.38"           # SVG parsing (for future asset processing)
tiny-skia = "0.11"      # 2D graphics rasterization
printpdf = "0.7"        # PDF generation with embedded image support
reqwest = "0.11"        # HTTP client for Lichess downloads (blocking, rustls-tls)
regex = "1.10"          # Regular expression processing for PGN parsing
anyhow = "1.0"          # Error handling and propagation
```

### No External Dependencies
- All chess piece graphics embedded in binary
- No external ImageMagick or asset files required  
- Fully self-contained executable

## Customization Points

### Layout Constants (src/main.rs)
Key layout parameters:
- `BOARD_SIZE = 75.0` - Board size in mm (75x75mm)
- `BOARDS_PER_PAGE = 9` - 3x3 grid layout
- `BOARD_SPACING` - Space between boards
- `PAGE_WIDTH/HEIGHT` - A4 dimensions (210x297mm)

### Visual Styling
- Board squares: Light gray background (#DDDDDD), white squares
- Typography: TimesRoman for coordinates and descriptions  
- High contrast coordinates for visibility
- Study name at top, page numbers at bottom

## Implementation Status

### ✅ Completed Features
- Command line argument parsing (study ID required)
- Lichess study download and validation
- Study not found error handling (prevents PDF creation)
- Dynamic PDF filename based on study name
- FEN position parsing with black-to-move detection
- Automatic board flipping for black perspective
- Chess piece rendering with embedded PNGs (12 pieces)
- Multi-page PDF generation with proper layout
- Colon-triggered line breaks in position descriptions
- Board coordinate rendering (a-h, 1-8)
- Professional A4 layout with 3x3 grid
- Study name headers and page numbering
- Self-contained executable (8.2MB) with no external dependencies

### Usage Examples
```bash
# Example with a specific study ID
./fen2pdf hVLtgoSL

# Build and run with Cargo
cargo run -- hVLtgoSL

# Build first, then run
./build.sh
./fen2pdf ABC123
```

### Output
- PDF named after study (e.g., `WM25.pdf`, `Chess_Tactics.pdf`)
- High-quality 75mm chess boards in 3x3 grid
- Professional layout suitable for printing
- Embedded chess piece graphics for portability

### Key Features
- **Study ID Input**: Simple study ID input instead of full URLs
- **Error Prevention**: No PDF created if study not found or invalid
- **Smart Naming**: PDF filename uses actual study name with underscore replacement
- **Text Formatting**: Colon (:) creates line breaks in position descriptions  
- **Quality Output**: High-resolution boards with proper coordinate labeling
- **Portability**: Single executable with all assets embedded

## Recent Updates

## Build System
Uses standard Rust toolchain with shell script wrappers:
- `build.sh` - Compile release binary and create symlink for easy access
- `clean.sh` - Clean build artifacts and temporary files (use --all for Cargo cache)

### Project Structure
```
fen2pdf/
├── src/
│   ├── main.rs          # Main application logic, PDF generation, PGN parsing
│   └── pieces.rs        # Embedded PNG chess piece assets
├── assets/
│   └── png/             # Source PNG files for chess pieces (12 pieces)
├── Cargo.toml           # Rust dependencies and project configuration
├── build.sh             # Build script with dependency checking
├── clean.sh             # Cleanup script with optional deep clean
└── CLAUDE.md            # This documentation file
```

Project optimized for simplicity with minimal source files and embedded assets.
