use regex::Regex;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub enum Format {
    Jpeg(u8),
    Png,
    WebP,
    Avif,
}

#[derive(Clone, Debug)]
pub struct Scale {
    pub width: u32,
    pub height: u32,
    pub algorithm: image::imageops::FilterType,
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
            "expected png, webp, avif, jpeg, jpeg:<quality> as format parameter",
        ))
    }
}

impl<'de> Deserialize<'de> for Scale {
    fn deserialize<D>(deserializer: D) -> Result<Scale, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let str = s.as_str();

        let mut x = str.split(":");
        let size: Vec<u32> = x
            .next()
            .unwrap()
            .split("x")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if size.len() != 2 {
            return Err(serde::de::Error::custom("expected width x height as scale"));
        }

        let algorithm = x.next().unwrap_or("nearest");

        let algorithm = match algorithm {
            "nearest" => image::imageops::FilterType::Nearest,
            "triangle" => image::imageops::FilterType::Triangle,
            "cubic" => image::imageops::FilterType::CatmullRom,
            "gaussian" => image::imageops::FilterType::Gaussian,
            "lanczos3" => image::imageops::FilterType::Lanczos3,
            _ => image::imageops::FilterType::Nearest,
        };

        Ok(Scale {
            width: size[0],
            height: size[1],
            algorithm,
        })
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct QueryParams {
    pub url: String,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>,
    pub grayscale: Option<bool>,
    pub blur: Option<f32>,
    pub crop: Option<String>,
    pub rotate: Option<u32>,
    pub format: Option<Format>,
    pub scale: Option<Scale>,
}
