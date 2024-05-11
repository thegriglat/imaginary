use crate::query::{Format, QueryParams, Scale};
use axum::body::Bytes;
use std::io::Cursor;

pub struct Converter {
    image: image::DynamicImage,
    params: QueryParams,
}

impl Converter {
    pub fn new(bytes: &[u8], params: QueryParams) -> Result<Self, String> {
        match image::load_from_memory(bytes) {
            Ok(image) => Ok(Self { image, params }),
            Err(err) => Err(format!("Failed to load image: {}", err)),
        }
    }

    fn flip_x(&mut self, flip: Option<bool>) -> &mut Self {
        if let Some(true) = flip {
            self.image = self.image.fliph();
        }
        self
    }

    fn flip_y(&mut self, flip: Option<bool>) -> &mut Self {
        if let Some(true) = flip {
            self.image = self.image.flipv();
        }
        self
    }

    fn blur(&mut self, blur: Option<f32>) -> &mut Self {
        if let Some(value) = blur {
            self.image = self.image.blur(value);
        }
        self
    }

    fn scale(&mut self, scale: Option<Scale>) -> &mut Self {
        if let Some(value) = scale {
            self.image = self
                .image
                .resize(value.width, value.height, value.algorithm);
        }
        self
    }

    fn crop(&mut self, crop: Option<String>) -> &mut Self {
        if let Some(value) = crop {
            let mut parts = value.split(',');
            let x = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
            let y = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
            let width = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
            let height = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
            self.image = self.image.crop_imm(x, y, width, height);
        }
        self
    }

    fn grayscale(&mut self, grayscale: Option<bool>) -> &mut Self {
        if let Some(true) = grayscale {
            self.image = self.image.grayscale();
        }
        self
    }

    fn rotate(&mut self, rotate: Option<u32>) -> &mut Self {
        match rotate {
            Some(90) => {
                self.image = self.image.rotate90();
            }
            Some(180) => {
                self.image = self.image.rotate180();
            }
            Some(270) => {
                self.image = self.image.rotate270();
            }
            _ => {}
        }
        self
    }

    fn bytes(&self, format: Option<Format>) -> Result<Bytes, String> {
        let mut bytes: Vec<u8> = Vec::new();
        match format {
            Some(value) => match value {
                Format::Jpeg(quality) => {
                    if self
                        .image
                        .write_to(
                            &mut Cursor::new(&mut bytes),
                            image::ImageOutputFormat::Jpeg(quality),
                        )
                        .is_ok()
                    {}
                }
                Format::Png => {
                    if self
                        .image
                        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                        .is_ok()
                    {}
                }
                Format::WebP => {
                    if self
                        .image
                        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::WebP)
                        .is_ok()
                    {}
                }
                Format::Avif => {
                    if self
                        .image
                        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Avif)
                        .is_ok()
                    {}
                }
            },
            _ => {
                if self
                    .image
                    .write_to(
                        &mut Cursor::new(&mut bytes),
                        image::ImageOutputFormat::Jpeg(95),
                    )
                    .is_ok()
                {}
            }
        }
        Ok(bytes.into())
    }

    pub fn result(&mut self) -> Result<Bytes, String> {
        let params = self.params.clone();
        self.scale(params.scale)
            .crop(params.crop)
            .flip_x(params.flip_x)
            .flip_y(params.flip_y)
            .rotate(params.rotate)
            .grayscale(params.grayscale)
            .blur(params.blur)
            .bytes(params.format)
    }
}

pub fn guess_mime_type(bytes: &Bytes) -> Result<&str, &str> {
    match image::guess_format(bytes) {
        Ok(value) => Ok(value.to_mime_type()),
        _ => Err("Unsupported format"),
    }
}
