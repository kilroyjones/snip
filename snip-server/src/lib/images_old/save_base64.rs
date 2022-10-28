use anyhow::anyhow;
use base64::decode;
use image::imageops::FilterType;

pub async fn save_base64_image(
    base: &String,
    filename: &String,
    url: &String,
) -> Result<(), anyhow::Error> {
    match url.find(',') {
        Some(img_start) => {
            let image = match decode_image(&url, img_start).await {
                Ok(image) => image,
                Err(e) => return Err(anyhow!(e)),
            };

            match save_base64_original(&image, &base, &filename).await {
                Ok(_) => match save_base64_thumbnail(&base, &filename).await {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(anyhow!(e)),
                },
                Err(e) => return Err(anyhow!(e)),
            }
        }
        _ => {
            return Err(anyhow!("Couldn't parse base64 url"));
        }
    }
}

pub async fn decode_image(
    url: &String,
    img_start: usize,
) -> Result<image::DynamicImage, anyhow::Error> {
    let decoded = match decode(url[(img_start + 1)..url.len()].to_string()) {
        Ok(decoded) => decoded,
        Err(e) => return Err(anyhow!(e)),
    };

    let image = match image::load_from_memory(&decoded) {
        Ok(image) => image,
        Err(e) => return Err(anyhow!(e)),
    };

    Ok(image)
}

pub async fn save_base64_original(
    image: &image::DynamicImage,
    base: &String,
    filename: &String,
) -> Result<(), anyhow::Error> {
    match image.save(format!("{}/original/{}", base, filename)) {
        Ok(_) => Ok(()),
        Err(e) => return Err(anyhow!(e)),
    }
}

pub async fn save_base64_thumbnail(base: &String, filename: &String) -> Result<(), anyhow::Error> {
    let encoded_image = match image::io::Reader::open(format!("{}/original/{}", base, filename)) {
        Ok(encoded_image) => encoded_image,
        Err(e) => return Err(anyhow!(e)),
    };

    let mut thumbnail = match encoded_image.decode() {
        Ok(thumbnail) => thumbnail,
        Err(e) => return Err(anyhow!(e)),
    };

    thumbnail = thumbnail.resize(200, 200, FilterType::Lanczos3);
    match thumbnail.save(format!("{}/thumbnail/{}", base, filename)) {
        Ok(_) => Ok(()),
        Err(e) => return Err(anyhow!(e)),
    }
}
