use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub enum Format {
    Jpeg(u8),
    Png,
    WebP,
    Avif,
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Format, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let str = s.as_str();

        if str == "png" {
            return Ok(Format::Png);
        }

        if str == "jpeg" {
            return Ok(Format::Jpeg(95));
        }

        if str == "webp" {
            return Ok(Format::WebP);
        }

        if str == "avif" {
            return Ok(Format::Avif);
        }

        let regex = Regex::new(r"jpeg:(\d+)").unwrap();

        if let Some(caps) = regex.captures(str) {
            let quality: u8 = caps.get(1).unwrap().as_str().parse().unwrap_or(95).min(100);
            return Ok(Format::Jpeg(quality));
        }

        Err(serde::de::Error::custom(
            "expected png or jpeg:<quality> as format",
        ))
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
