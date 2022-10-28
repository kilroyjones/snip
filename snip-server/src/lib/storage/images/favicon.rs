use crate::lib::download::Download;
use crate::lib::storage::Storage;
use anyhow::anyhow;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

impl Storage {
    /// Returns a PathBuf to the favicon location
    ///
    /// This will save a favicon with the form www.domain.com-[favicon.ico] or other
    /// filename as given in the Download object. It will overwrite previously saved
    /// favicons in favor of the most recent one.
    ///
    /// TODO
    ///  [ ] - Look at error checking for the two unwrap statements on Uurl and what to do
    ///        if they fail. Use a random file name?
    ///  [ ] - Assess whether to add the capability to use potentially multiple favicon
    ///        icons. If the site changes overtime, should the old saved items retain
    ///        that older icon or not?
    ///
    pub async fn save_favicon(&mut self, image: Download) -> Result<PathBuf, anyhow::Error> {
        let parse_url = Url::parse(&image.url).unwrap();
        let host = parse_url.host().unwrap();
        let file_path = self
            .image_favicon_path
            .join(format!("{}-{}", host, &image.filename));

        let mut img_file = match File::create(&file_path) {
            Ok(img_file) => img_file,
            Err(e) => return Err(anyhow!(format!("Error creating {}", e))),
        };

        match img_file.write_all(&image.content) {
            Ok(_) => Ok(file_path),
            Err(_) => return Err(anyhow!("Error writing")),
        }
    }
}
