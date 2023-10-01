pub struct TextureData {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl TextureData {
    pub fn from_bytes(colors: &[u8], width: usize, height: usize) -> Self {
        Self {
            data: colors.to_vec(),
            width,
            height,
        }
    }

    pub fn load(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let data = image.to_rgba8().as_raw().to_vec();

        TextureData {
            data,
            width,
            height,
        }
    }
}
