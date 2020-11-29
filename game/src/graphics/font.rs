pub struct Font {
    font: fontdue::Font,
}

impl Font {
    pub fn load(bytes: &[u8]) -> Self {
        let font = fontdue::Font::from_bytes(&bytes[..], fontdue::FontSettings::default()).unwrap();
        Self { font }
    }
}
