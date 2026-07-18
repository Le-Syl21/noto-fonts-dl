//! # noto-fonts-dl
//!
//! Download Noto fonts at build time, compress them into a single XZ bundle,
//! and decompress at runtime. Feature-gated per language.
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! noto-fonts-dl = { version = "0.1", features = ["ko", "hi"] }
//! ```
//!
//! ```no_run
//! let fonts = noto_fonts_dl::load_fonts();
//! for (name, data) in &fonts {
//!     println!("{}: {} bytes", name, data.len());
//! }
//! ```

use std::io::Read;
use std::sync::OnceLock;
use xz2::read::XzDecoder;

/// The compressed font bundle, embedded at compile time
const FONT_BUNDLE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fonts.bundle.xz"));

/// A loaded font: (name, font data)
pub type Font = (String, Vec<u8>);

static FONTS: OnceLock<Vec<Font>> = OnceLock::new();

/// Load and decompress all fonts from the embedded bundle.
/// Cached after first call — subsequent calls return instantly.
pub fn load_fonts() -> &'static Vec<Font> {
    FONTS.get_or_init(|| {
        if FONT_BUNDLE.is_empty() {
            return Vec::new();
        }

        // Decompress XZ
        let mut decoder = XzDecoder::new(FONT_BUNDLE);
        let mut raw = Vec::new();
        decoder
            .read_to_end(&mut raw)
            .expect("Failed to decompress font bundle");

        // Parse our minimal format
        let mut pos = 0;
        let count = u32::from_le_bytes(raw[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;

        let mut fonts = Vec::with_capacity(count);
        for _ in 0..count {
            let name_len = u32::from_le_bytes(raw[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;
            let name = String::from_utf8(raw[pos..pos + name_len].to_vec()).unwrap();
            pos += name_len;

            let data_len = u32::from_le_bytes(raw[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;
            let data = raw[pos..pos + data_len].to_vec();
            pos += data_len;

            fonts.push((name, data));
        }

        fonts
    })
}
