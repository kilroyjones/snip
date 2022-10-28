use crate::lib::download::content_type::ContentType;
use anyhow::anyhow;
use base64::decode;
use std::path::PathBuf;

/// Returns a string of the image format
///
/// This function favors the content_type over the extension from the filename. If
/// neither are found it will guess at png.
///
pub fn get_image_format(content_type: &Option<ContentType>, extension: &Option<String>) -> String {
    match content_type {
        Some(content_type) => return content_type.subtype.clone(),
        None => (),
    };

    match extension {
        Some(extension) => extension.clone(),
        None => String::from("png"),
    }
}

/// Returns a string of a filename
///
/// This function will check for files with the same name and append a number at the
/// start.
///
/// TODO
///  [ ] - This could be an issue if a lot of files have the same name. Is there better way to do it?
///        Maybe add a random number or something?
pub fn handle_duplicate_filenames(path: &PathBuf, filename: String) -> String {
    match path.join("originals").join(filename.clone()).exists() {
        true => {
            let mut count = 1;
            while path.join(format!("{}_{}", count, filename)).exists() {
                count = count + 1;
            }
            format!("{}_{}", count, filename)
        }
        false => filename,
    }
}

/// Returns a base64 image that have been decoded from a url
///
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
