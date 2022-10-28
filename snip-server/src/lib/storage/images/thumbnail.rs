use super::utils::{decode_image, get_image_format, handle_duplicate_filenames};
use crate::lib::download::Download;
use crate::lib::storage::Storage;
use crate::lib::url::UrlType;
use anyhow::anyhow;
use image::imageops::FilterType;
use image::EncodableLayout;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

impl Storage {
    /// Returns a PathBuf to a saved original image, whether base64 or not
    ///
    /// Currently favors content_type over the extension on the filename and will attempt to
    /// save with png if neither can be determined (get_image_format). Duplicate filenames will
    /// be prefixed with a number.
    ///
    /// This is similar to saving the original image (original.rs) but requires the format is known
    /// to determine between webp and other images. Currently image-rs does not handle webp.
    ///
    pub async fn save_thumbnail(&mut self, image: &Download) -> Result<PathBuf, anyhow::Error> {
        let image_format: String = get_image_format(&image.content_type, &image.extension);
        let filename: String = handle_duplicate_filenames(
            &self.base_path,
            format!("{}.{}", image.filename, image_format),
        );
        let file_path = self.image_thumbnail_path.join(&filename);

        match image.url_type {
            UrlType::COMMON => {
                match self
                    .save_common_thumbnail(&image.content, &file_path, &image_format)
                    .await
                {
                    Ok(()) => return Ok(file_path),
                    Err(e) => return Err(anyhow!(format!("saving commong{:?}", e))),
                };
            }
            UrlType::BASE64 => {
                match self.save_base64_thumbnail(&image.url, &file_path).await {
                    Ok(()) => return Ok(file_path),
                    Err(e) => return Err(anyhow!("")),
                };
            }
            _ => return Err(anyhow!("")),
        };
    }

    /// Returns nothing if the save to disk is successful
    ///
    /// With webp images it requires a separate enconder, until image-rs is able to
    /// handle webp.
    ///
    /// TODO
    ///  [ ] - Fix the errors here to be more explicit
    ///  [ ] - Add something to config which allows different thumbnail sizes.
    ///
    async fn save_common_thumbnail(
        &mut self,
        content: &actix_web::web::Bytes,
        file_path: &PathBuf,
        image_format: &String,
    ) -> Result<(), anyhow::Error> {
        // let path = self.image_thumbnail_path.join(file_path);
        println!("{:?}", file_path);
        let thumbnail = match image::load_from_memory(content.as_bytes()) {
            Ok(thumbnail) => thumbnail.resize(200, 200, FilterType::Lanczos3),
            Err(_) => return Err(anyhow!("")),
        };
        println!("Here");

        match image_format.as_str() {
            "webp" => {
                let encoder = match webp::Encoder::from_image(&thumbnail) {
                    Ok(encoder) => encoder,
                    Err(_) => return Err(anyhow!("".to_string())),
                };

                let memory = encoder.encode_lossless();
                let mut f = match File::create(file_path) {
                    Ok(f) => f,
                    Err(e) => return Err(anyhow!(e)),
                };

                match f.write_all(memory.as_bytes()) {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(anyhow!(e)),
                }
            }
            _ => match thumbnail.save(&file_path) {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{}", e);
                    return Err(anyhow!("Error"));
                }
            },
        }
    }

    /// Returns nothing if able to save to disk.
    ///
    /// TODO
    ///  [ ] - Add something to config which allows different thumbnail sizes.
    ///
    async fn save_base64_thumbnail(
        &mut self,
        url: &String,
        file_path: &PathBuf,
    ) -> Result<(), anyhow::Error> {
        match url.find(',') {
            Some(img_start) => {
                let image = match decode_image(&url, img_start).await {
                    Ok(image) => image,
                    Err(e) => return Err(anyhow!(e)),
                };

                let thumbnail = image.resize(200, 200, FilterType::Lanczos3);
                match thumbnail.save(file_path) {
                    Ok(_) => Ok(()),
                    Err(e) => return Err(anyhow!(e)),
                }
            }
            _ => {
                return Err(anyhow!("Couldn't parse base64 url"));
            }
        }
    }
}
