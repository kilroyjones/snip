use rand::distributions::{Alphanumeric, DistString};
use std::path::Path;

/// Returns either a string of the filename or None
///
/// Parses the url into segments which per the docs are based on splitting by slash '/'
///
pub fn get_filename_from_url(url: &str) -> Option<String> {
    let parsed = match url::Url::parse(url) {
        Ok(parsed) => parsed,
        Err(_) => return None,
    };

    let segments = match parsed.path_segments() {
        Some(parsed) => parsed,
        None => return None,
    };

    let mut last = match segments.last() {
        Some(last) => last.to_string(),
        None => return None,
    };

    // Remove whitespace to prevent %20 errors in the url
    // TODO: Replace this, it's really hacky right now
    last.retain(|c| !c.is_whitespace());
    last.retain(|c| c != '%');
    Some(last)
}

/// Returns a random 24 character filename
///
/// The number 24 was not chosed for any particular reason, just figured it was
/// sufficiently large to avoid duplicates.
///
pub fn get_random_filename() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 24)
}

/// Returns either the extension of None
///
/// The reason for the variable name raw_extension is that the extension function
/// returns a value that may or may not be encoded correctly and may failed when
/// converted to what I assume is a utf-8 string?
///
pub fn get_filename_extension(filename: &String) -> Option<String> {
    match Path::new(filename).extension() {
        Some(raw_extension) => match raw_extension.to_str() {
            Some(extension) => Some(String::from(extension.to_lowercase())),
            None => None,
        },
        None => None,
    }
}

/// Returns the filename without the extension
///
pub fn get_filestem(filename: &String) -> Option<String> {
    let path = Path::new(&filename)
        .file_stem()
        .map(|s| s.to_string_lossy().into())
        .unwrap_or("".to_string());

    match path.as_str() {
        "" => None,
        _ => Some(path),
    }
}
