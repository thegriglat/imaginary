use std::io::Cursor;

use actix_web::web::Bytes;

pub struct Converter {
    image: image::DynamicImage,
}

impl Converter {
    pub fn new(bytes: &Bytes) -> Result<Self, &str> {
        match image::load_from_memory(bytes) {
            Ok(image) => {
                println!("Image loaded");
                println!("HxW: {}x{}", image.height(), image.width());
                Ok(Self { image })
            }
            Err(_) => {
                println!("Error: Failed to load image");
                Err("Failed to load image")
            }
        }
    }

    pub fn flip_x(&mut self) -> &Self {
        self.image = self.image.flipv();
        println!("HxW: {}x{}", self.image.height(), self.image.width());
        self
    }

    pub fn bytes(&self) -> Bytes {
        let mut bytes: Vec<u8> = Vec::new();
        match self.image.write_to(
            &mut Cursor::new(&mut bytes),
            image::ImageOutputFormat::Jpeg(90),
        ) {
            Ok(_) => println!("Image converted"),
            Err(_) => println!("Error: Failed to convert image"),
        }
        bytes.into()
    }
}
