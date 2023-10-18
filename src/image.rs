use std::io::Cursor;

use actix_web::web::Bytes;

use crate::api::QueryParams;

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

    fn bytes(&self) -> Result<Bytes, String> {
        let mut bytes: Vec<u8> = Vec::new();
        match self.image.write_to(
            &mut Cursor::new(&mut bytes),
            image::ImageOutputFormat::Jpeg(90),
        ) {
            Ok(_) => {}
            Err(_) => return Err(String::from("Error: Failed to convert image")),
        }
        Ok(bytes.into())
    }

    pub fn result(&mut self) -> Result<Bytes, String> {
        let params = self.params.clone();
        self.flip_x(params.flip_x).flip_y(params.flip_y).bytes()
    }
}
