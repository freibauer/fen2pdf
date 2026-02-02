use anyhow::{Result, anyhow};
use printpdf::*;
use std::fs;
use std::io::Write;

mod pieces;

#[derive(Debug, Clone)]
struct ChessPosition {
    number: i32,
    description: String,
    fen: String,
    black_to_move: bool,
}

#[derive(Debug, Clone)]
struct StudyData {
    name: String,
    positions: Vec<ChessPosition>,
}

// A4 dimensions in mm (f32 for printpdf compatibility)
const PAGE_WIDTH: f32 = 210.0;
const PAGE_HEIGHT: f32 = 297.0;
const MARGIN_LEFT: f32 = 30.0;    // Moderate left margin
const MARGIN_RIGHT: f32 = 12.0; 
const MARGIN_TOP: f32 = 35.0;     // Moderate top margin
const MARGIN_BOTTOM: f32 = 10.0;

// Board layout - 3x3 grid with balanced spacing
const BOARDS_PER_ROW: usize = 3;
const BOARDS_PER_COL: usize = 3;
const BOARDS_PER_PAGE: usize = 9;

// Board spacing and sizing - improved layout
const DESC_HEIGHT: f32 = 12.0;     // More space for larger text
const BOARD_DESC_GAP: f32 = 10.0;   // Gap between board and description

// Calculate maximum board size for A4: 
// Width: (210mm - 20mm margins - 2*3mm spacing) / 3 = ~60mm per column
// Height per row: (297mm - 20mm margins) / 3 = ~85mm per row
// Board size: 85mm - 8mm text = ~77mm available
// Let's use ~75mm for comfortable fit
const BOARD_SIZE: f32 = 75.0;  // Much larger: 75mm x 75mm boards!


fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <study-id>", args[0]);
        eprintln!("Note: A colon (:) in position descriptions triggers a line feed in the PDF");
        std::process::exit(1);
    }
    
    let study_id = &args[1];
    let lichess_url = format!("https://lichess.org/study/{}.pgn", study_id);
    println!("Using Lichess study ID: {}", study_id);
    println!("Downloading from: {}", lichess_url);
    
    // Create a random temporary filename for the PGN download
    let temp_dir = std::env::temp_dir();
    let temp_pgn_file = temp_dir.join(format!("lichess_study_{}.pgn", std::process::id()))
        .to_string_lossy()
        .to_string();
    println!("Using temporary file: {}", temp_pgn_file);
    
    // Download the latest study data from Lichess
    println!("Downloading Lichess study data...");
    download_lichess_study(&lichess_url, &temp_pgn_file)?;
    
    println!("Reading study positions...");
    let study_data = read_lichess_study(&temp_pgn_file)?;
    println!("Found {} positions in study: {}", study_data.positions.len(), study_data.name);
    
    println!("Creating PDF...");
    let pdf_filename = format!("{}.pdf", study_data.name.replace(' ', "_").replace('.', ""));
    create_pdf(&study_data, &pdf_filename)?;
    
    println!("Generated PDF: {} with {} chess positions", pdf_filename, study_data.positions.len());
    Ok(())
}

fn download_lichess_study(url: &str, filename: &str) -> Result<()> {
    println!("Sending HTTP request to: {}", url);
    let response = reqwest::blocking::get(url)?;
    
    // Check if the response is successful
    if !response.status().is_success() {
        return Err(anyhow!("Study not found: HTTP {}", response.status()));
    }
    
    println!("Got HTTP response, reading content...");
    let content = response.text()?;
    
    // Check if content looks like a valid PGN (should contain study data)
    if content.trim().is_empty() || (!content.contains("[Event") && !content.contains("[StudyName")) {
        return Err(anyhow!("Study not found or invalid: no chess positions detected"));
    }
    
    println!("Downloaded {} bytes, writing to file...", content.len());
    
    let mut file = std::fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    println!("File written successfully: {}", filename);
    
    Ok(())
}


fn read_lichess_study(filename: &str) -> Result<StudyData> {
    let content = fs::read_to_string(filename)?;
    let mut positions = Vec::new();
    let mut position_number = 1;
    let mut current_event = String::new();
    let mut current_chapter = String::new();
    let mut current_fen = String::new();
    let mut study_name = String::new();
    
    // Extract study name from the first [Event] line which usually contains the study name
    let mut found_study_name = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        // Parse StudyName line first (higher priority)
        if line.starts_with("[StudyName \"") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if end > start {
                        study_name = line[start + 1..end].to_string();
                        found_study_name = true;
                    }
                }
            }
        }
        
        // Parse ChapterName line
        if line.starts_with("[ChapterName \"") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if end > start {
                        current_chapter = line[start + 1..end].to_string();
                    }
                }
            }
        }
        
        // Parse Event line
        if line.starts_with("[Event \"") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if end > start {
                        current_event = line[start + 1..end].to_string();
                        
                        // Use the first Event as the study name if we haven't found StudyName yet
                        if !found_study_name {
                            study_name = current_event.clone();
                            // Remove "WM25: " prefix if present for the study name
                            study_name = study_name.strip_prefix("WM25: ").unwrap_or(&study_name).to_string();
                        }
                    }
                }
            }
        }
        
        // Parse FEN line
        if line.starts_with("[FEN \"") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if end > start {
                        current_fen = line[start + 1..end].to_string();
                    }
                }
            }
        }
        
        // When we have ChapterName and FEN, create position
        if !current_chapter.is_empty() && !current_fen.is_empty() {
            let black_to_move = current_fen.contains(" b ");
            let pos = ChessPosition {
                number: position_number,
                description: current_chapter.clone(),
                fen: current_fen.clone(),
                black_to_move,
            };
            positions.push(pos);
            position_number += 1;
            
            // Reset for next position
            current_chapter.clear();
            current_fen.clear();
        }
    }
    
    // If no study name found, use a default
    if study_name.is_empty() {
        study_name = "Chess Positions".to_string();
    }
    
    // Check if we found any positions
    if positions.is_empty() {
        return Err(anyhow!("No chess positions found in the study"));
    }
    
    Ok(StudyData {
        name: study_name,
        positions,
    })
}

fn create_pdf(study_data: &StudyData, filename: &str) -> Result<()> {
    let (doc, page1, layer1) = PdfDocument::new(&study_data.name, Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
    
    // Add fonts for text rendering
    let font = doc.add_builtin_font(printpdf::BuiltinFont::TimesRoman)?;
    let _font_bold = doc.add_builtin_font(printpdf::BuiltinFont::TimesBold)?;
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    
    let positions = &study_data.positions;
    let page_count = (positions.len() + BOARDS_PER_PAGE - 1) / BOARDS_PER_PAGE;
    
    for page in 0..page_count {
        if page > 0 {
            let (page_id, layer_id) = doc.add_page(Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
            current_layer = doc.get_page(page_id).get_layer(layer_id);
        }
        
        // Add study name centered before the first boards
        let study_name_y = PAGE_HEIGHT - 25.0; // 25mm from top
        let title_width_estimate = study_data.name.len() as f32 * 1.8; // Rough estimate
        let study_name_x = (PAGE_WIDTH - title_width_estimate) / 2.0; // Centered
        current_layer.use_text(&study_data.name, 18.0, Mm(study_name_x), Mm(study_name_y), &font);
        
        // Add page number centered at the bottom
        let page_info = format!("{}/{}", page + 1, page_count);
        let page_info_width_estimate = page_info.len() as f32 * 1.2;
        let page_info_x = (PAGE_WIDTH - page_info_width_estimate) / 2.0; // Centered
        let page_info_y = 10.0; // 10mm from bottom
        current_layer.use_text(page_info, 14.0, Mm(page_info_x), Mm(page_info_y), &font);
        
        // Add more space before the first row of boards for better layout
        let adjusted_margin_top = MARGIN_TOP + 30.0; // Add 30mm extra space at top
        
        let start_idx = page * BOARDS_PER_PAGE;
        let end_idx = std::cmp::min(start_idx + BOARDS_PER_PAGE, positions.len());
        
        for (i, pos) in positions[start_idx..end_idx].iter().enumerate() {
            let row = 2 - (i / BOARDS_PER_ROW); // Reverse row order: top=0, middle=1, bottom=2 becomes top=2, middle=1, bottom=0
            let col = i % BOARDS_PER_ROW;
            
            // Layout calculation with balanced margins and adjusted top margin
            let available_width = PAGE_WIDTH - MARGIN_LEFT - MARGIN_RIGHT;
            let available_height = PAGE_HEIGHT - adjusted_margin_top - MARGIN_BOTTOM;
            let col_width = available_width / BOARDS_PER_ROW as f32;
            let row_height = available_height / BOARDS_PER_COL as f32;
            
            let x = MARGIN_LEFT + (col as f32) * col_width + (col_width - BOARD_SIZE) / 2.0;
            // Simplify Y calculation and add explicit top spacing
            let top_spacing = 40.0; // 40mm from top of page
            let y = PAGE_HEIGHT - top_spacing - (row as f32) * row_height - (row_height - DESC_HEIGHT - BOARD_DESC_GAP) / 2.0 - BOARD_SIZE;
            
            draw_chess_board(&current_layer, x, y, pos, &font)?;
        }
    }
    
    doc.save(&mut std::io::BufWriter::new(std::fs::File::create(filename)?))?;
    Ok(())
}

fn draw_chess_board(layer: &PdfLayerReference, x: f32, y: f32, pos: &ChessPosition, font: &printpdf::IndirectFontRef) -> Result<()> {
    // Generate board image in RGB format for better Apple PDF viewer compatibility
    let (width, height, rgb_data) = generate_board_rgb_data(pos)?;
    
    // Create image from RGB data using DynamicImage for Apple PDF viewer compatibility
    use printpdf::image_crate::{DynamicImage, ImageBuffer, Rgb};
    let image_buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, rgb_data)
        .ok_or_else(|| anyhow!("Failed to create image buffer from RGB data"))?;
    let dynamic_image = DynamicImage::ImageRgb8(image_buffer);
    let image = printpdf::Image::from_dynamic_image(&dynamic_image);
    
    let scale_factor = 1.0;
    
    // PDF coordinates start from bottom-left, but our y is calculated from top
    let pdf_y = PAGE_HEIGHT - y - BOARD_SIZE; // Flip Y coordinate
    
    image.add_to_layer(layer.clone(), ImageTransform {
        translate_x: Some(Mm(x)),
        translate_y: Some(Mm(pdf_y)),
        scale_x: Some(scale_factor),
        scale_y: Some(scale_factor),
        ..Default::default()
    });
    
    // Draw coordinates and description
    draw_coordinates_and_description(layer, x, y, pos, font)?;
    
    Ok(())
}

fn parse_fen(fen_board: &str) -> [[char; 8]; 8] {
    let mut board = [[' '; 8]; 8];
    let ranks: Vec<&str> = fen_board.split('/').collect();
    
    for (rank_idx, rank) in ranks.iter().enumerate().take(8) {
        let mut file = 0;
        for ch in rank.chars() {
            if ch.is_ascii_digit() {
                let empty_count = ch.to_digit(10).unwrap_or(0) as usize;
                for _ in 0..empty_count {
                    if file < 8 {
                        board[rank_idx][file] = ' ';
                        file += 1;
                    }
                }
            } else if file < 8 {
                board[rank_idx][file] = ch;
                file += 1;
            }
        }
    }
    
    board
}

fn generate_board_rgb_data(pos: &ChessPosition) -> Result<(u32, u32, Vec<u8>)> {
    use tiny_skia::*;
    
    // Scale board image size to match the larger 75mm boards
    // 75mm boards need higher resolution for crisp PDF embedding
    let board_size_px = 600u32;  // Increased from 400px to 600px for larger boards
    let square_size_px = board_size_px / 8;
    let mut pixmap = Pixmap::new(board_size_px, board_size_px).unwrap();
    
    // Parse FEN
    let fen_parts: Vec<&str> = pos.fen.split(' ').collect();
    if fen_parts.is_empty() {
        return Ok((board_size_px, board_size_px, Vec::new()));
    }
    let board = parse_fen(fen_parts[0]);
    
    // Draw squares and pieces
    for rank in 0..8 {
        for file in 0..8 {
            let mut draw_rank = rank;
            let mut draw_file = file;
            
            // Flip board if black to move
            if pos.black_to_move {
                draw_rank = 7 - rank;
                draw_file = 7 - file;
            }
            
            let square_x = (file as u32) * square_size_px;
            let square_y = (rank as u32) * square_size_px;
            
            // Draw square background
            let is_light_square = (draw_rank + draw_file) % 2 == 0;
            let color = if is_light_square {
                Color::WHITE
            } else {
                Color::from_rgba8(221, 221, 221, 255) // Light gray
            };
            
            // Fill square
            let rect = Rect::from_xywh(square_x as f32, square_y as f32, square_size_px as f32, square_size_px as f32).unwrap();
            let mut paint = Paint::default();
            paint.set_color(color);
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            
            // Draw piece if present
            let piece = board[draw_rank][draw_file];
            if piece != ' ' {
                draw_piece_to_pixmap(&mut pixmap, piece, square_x as usize, square_y as usize, square_size_px as usize, is_light_square)?;
            }
        }
    }
    
    // Convert pixmap to RGB data for Apple PDF viewer compatibility
    let mut rgb_data = Vec::with_capacity((board_size_px * board_size_px * 3) as usize);
    let pixels = pixmap.pixels();
    
    for pixel in pixels {
        rgb_data.push(pixel.red());
        rgb_data.push(pixel.green());
        rgb_data.push(pixel.blue());
        // Skip alpha channel for RGB format
    }
    
    Ok((board_size_px, board_size_px, rgb_data))
}


fn draw_piece_to_pixmap(pixmap: &mut tiny_skia::Pixmap, piece: char, x: usize, y: usize, size: usize, is_light_square: bool) -> Result<()> {
    if let Some(png_data) = pieces::get_piece_png_data(piece) {
        // Load PNG data from embedded bytes
        let png_pixmap = tiny_skia::Pixmap::decode_png(png_data)
            .map_err(|e| anyhow!("PNG loading failed for piece '{}': {:?}", piece, e))?;
        
        // Create piece pixmap with appropriate background
        let mut piece_pixmap = tiny_skia::Pixmap::new(size as u32, size as u32).unwrap();
        let bg_color = if is_light_square {
            tiny_skia::Color::WHITE
        } else {
            tiny_skia::Color::from_rgba8(221, 221, 221, 255)
        };
        piece_pixmap.fill(bg_color);
        
        // Scale the PNG to fit the square size
        let scale_x = size as f32 / png_pixmap.width() as f32;
        let scale_y = size as f32 / png_pixmap.height() as f32;
        let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);
        
        // Draw the PNG piece onto the piece pixmap
        piece_pixmap.draw_pixmap(
            0, 0, 
            png_pixmap.as_ref(), 
            &tiny_skia::PixmapPaint::default(), 
            transform, 
            None
        );
        
        // Copy piece pixmap to board pixmap
        use tiny_skia::{PixmapPaint, Transform};
        pixmap.draw_pixmap(x as i32, y as i32, piece_pixmap.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
    }
    
    Ok(())
}

fn draw_coordinates_and_description(layer: &PdfLayerReference, x: f32, y: f32, pos: &ChessPosition, font: &printpdf::IndirectFontRef) -> Result<()> {
    use printpdf::*;
    
    // Use chapter name with position number for board descriptions
    let mut first_line = format!("{}. {}", pos.number, pos.description);
    let mut second_line = String::new();
    
    // Split at colon if present
    if let Some(colon_pos) = pos.description.find(':') {
        first_line = format!("{}. {}", pos.number, &pos.description[..colon_pos + 1]);
        second_line = pos.description[colon_pos + 1..].trim().to_string();
    }
    
    // Position text below the board with proper gap
    let text_y = y + BOARD_SIZE + BOARD_DESC_GAP; // Below the board with gap
    let pdf_text_y = PAGE_HEIGHT - text_y; // Flip Y coordinate for PDF
    
    // Add first line of text
    layer.use_text(first_line, 11.0, Mm(x), Mm(pdf_text_y), font);
    
    // Add second line if it exists
    if !second_line.is_empty() {
        let second_line_y = pdf_text_y - 5.0; // 5mm below first line
        layer.use_text(second_line, 11.0, Mm(x), Mm(second_line_y), font);
    }
    
    // Add chess board coordinates (a1-h8)
    let square_size = BOARD_SIZE / 11.5;
    
    // Add file coordinates (a-h) at the bottom

    if pos.black_to_move {

        for i in 0..8 {
            let file_char = (b'h' - i) as char;
            let coord_x = x + (i as f32 * square_size) + (square_size / 2.0) - 1.0; // Center in square
            let coord_y = PAGE_HEIGHT - (y + BOARD_SIZE + 4.0) + 1.5 ; // Just below board
            layer.use_text(file_char.to_string(), 6.0, Mm(coord_x), Mm(coord_y), font);
        }
    }
    else {
                for i in 0..8 {
            let file_char = (b'a' + i) as char;
            let coord_x = x + (i as f32 * square_size) + (square_size / 2.0) - 1.0; // Center in square
            let coord_y = PAGE_HEIGHT - (y + BOARD_SIZE + 4.0) + 1.5 ; // Just below board
            layer.use_text(file_char.to_string(), 6.0, Mm(coord_x), Mm(coord_y), font);
        }
    }
    
    // Add rank coordinates (1-8) on the left
    if pos.black_to_move {
        for i in 0..8 {
            let rank_char = (b'0' + 1 + i) as char; 
            let coord_x = x - 2.5 ; // To the left of board
            let coord_y = PAGE_HEIGHT - 25.0 - (y + (i as f32 * square_size) + (square_size / 4.0) + 1.0); // Center in square
            layer.use_text(rank_char.to_string(), 6.0, Mm(coord_x), Mm(coord_y), font);
        }
    }
    else {
        for i in 0..8 {
            let rank_char = (b'1' + (7 - i)) as char; 
            let coord_x = x - 2.5 ; // To the left of board
            let coord_y = PAGE_HEIGHT - 25.0 - (y + (i as f32 * square_size) + (square_size / 4.0) + 1.0); // Center in square
            layer.use_text(rank_char.to_string(), 6.0, Mm(coord_x), Mm(coord_y), font);
        }
    }
    
    Ok(())
}
