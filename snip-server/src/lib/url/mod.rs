// put this in base folder

use anyhow::anyhow;
use regex::Regex;
use url::Url;

#[derive(Debug)]
pub enum UrlType {
    BASE64,
    COMMON,
    UNKNOWN,
}

/// Retuns url type based on UrlType enum
///
/// Types are only COMMON, which represents the majority of urls, and
/// base64. Everything else will be UNKNOWN
///
/// Todo
/// [ ] - Add further granularity for ftp, magnet links and others.
///
pub fn get_url_type(url: &String) -> Result<UrlType, anyhow::Error> {
    if is_common_url(url) {
        return Ok(UrlType::COMMON);
    }

    match is_base64_url(url) {
        Ok(result) => match result {
            true => Ok(UrlType::BASE64),
            false => Ok(UrlType::UNKNOWN),
        },
        Err(e) => Err(e),
    }
}

/// Returns bool base on whether the url is a valid structure
///
/// Structure is determined first by the ability to parse and then by if it can
/// be a *base* or not. This is mainly used to weed out **mailto:** and **data:**
/// type urls, but will not catch those that are **ftp://** or those which include
/// local file systems.
///
pub fn is_common_url(url: &String) -> bool {
    match Url::parse(url.as_str()) {
        Ok(parsed_url) => {
            if parsed_url.cannot_be_a_base() {
                return false;
            }
        }
        Err(_) => {
            return false;
        }
    }
    true
}

/// Returns bool Result if the url is structured like a base64 url
///
/// Note that this does not determine of the base64 data is valid or not, and as
/// such any url that this states is valid, may still fail on attempt to decode.
///
pub fn is_base64_url(url: &String) -> Result<bool, anyhow::Error> {
    println!("FUCK");
    let re = match Regex::new(r"data:image/[^;]+;base64.+") {
        Ok(re) => re,
        Err(e) => {
            println!("{}", e);
            return Err(anyhow!("[URL] - Error setting up regex"));
        }
    };

    println!("FUCK2");
    match re.is_match(url) {
        true => Ok(true),
        false => Ok(false),
    }
}
