use std::collections::HashMap;
use std::io::{Read, Write};
use xz2::write::XzEncoder;

const FONTS_JSON: &str = include_str!("fonts.json");

/// Base URL for Google Fonts CSS API — we parse the CSS to get the actual .ttf URL
const GOOGLE_FONTS_CSS: &str = "https://fonts.googleapis.com/css2?family=";

/// Fallback: direct download from notofonts GitHub repo
const NOTO_GITHUB_BASE: &str = "https://github.com/notofonts/noto-fonts/raw/main/unhinted/slim-variable-ttf/";
const NOTO_CJK_GITHUB_BASE: &str = "https://github.com/notofonts/noto-cjk/raw/main/Sans/SubsetOTF/";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bundle_path = std::path::Path::new(&out_dir).join("fonts.bundle.xz");

    // Parse fonts.json: feature_name -> font_filename
    let font_map: HashMap<String, String> = serde_json::from_str(FONTS_JSON)
        .expect("Failed to parse fonts.json");

    // Collect fonts for enabled features
    let mut fonts_data: Vec<(String, Vec<u8>)> = Vec::new();

    for (feature, filename) in &font_map {
        let env_key = format!("CARGO_FEATURE_{}", feature.to_uppercase());
        if std::env::var(&env_key).is_err() {
            continue;
        }

        let cache_path = std::path::Path::new(&out_dir).join(filename);
        let data = if cache_path.exists() {
            eprintln!("cargo:warning=Using cached: {}", filename);
            std::fs::read(&cache_path).unwrap()
        } else {
            eprintln!("cargo:warning=Downloading font: {}", filename);
            let data = download_font(filename);
            std::fs::write(&cache_path, &data).unwrap();
            eprintln!("cargo:warning=Downloaded {} ({} bytes)", filename, data.len());
            data
        };

        fonts_data.push((feature.clone(), data));
    }

    if fonts_data.is_empty() {
        std::fs::write(&bundle_path, &[]).unwrap();
        return;
    }

    // Sort for deterministic output
    fonts_data.sort_by(|a, b| a.0.cmp(&b.0));

    // Pack into minimal format:
    //   u32le: number of fonts
    //   For each font:
    //     u32le: name length
    //     [u8]: name bytes (feature/language code)
    //     u32le: data length
    //     [u8]: font data
    let mut raw = Vec::new();
    raw.extend_from_slice(&(fonts_data.len() as u32).to_le_bytes());
    for (name, data) in &fonts_data {
        let name_bytes = name.as_bytes();
        raw.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
        raw.extend_from_slice(name_bytes);
        raw.extend_from_slice(&(data.len() as u32).to_le_bytes());
        raw.extend_from_slice(data);
    }

    eprintln!("cargo:warning=Raw bundle size: {} bytes", raw.len());

    // Compress with XZ
    let mut encoder = XzEncoder::new(Vec::new(), 6);
    encoder.write_all(&raw).unwrap();
    let compressed = encoder.finish().unwrap();

    eprintln!("cargo:warning=Compressed bundle size: {} bytes (ratio: {:.0}%)",
        compressed.len(), compressed.len() as f64 / raw.len() as f64 * 100.0);

    std::fs::write(&bundle_path, &compressed).unwrap();
}

fn download_font(filename: &str) -> Vec<u8> {
    // CJK fonts need special handling — use SubsetOTF from noto-cjk repo
    if filename.contains("CJK") {
        return download_cjk_font(filename);
    }

    // Color Emoji — hosted in the noto-emoji repo, not noto-fonts
    if filename.contains("Emoji") {
        let url = "https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf";
        return download_url(url);
    }

    // Try Google Fonts CSS API first (gives optimized/subset fonts)
    let font_name = filename
        .replace("-VF.ttf", "")
        .replace("-Regular.ttf", "")
        .replace("NotoLooped", "Noto+Looped+")
        .replace("NotoNaskh", "Noto+Naskh+")
        .replace("NotoNastaliq", "Noto+Nastaliq+")
        .replace("NotoSerif", "Noto+Serif+")
        .replace("NotoSans", "Noto+Sans+");

    let css_url = format!("{}{}&display=swap", GOOGLE_FONTS_CSS, font_name);
    if let Ok(data) = try_google_fonts(&css_url) {
        return data;
    }

    // Fallback: download directly from notofonts GitHub
    let url = format!("{}{}", NOTO_GITHUB_BASE, filename);
    download_url(&url)
}

fn download_cjk_font(_filename: &str) -> Vec<u8> {
    // Map CJK-Regular.ttc references to SubsetOTF individual files
    // The fonts.json maps ja/ko/zh/zh_tw all to NotoSansCJK-Regular.ttc
    // but we download region-specific SubsetOTF instead
    let feature = std::env::vars()
        .filter(|(k, _)| k.starts_with("CARGO_FEATURE_"))
        .find(|(k, _)| {
            let f = k.replace("CARGO_FEATURE_", "").to_lowercase();
            matches!(f.as_str(), "ja" | "ko" | "zh" | "zh_tw")
        });

    let (region, otf_name) = match feature {
        Some((k, _)) => {
            let f = k.replace("CARGO_FEATURE_", "").to_lowercase();
            match f.as_str() {
                "ja" => ("JP", "NotoSansJP-Regular.otf"),
                "ko" => ("KR", "NotoSansKR-Regular.otf"),
                "zh_tw" => ("TC", "NotoSansTC-Regular.otf"),
                _ => ("SC", "NotoSansSC-Regular.otf"),
            }
        }
        None => ("SC", "NotoSansSC-Regular.otf"),
    };

    let url = format!("{}{}/{}", NOTO_CJK_GITHUB_BASE, region, otf_name);
    download_url(&url)
}

fn try_google_fonts(css_url: &str) -> Result<Vec<u8>, String> {
    let resp = ureq::get(css_url)
        .set("User-Agent", "Mozilla/5.0")
        .call()
        .map_err(|e| format!("CSS fetch failed: {}", e))?;

    let css = resp.into_string().map_err(|e| format!("CSS read failed: {}", e))?;

    // Extract first .ttf URL from CSS
    let url = css.split("url(")
        .nth(1)
        .and_then(|s| s.split(')').next())
        .ok_or_else(|| "No font URL in CSS".to_string())?;

    Ok(download_url(url))
}

fn download_url(url: &str) -> Vec<u8> {
    eprintln!("cargo:warning=  GET {}", url);
    let resp = ureq::get(url)
        .call()
        .unwrap_or_else(|e| panic!("Failed to download {}: {}", url, e));
    let mut data = Vec::new();
    resp.into_reader().read_to_end(&mut data)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", url, e));
    data
}
