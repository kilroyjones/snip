use super::utils::{decode_image, get_image_format, handle_duplicate_filenames};
use crate::lib::download::Download;
use crate::lib::storage::Storage;
use crate::lib::url::UrlType;
use anyhow::anyhow;
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
    /// TODO
    ///  [ ] - Look at better ways to get the filename in this case or perhaps offload to a
    ///        util function. This seems a bit messy.
    ///
    pub async fn save_image(&mut self, image: &Download) -> Result<PathBuf, anyhow::Error> {
        let image_format = get_image_format(&image.content_type, &image.extension);
        let filename = match image.extension {
            Some(_) => image.filename.clone(),
            None => format!("{}.{}", image.filename, image_format),
        };

        println!("asving base54 {} ", &filename);
        let filename: String = handle_duplicate_filenames(&self.base_path, filename);
        let file_path = self.image_original_path.join(&filename);

        match image.url_type {
            UrlType::COMMON => {
                match self.save_common_original(&image.content, &file_path).await {
                    Ok(()) => return Ok(file_path),
                    Err(e) => return Err(anyhow!("SAVE ORIGINAL")),
                };
            }
            UrlType::BASE64 => {
                println!("asving base54");
                match self.save_base64_original(&image.url, &file_path).await {
                    Ok(()) => return Ok(file_path),
                    Err(e) => return Err(anyhow!("")),
                };
            }
            _ => return Err(anyhow!("")),
        };
    }

    /// Returns nothing if the save to disk is successful
    ///
    /// TODO
    ///  [ ] - Fix the errors here to be more explicit
    ///
    async fn save_common_original(
        &mut self,
        content: &actix_web::web::Bytes,
        file_path: &PathBuf,
    ) -> Result<(), anyhow::Error> {
        let mut img_file = match File::create(&file_path) {
            Ok(img_file) => img_file,
            Err(e) => return Err(anyhow!("Error")),
        };

        match img_file.write_all(&content) {
            Ok(_) => Ok(()),
            Err(_) => return Err(anyhow!("Error")),
        }
    }

    /// Returns nothing if the base64 image is saved correctly.
    ///
    /// This looks for the comma after the initial base64 metadata and the attempts to
    /// decode that as an image file.  
    ///
    async fn save_base64_original(
        &mut self,
        url: &String,
        file_path: &PathBuf,
    ) -> Result<(), anyhow::Error> {
        println!("HERe{:?}", file_path);
        match url.find(',') {
            Some(img_start) => {
                let image = match decode_image(&url, img_start).await {
                    Ok(image) => image,
                    Err(e) => return Err(anyhow!(e)),
                };

                match image.save(file_path) {
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
