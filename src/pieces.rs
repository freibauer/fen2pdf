// Chess piece PNG data as literal byte arrays
// This eliminates the need for external asset files

// White pieces (using include_bytes! for embedded PNG data)
pub const WK_PNG: &[u8] = include_bytes!("../assets/png/wK.png");
pub const WQ_PNG: &[u8] = include_bytes!("../assets/png/wQ.png");
pub const WR_PNG: &[u8] = include_bytes!("../assets/png/wR.png");
pub const WB_PNG: &[u8] = include_bytes!("../assets/png/wB.png");
pub const WN_PNG: &[u8] = include_bytes!("../assets/png/wN.png");
pub const WP_PNG: &[u8] = include_bytes!("../assets/png/wP.png");

// Black pieces  
pub const BK_PNG: &[u8] = include_bytes!("../assets/png/bK.png");
pub const BQ_PNG: &[u8] = include_bytes!("../assets/png/bQ.png");
pub const BR_PNG: &[u8] = include_bytes!("../assets/png/bR.png");
pub const BB_PNG: &[u8] = include_bytes!("../assets/png/bB.png");
pub const BN_PNG: &[u8] = include_bytes!("../assets/png/bN.png");
pub const BP_PNG: &[u8] = include_bytes!("../assets/png/bP.png");

// Function to get piece PNG data by character
pub fn get_piece_png_data(piece: char) -> Option<&'static [u8]> {
    match piece {
        'K' => Some(WK_PNG),
        'Q' => Some(WQ_PNG),
        'R' => Some(WR_PNG),
        'B' => Some(WB_PNG),
        'N' => Some(WN_PNG),
        'P' => Some(WP_PNG),
        'k' => Some(BK_PNG),
        'q' => Some(BQ_PNG),
        'r' => Some(BR_PNG),
        'b' => Some(BB_PNG),
        'n' => Some(BN_PNG),
        'p' => Some(BP_PNG),
        _ => None,
    }
}

// Example of how to convert PNG to literal array (for WK_PNG):
// 1. Run: xxd -i assets/png/wK.png
// 2. Replace: unsigned char assets_png_wK_png[] = { with pub const WK_PNG: &[u8] = &[
// 3. Replace: }; with ];
// 
// This would result in a file with ~25,000+ lines of hex data for all 12 pieces.
// For practical development, include_bytes! is more manageable.