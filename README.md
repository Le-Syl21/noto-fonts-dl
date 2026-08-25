# noto-fonts-dl

[![Crates.io](https://img.shields.io/crates/v/noto-fonts-dl.svg)](https://crates.io/crates/noto-fonts-dl)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Discord](https://img.shields.io/badge/Discord-Le--Syl21%20Tools-5865F2?logo=discord&logoColor=white)](https://discord.gg/T37DYHmt2j)

Download and embed [Google Noto fonts](https://fonts.google.com/noto) at build time with Cargo feature flags. **134 languages/scripts supported** — just enable the ones you need.

- Fonts are downloaded from Google Fonts at compile time (cached for subsequent builds)
- All selected fonts are compressed together into a single XZ bundle
- Decompressed in memory at runtime (~0.5s for the largest bundles)
- Zero font files stored in the crate itself
- Language-to-font mapping derived from [Android's official Noto configuration](https://github.com/notofonts/noto-fonts)

## Community & support

Questions, bug reports, beta testing, or just want to chat? Join the Discord:

[![Discord](https://img.shields.io/badge/Discord-Le--Syl21%20Tools-5865F2?logo=discord&logoColor=white)](https://discord.gg/T37DYHmt2j)

## Quick Start

```toml
[dependencies]
noto-fonts-dl = { version = "0.1", features = ["ko", "ar", "hi"] }
```

```rust
let fonts = noto_fonts_dl::load_fonts();
for (lang, data) in fonts {
    println!("{}: {} bytes", lang, data.len());
}
```

## Integration with egui

```rust
use eframe::egui;
use std::sync::Arc;

// In your eframe App setup:
let fonts_data = noto_fonts_dl::load_fonts();

let mut fonts = egui::FontDefinitions::default();
for (name, data) in fonts_data {
    fonts.font_data.insert(
        name.clone(),
        Arc::new(egui::FontData::from_owned(data.clone())),
    );
    // Add as fallback after the default font
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push(name.clone());
}
ctx.set_fonts(fonts);
```

## Integration with other frameworks

`load_fonts()` returns `&Vec<(String, Vec<u8>)>` — a list of `(language_code, raw_ttf_bytes)` pairs. Use the raw bytes with any font loading API:

### iced

```rust
let fonts = noto_fonts_dl::load_fonts();
for (_, data) in fonts {
    iced::font::load(data.clone());
}
```

### wgpu-text / glyph-brush

```rust
let fonts = noto_fonts_dl::load_fonts();
for (_, data) in fonts {
    let font = ab_glyph::FontArc::try_from_vec(data.clone()).unwrap();
    glyph_brush.add_font(font);
}
```

### Any framework

The font data is standard TrueType/OpenType — it works with any library that accepts `&[u8]` font data: `fontdue`, `rusttype`, `cosmic-text`, `swash`, `ab_glyph`, etc.

## Available Features

### Common Languages

| Feature | Language | Font |
|---------|----------|------|
| `ar` | Arabic | Noto Naskh Arabic |
| `bn` | Bengali | Noto Sans Bengali |
| `hi` | Hindi | Noto Sans Devanagari |
| `ja` | Japanese | Noto Sans CJK JP |
| `ko` | Korean | Noto Sans CJK KR |
| `zh` | Chinese (Simplified) | Noto Sans CJK SC |
| `zh_tw` | Chinese (Traditional) | Noto Sans CJK TC |
| `he` | Hebrew | Noto Serif Hebrew |
| `th` | Thai | Noto Looped Thai |
| `ka` | Georgian | Noto Sans Georgian |
| `hy` | Armenian | Noto Sans Armenian |
| `am` | Amharic (Ethiopic) | Noto Sans Ethiopic |
| `ta` | Tamil | Noto Sans Tamil |
| `te` | Telugu | Noto Sans Telugu |
| `kn` | Kannada | Noto Sans Kannada |
| `ml` | Malayalam | Noto Sans Malayalam |
| `gu` | Gujarati | Noto Sans Gujarati |
| `pa` | Punjabi | Noto Sans Gurmukhi |
| `or` | Odia | Noto Sans Oriya |
| `si` | Sinhala | Noto Sans Sinhala |
| `km` | Khmer | Noto Sans Khmer |
| `my` | Myanmar | Noto Sans Myanmar |
| `lo` | Lao | Noto Looped Lao |
| `bo` | Tibetan | Noto Serif Tibetan |
| `ur` | Urdu | Noto Nastaliq Urdu |

### Symbols & Emoji

| Feature | Description | Font |
|---------|-------------|------|
| `zsye` | Color Emoji | Noto Color Emoji |
| `zsym` | Symbols (monochrome) | Noto Sans Symbols |
| `zsym2` | Symbols 2 (monochrome) | Noto Sans Symbols 2 |

The feature codes `zsye` and `zsym` come from [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) script codes. **Zsye** stands for "Symbols (emoji variant)" — the standard script subtag for color emoji presentation. **Zsym** stands for "Symbols" — for monochrome symbol glyphs. These are not language codes but script codes used in [BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) to distinguish emoji rendering from plain text symbols.

### Convenience Groups

| Feature | Includes |
|---------|----------|
| `all-common` | ar, bn, hi, ja, ko, zh, zh_tw, he, th |
| `all-indic` | hi, bn, ta, te, kn, ml, gu, pa, or, si |
| `all-cjk` | ja, ko, zh, zh_tw |

### 100+ Additional Scripts

The crate supports 134 scripts total, including historical and rare writing systems (Cuneiform, Egyptian Hieroglyphs, Runic, Gothic, etc.). See `Cargo.toml` for the full list.

## Color Emoji Limitations

The `zsye` feature downloads **Noto Color Emoji**, a CBDT/CBLC bitmap font (pre-rendered color images embedded at fixed sizes). Most Rust GUI text rasterizers — including egui's `epaint`/`ab_glyph` pipeline — only support **outline** fonts (vector glyphs defined by Bézier curves). They cannot render bitmap color tables, so color emoji will appear as blank squares or monochrome fallbacks.

The `zsym` feature (Noto Sans Symbols) works correctly everywhere since it uses standard vector outlines. It covers arrows, math symbols, dingbats, zodiac signs, chess pieces, musical notation, braille, and other monochrome Unicode symbols.

`zsym2` (Noto Sans Symbols 2) is a **different set, not a newer version** — the two barely overlap, so pick by the glyphs you need rather than by the number. Symbols 2 carries Miscellaneous Symbols and Miscellaneous Technical: ballot boxes (`☐` `☑`), the power symbol (`⏻`), keyboard and clock glyphs, and a much wider dingbat range — 2655 codepoints against 840. Several of those are missing from the fonts a GUI toolkit bundles by default, which is usually why you end up needing it.

### Why this happens

| Font | Format | egui | iced | cosmic-text |
|------|--------|------|------|-------------|
| Noto Color Emoji (`zsye`) | CBDT/CBLC bitmap | Not supported | Not supported | Not supported |
| Noto Sans Symbols (`zsym`) | TrueType outlines | Works | Works | Works |
| Noto Sans Symbols 2 (`zsym2`) | TrueType outlines | Works | Works | Works |

The color emoji spec (CBDT/CBLC) stores pre-rendered PNG bitmaps at specific sizes. Rendering them requires extracting the bitmap from the font tables and blitting it as a texture — standard glyph rasterizers (ab_glyph, fontdue, rusttype, swash) skip these tables entirely.

### Alternatives for color emoji rendering

- **[emojis](https://crates.io/crates/emojis) + custom texture atlas**: Render emoji as images/sprites instead of font glyphs. Load Noto Color Emoji PNGs from [noto-emoji](https://github.com/googlefonts/noto-emoji/tree/main/png) and display them as `egui::Image` inline.
- **[cosmic-text](https://crates.io/crates/cosmic-text) + [swash](https://crates.io/crates/swash)**: `swash` has experimental CBDT rasterization support — the most promising Rust-native path for color emoji rendering.
- **Web-based renderers**: If targeting WebAssembly, the browser's native text renderer handles color emoji natively via `<canvas>` or DOM text.
- **[resvg](https://crates.io/crates/resvg)**: For SVG-based emoji fonts (like Noto Emoji SVG), `resvg` can rasterize individual glyphs to images.
- **System font fallback**: On desktop, the OS text stack (DirectWrite, CoreText, HarfBuzz+FreeType) supports color emoji natively. Frameworks that delegate to the system shaper (e.g., GTK/Qt bindings) will render them correctly.

## How It Works

1. **Build time**: `build.rs` reads `fonts.json` (a mapping from language codes to Noto font filenames, derived from Android's official configuration)
2. **Download**: For each enabled feature, downloads the corresponding font from Google Fonts (with fallback to the notofonts GitHub repository for CJK)
3. **Cache**: Downloaded fonts are cached in `OUT_DIR` — subsequent builds don't re-download
4. **Compress**: All fonts are packed into a minimal binary format and compressed with XZ (~50-60% compression ratio)
5. **Embed**: The compressed bundle is embedded in the binary via `include_bytes!`
6. **Runtime**: `load_fonts()` decompresses the bundle on first call (~0.5s) and returns the font data. Cached via `OnceLock` for instant subsequent calls.

## Bundle Sizes

| Features | Raw | Compressed | Binary overhead |
|----------|-----|------------|----------------|
| `hi` | 220 KB | ~100 KB | ~100 KB |
| `ar` | 200 KB | ~90 KB | ~90 KB |
| `ko` | 4.6 MB | ~3 MB | ~3 MB |
| `ko,hi` | 4.8 MB | ~3.1 MB | ~3.1 MB |
| `all-cjk` | ~23 MB | ~13 MB | ~13 MB |
| `all-common` | ~28 MB | ~15 MB | ~15 MB |

## Requirements

- Internet connection at build time (first build only — fonts are cached)
- No runtime dependencies beyond `xz2` for decompression

## License

MIT OR Apache-2.0

Noto fonts are licensed under the [SIL Open Font License](https://scripts.sil.org/OFL).
