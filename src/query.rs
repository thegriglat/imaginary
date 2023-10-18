use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub enum Format {
    JPEG(u8),
    PNG,
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Format, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let str = s.as_str();
        if str == "png" {
            return Ok(Format::PNG);
        }

        if str == "jpeg" {
            return Ok(Format::JPEG(95));
        }

        let regex = Regex::new(r"jpeg:(\d+)").unwrap();
        match regex.captures(str) {
            Some(caps) => {
                let quality: u8 = caps.get(1).unwrap().as_str().parse().unwrap_or(95).min(100);
                return Ok(Format::JPEG(quality));
            }
            None => {}
        }

        return Err(serde::de::Error::custom(
            "expected png or jpeg:<quality> as format",
        ));
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QueryParams {
    pub url: String,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub grayscale: Option<bool>,
    pub blur: Option<f32>,
    pub crop: Option<String>,
    pub rotate: Option<u32>,
    pub format: Option<Format>,
}
