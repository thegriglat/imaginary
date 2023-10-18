use std::io::Cursor;

use actix_web::web::Bytes;

use crate::query::{Format, QueryParams};

pub struct Converter {
    image: image::DynamicImage,
    params: QueryParams,
}

impl Converter {
    pub fn new(bytes: &Bytes, params: QueryParams) -> Result<Self, String> {
        match image::load_from_memory(bytes) {
            Ok(image) => Ok(Self { image, params }),
            Err(_) => Err(String::from("Failed to load image")),
        }
    }

    pub fn guess_format(bytes: &Bytes) -> &str {
        match image::guess_format(bytes) {
            Ok(image::ImageFormat::Jpeg) => "image/jpeg",
            Ok(image::ImageFormat::Png) => "image/png",
            _ => "image/jpeg",
        }
    }

    fn flip_x(&mut self, flip: Option<bool>) -> &mut Self {
        match flip {
            Some(true) => {
                self.image = self.image.fliph();
            }
            _ => {}
        }
        self
    }

    fn flip_y(&mut self, flip: Option<bool>) -> &mut Self {
        match flip {
            Some(true) => {
                self.image = self.image.flipv();
            }
            _ => {}
        }
        self
    }

    fn blur(&mut self, blur: Option<f32>) -> &mut Self {
        match blur {
            Some(value) => {
                println!("blur: {}", value);
                self.image = self.image.blur(value);
            }
            _ => {}
        }
        self
    }

    fn crop(&mut self, crop: Option<String>) -> &mut Self {
        match crop {
            Some(value) => {
                println!("crop: {}", value);
                let mut parts = value.split(',');
                let x = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
                let y = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
                let width = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
                let height = parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
                self.image = self.image.crop_imm(x, y, width, height);
            }
            _ => {}
        }
        self
    }

    fn grayscale(&mut self, grayscale: Option<bool>) -> &mut Self {
        match grayscale {
            Some(true) => {
                self.image = self.image.grayscale();
            }
            _ => {}
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
                Format::JPEG(quality) => {
                    match self.image.write_to(
                        &mut Cursor::new(&mut bytes),
                        image::ImageOutputFormat::Jpeg(quality),
                    ) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                Format::PNG => {
                    match self
                        .image
                        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                    {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
            },
            _ => {}
        }
        Ok(bytes.into())
    }

    pub fn result(&mut self) -> Result<Bytes, String> {
        let params = self.params.clone();
        self.flip_x(params.flip_x)
            .flip_y(params.flip_y)
            .blur(params.blur)
            .crop(params.crop)
            .grayscale(params.grayscale)
            .rotate(params.rotate)
            .bytes(params.format)
    }
}
