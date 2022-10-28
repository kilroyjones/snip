use crate::lib::url::UrlType;
use actix_web::web::Bytes;
use anyhow::anyhow;
use base64::decode;

pub async fn get_common_content(
    url: &String,
    response: reqwest::Response,
) -> Result<Bytes, anyhow::Error> {
    match response.bytes().await {
        Ok(content) => Ok(content),
        Err(_) => {
            return Err(anyhow!(
                "[file_download] - Failed to get file contents from response"
            ))
        }
    }
}

pub async fn get_base64_content(url: &String) -> Result<Bytes, anyhow::Error> {
    match url.find(',') {
        Some(img_start) => match decode(url[(img_start + 1)..url.len()].to_string()) {
            Ok(decoded) => Ok(Bytes::from(decoded)),
            Err(e) => return Err(anyhow!("[file_download] - Invalid base64 url")),
        },
        None => return Err(anyhow!("[file_download] - Failed to parse base64 image")),
    }
}
