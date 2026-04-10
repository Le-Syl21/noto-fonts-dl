use eframe::egui;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "noto-fonts-dl POC",
        options,
        Box::new(|cc| {
            // Load fonts from our crate
            let fonts_data = noto_fonts_dl::load_fonts();
            println!("Loaded {} fonts:", fonts_data.len());
            for (name, data) in fonts_data {
                println!("  {} — {} bytes", name, data.len());
            }

            // Configure egui fonts
            let mut fonts = egui::FontDefinitions::default();
            for (name, data) in fonts_data {
                fonts.font_data.insert(
                    name.clone(),
                    Arc::new(egui::FontData::from_owned(data.clone())),
                );
                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .push(name.clone());
            }
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(HelloApp))
        }),
    )
}

struct HelloApp;

impl eframe::App for HelloApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("noto-fonts-dl POC");
            ui.separator();
            ui.label("English: Hello World!");
            ui.label("Korean: 안녕하세요 세계!");
            ui.label("Hindi: नमस्ते दुनिया!");
            ui.label("Arabic: مرحبا بالعالم!");
            ui.label("Bengali: হ্যালো বিশ্ব!");
            ui.label("Japanese: こんにちは世界！");
            ui.label("Chinese: 你好世界！");
            ui.separator();
            ui.heading("Emoji Test");
            ui.label("Faces: 😀😂🥹😍🤩🥳😎🤔😴🫠");
            ui.label("Hands: 👋🤝👏🙌🤞✌️👍👎🫶🤙");
            ui.label("Animals: 🐶🐱🐻🦊🐼🐨🦁🐸🦋🐝");
            ui.label("Food: 🍕🍔🌮🍣🍩🍪🍰🧁🍫🥐");
            ui.label("Travel: 🚀✈️🚂🏔️🌋🏖️🗼🎡⛵🛸");
            ui.label("Symbols: ❤️🧡💛💚💙💜🖤🤍💯✨");
            ui.label("Flags: 🇫🇷🇯🇵🇰🇷🇨🇳🇮🇳🇧🇷🇺🇸🇬🇧🇩🇪🇪🇸");
            ui.separator();
            ui.heading("Symbols Test (zsym — N&B)");
            ui.label("Arrows: ← → ↑ ↓ ↔ ↕ ⇐ ⇒ ⇑ ⇓");
            ui.label("Math: ∞ ∑ √ ∫ ≈ ≠ ≤ ≥ ± ÷");
            ui.label("Misc: ☀ ☁ ☂ ☃ ★ ☆ ♠ ♣ ♥ ♦");
            ui.label("Music: ♩ ♪ ♫ ♬ ♭ ♮ ♯");
            ui.label("Dingbats: ✓ ✗ ✠ ✡ ✢ ✣ ✤ ✥ ✦ ✧");
            ui.label("Zodiac: ♈ ♉ ♊ ♋ ♌ ♍ ♎ ♏ ♐ ♑ ♒ ♓");
            ui.label("Chess: ♔ ♕ ♖ ♗ ♘ ♙ ♚ ♛ ♜ ♝ ♞ ♟");
            ui.label("Braille: ⠁ ⠃ ⠉ ⠙ ⠑ ⠋ ⠛ ⠝ ⠏ ⠟");
        });
    }
}
