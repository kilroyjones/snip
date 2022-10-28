pub mod content_type;
mod file_content;
mod filename;

use crate::lib::url::{get_url_type, UrlType};
use anyhow::anyhow;
use content_type::{get_base64_content_type, get_common_content_type, ContentType};
use file_content::{get_base64_content, get_common_content};
use filename::*;

#[derive(Debug)]
pub struct Download {
    pub url: String,
    pub url_type: UrlType,
    pub filename: String,
    pub extension: Option<String>,
    pub content: actix_web::web::Bytes,
    pub content_type: Option<ContentType>,
}

impl Download {
    #[tracing::instrument(name = "Starting download")]
    pub async fn new(url: String) -> Result<Download, anyhow::Error> {
        let url_type: UrlType = match get_url_type(&url) {
            Ok(url_type) => url_type,
            Err(_) => return Err(anyhow!("[file_download] - Invalid URL")),
        };

        let download = match url_type {
            UrlType::COMMON | UrlType::UNKNOWN => match handle_common(url, url_type).await {
                Ok(download) => download,
                Err(_) => return Err(anyhow!("Error downloading common image")),
            },
            UrlType::BASE64 => match handle_base64(url, url_type).await {
                Ok(download) => download,
                Err(_) => return Err(anyhow!("Error downloading base64 image")),
            },
        };
        Ok(download)
    }
}

async fn handle_common(url: String, url_type: UrlType) -> Result<Download, anyhow::Error> {
    let filename = match get_filename_from_url(&url) {
        Some(filename) => filename,
        None => get_random_filename(),
    };

    let extension = get_filename_extension(&filename);

    let response = match reqwest::get(url.clone()).await {
        Ok(response) => match response.error_for_status() {
            Ok(response) => response,
            Err(_) => return Err(anyhow!("[file_download] - Download request failed")),
        },
        Err(_) => return Err(anyhow!("[file_download] - Download request failed")),
    };

    tracing::info!("RESPONSE: {:?}", &response);

    let content_type = get_common_content_type(&response);

    let content = match get_common_content(&url, response).await {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    Ok(Download {
        url: url,
        url_type: url_type,
        filename: filename,
        extension: extension,
        content: content,
        content_type: content_type,
    })
}

async fn handle_base64(url: String, url_type: UrlType) -> Result<Download, anyhow::Error> {
    let filename = get_random_filename();
    let extension = get_filename_extension(&filename);
    let content_type = get_base64_content_type(&url);
    let content = match get_base64_content(&url).await {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    Ok(Download {
        url: url,
        url_type: url_type,
        filename: filename,
        extension: extension,
        content: content,
        content_type: content_type,
    })
}
