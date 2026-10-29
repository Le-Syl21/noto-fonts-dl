//! The embedded bundle decompresses into the enabled fonts.

#[test]
#[cfg(feature = "ar")]
fn bundle_decompresses() {
    let fonts = noto_fonts_dl::load_fonts();
    assert!(
        fonts
            .iter()
            .any(|(name, data)| name == "ar" && data.len() > 10_000),
        "{:?}",
        fonts.iter().map(|(n, d)| (n, d.len())).collect::<Vec<_>>()
    );
}
