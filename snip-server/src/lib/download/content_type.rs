use mime::Mime;
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use std::str::FromStr;

#[derive(Debug)]
pub struct ContentType {
    pub maintype: String,
    pub subtype: String,
}

impl ContentType {
    pub fn new(maintype: String, subtype: String) -> ContentType {
        ContentType {
            maintype: maintype,
            subtype: subtype,
        }
    }
}

/// Returns the content type of a non-base64 url
///
pub fn get_common_content_type(response: &reqwest::Response) -> Option<ContentType> {
    // Should add other mediatypes
    let header = response.headers();
    match header.get(CONTENT_TYPE) {
        Some(content_type) => get_types(content_type),
        None => None,
    }
}

/// Returns a lowercase String type of the mime/content_type
///
/// This function returns an Option for the type if it is not found. Additional
/// changes will likely be made within the match subtype block as needed. I've changed
/// jpeg to jpg here to alleviate some difficulties I've had with parsing and saving
/// using the two different extensions.
///
/// In cases when converting to or from strings there may be encoding errors, and when that
/// happens it'll simply return None.  
///
pub fn get_types(content_type: &reqwest::header::HeaderValue) -> Option<ContentType> {
    let mime = match content_type.to_str() {
        Ok(mime) => mime,
        Err(_) => "",
    };

    match Mime::from_str(mime) {
        Ok(content_type) => {
            let maintype: String = content_type.type_().as_str().trim().to_lowercase();
            let subtype: String = content_type.subtype().as_str().trim().to_lowercase();
            Some(ContentType::new(maintype, subtype))
        }
        Err(_) => None,
    }
}

/// Returns the content type of a base64 url
///
/// This makes no assumptions about the validity of the url aside from that it contains
/// the normal pattern of data:image/ followed by the base64 data.
///
pub fn get_base64_content_type(url: &String) -> Option<ContentType> {
    match Regex::new(r"data:image/(.*);base64") {
        Ok(re) => match re.captures(&url) {
            Some(cap) => Some(ContentType::new(
                String::from("image"),
                cap[1].to_string().to_lowercase(),
            )),
            None => None,
        },
        Err(_) => None,
    }
}
