// let mut filename = String::from("temp");
// let mut filename = Url::parse(url.as_str());
// let base: String = String::from("./files/images");

// let response = reqwest::get(url).await;
// let content = response.bytes().await.unwrap();
// let path = format!("{}/original/{}", base, filename);
// let mut original = File::create(&path).unwrap();
// let t = mime_guess::from_path(&path);
// println!("{:?}", t);

// println!("{}", url.clone());
// let mut filename = get_last(url.as_str()).unwrap();
// println!("{}", filename);
// let mut count = 0;
// if Path::new(&format!("{}/original/{}", base, filename)).exists() {
//     while Path::new(&format!("{}/original/{}_{}", base, filename, count)).exists() {
//         count = count + 1;
//     }
// }
// if count > 0 {
//     filename = format!("{}_{}", filename, count);
// }

// let response = reqwest::get(url).await.unwrap();
// let content = response.bytes().await.unwrap();
// let path = format!("{}/original/{}", base, filename);
// let mut original = File::create(&path).unwrap();
// let t = mime_guess::from_path(&path);

//     Ok(true) => {
//     }
//     _ => {
//         //Strip extension and use image_type
//     }
// match is_valid_extension(&filename, image_type.clone()) {
//     Ok(true) => {
//         //Save file as is
//         let filename = get_checked_filename(&base, &filename, &image_type).await;
//         match write_image_to_disk(base, content, &filename).await {
//             Ok(_) => return Ok(filename),
//             Err(e) => return Err(e),
//         }
//     }
//     _ => {
//         //Strip extension and use image_type
//     }
// }

// pub fn get_suffix(url: String) -> Result<String, anyhow::Error> {
//     let re = Regex::new(r"[^\/]+(?=\/$|$)").unwrap();
//     let caps = re.captures(&url).unwrap();
//     let res = caps.get(1).map_or("", |m| m.as_str());
//     Ok(res.into())
// }

// pub fn is_image_url(response: reqwest::Response) -> bool {
//     let header = response.headers();
//     let content_type = match header.get("content-type") {
//         Some(content_type) => content_type,
//         _ => return false,
//     };

//     let value = match content_type.to_str() {
//         Ok(value) => value,
//         _ => {
//             return false;
//         }
//     };

//     if value[0..5].eq("image") {
//         return true;
//     }
//     false
// }

// if extension.eq(&image_type) {
//     return filename;
// }

// pub fn is_webp(url: String) -> bool {
//     //First check file extension
//     let len = url.len();
//     if len > 5 {
//         if &url[len - 5..len].to_lowercase() == ".webp" {
//             return true;
//         }
//     }
//     false
// }

//     if res {
//         //save file as is
//         let filename = get_checked_filename(&base, &filename, &content_type).await;
//         match write_image_to_disk(base, content, &filename).await {
//             ok(_) => return (filename),
//             err(e) => return err(e),
//         };
//     } else {
//         // strip extension and use image type
//         let filename = format!("{}.{}", get_file_stem(&filename), content_type);
//         match write_image_to_disk(base, content, &filename).await {
//             ok(_) => return ok(filename),
//             err(e) => return err(e),
//         };
//     }
// } else {
//     let filename = format!("{}.{}", get_file_stem(&filename), content_type);
//     match write_image_to_disk(base, content, &filename).await {
//         ok(_) => return ok(filename),
//         err(e) => return err(e),
//     };
// }

// async fn save_webp_image(
//     base: String,
//     content: actix_web::web::Bytes,
//     filename: String,
//     content_type: String,
// ) -> Result<String, anyhow::Error> {
//     if has_extension(&filename) {
//         let res = match is_valid_extension(&filename, content_type.clone()) {
//             Ok(true) => true,
//             _ => false,
//         };

//         if res {
//             //Save file as is
//             let filename = get_checked_filename(&base, &filename, &content_type).await;
//             match write_webp_image_to_disk(base, content, &filename).await {
//                 Ok(_) => return Ok(filename),
//                 Err(e) => return Err(e),
//             };
//         } else {
//             //Strip extension and usecontent_type
//             let filename = format!("{}.{}", get_file_stem(&filename), content_type);
//         };
//     } else {
//         let filename = format!("{}.{}", get_file_stem(&filename), content_type);
//         match write_webp_image_to_disk(base, content, &filename).await {
//             Ok(_) => return Ok(filename),
//             Err(e) => return Err(e),
//         };
//     }
// }

// pub fn get_content_type(headers: &reqwest::header::HeaderMap) -> Result<String, anyhow::Error> {
//     // Should add other mediatypes
//     println!("{:?}", headers.get(CONTENT_TYPE));
//     let content_type = match headers.get(CONTENT_TYPE) {
//         None => "",
//         Some(content_type) => {
//             let content_type = Mime::from_str(content_type.to_str()?)?;
//             let subtype: String = content_type.subtype().as_str().trim().to_lowercase();
//             let media_type = match subtype.as_ref() {
//                 "png" => "png",
//                 "jpeg" => "jpg",
//                 "jpg" => "jpg",
//                 "gif" => "gif",
//                 "webp" => "webp",
//                 _ => "",
//             };
//             media_type
//         }
//     };
//     Ok(content_type.to_string())
// }

// pub async fn process_image(url: String) -> Result<String, anyhow::Error> {
//     let base: String = String::from("./files/images");
//     if is_url(url.clone()) {
//         match add_from_url(base, url).await {
//             ok(filename) => {
//                 return ok(filename);
//             }
//             err(e) => return err(e),
//         };
//     } else {
//         match add_from_base64(base, url).await {
//             ok(filename) => {
//                 return ok(filename);
//             }
//             err(e) => return err(e),
//         }
//     }
// }

// async pub fn filename_has_suffix(url: &String) -> Result<bool, anyhow::Error> {
//     let re = Regex::new(r"[^\/]+(?=\/$|$)").unwrap();
//     if re.is_match(&url) {
//         return Ok(true);
//     }
//     Ok(false)
// }

// let filename = match process_image(image.url.clone()).await {
//     Ok(filename) => filename,
//     Err(_) => {
//         return web::Json(SnipConfirmation {
//             msg: "Failed".into(),
//         });
//     }
// };

// pub fn has_suffix(url: String) -> Result<bool, anyhow::Error> {
//     let re = Regex::new(r"[^\/]+(?=\/$|$)").unwrap();
//     if re.is_match(&url) {
//         return Ok(true);
//     }
//     Ok(false)
// }

// pub async fn get_unique_filename(base: &String) -> String {
//     let mut filename = Alphanumeric.sample_string(&mut rand::thread_rng(), 24);
//     while Path::new(&format!("{}/original/{}", base, filename)).exists() {
//         filename = Alphanumeric.sample_string(&mut rand::thread_rng(), 24);
//     }
//     filename
// }

pub fn is_valid_extension(filename: &String, content_type: String) -> Result<bool, anyhow::Error> {
    let extension: String = Path::new(filename)
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if extension.eq(&content_type) {
        return Ok(true);
    }
    Ok(false)
}
use super::file_helpers::get_unique_filename;
use anyhow::anyhow;
use base64::decode;
use image::imageops::FilterType;
use regex::Regex;

pub async fn add_from_base64(base: String, url: String) -> Result<String, anyhow::Error> {
    match url.find(',') {
        Some(img_start) => {
            let image = match decode_image(&url, img_start).await {
                Ok(image) => image,
                Err(e) => return Err(anyhow!(e)),
            };
            let filename = get_unique_filename(&base).await;

            let content_type = match get_content_type(&url).await {
                Ok(content_type) => content_type,
                Err(e) => return Err(anyhow!(e)),
            };

            match save_original(&image, &base, &filename, &content_type).await {
                Ok(_) => match save_thumbnail(&base, &filename, &content_type).await {
                    Ok(_) => return Ok(filename),
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

pub async fn get_content_type(url: &String) -> Result<String, anyhow::Error> {
    let re = match Regex::new(r"data:image/(.*);base64") {
        Ok(re) => re,
        Err(e) => {
            println!("{:?}", e);
            return Err(anyhow!(e));
        }
    };

    let content_type = match re.captures(&url) {
        Some(cap) => cap[1].to_string(),
        None => return Err(anyhow!("".to_string())),
    };
    Ok(content_type)
}

pub async fn save_original(
    image: &image::DynamicImage,
    base: &String,
    filename: &String,
    content_type: &String,
) -> Result<(), anyhow::Error> {
    match image.save(format!("{}/original/{}.{}", base, filename, content_type)) {
        Ok(_) => Ok(()),
        Err(e) => return Err(anyhow!(e)),
    }
}

pub async fn save_thumbnail(
    base: &String,
    filename: &String,
    content_type: &String,
) -> Result<(), anyhow::Error> {
    let encoded_image =
        match image::io::Reader::open(format!("{}/original/{}.{}", base, filename, content_type)) {
            Ok(encoded_image) => encoded_image,
            Err(e) => return Err(anyhow!(e)),
        };

    let mut thumbnail = match encoded_image.decode() {
        Ok(thumbnail) => thumbnail,
        Err(e) => return Err(anyhow!(e)),
    };

    thumbnail = thumbnail.resize(100, 100, FilterType::Lanczos3);
    match thumbnail.save(format!("{}/thumbnail/{}.{}", base, filename, content_type)) {
        Ok(_) => Ok(()),
        Err(e) => return Err(anyhow!(e)),
    }
}

use super::file_helpers::{
    get_checked_filename, get_file_stem, get_unique_filename, has_extension, is_valid_extension,
};
use super::url_helpers::{get_content_type, get_filename_from_url};
use anyhow::anyhow;
use image::imageops::FilterType;
use image::EncodableLayout;
use std::fs::File;
use std::io::Write;

pub async fn add_from_url(base: String, url: String) -> Result<String, anyhow::Error> {
    let response = reqwest::get(url.clone()).await.unwrap();
    let headers = response.headers();
    let content_type = match get_content_type(headers) {
        Ok(content_type) => content_type,
        Err(e) => {
            return Err(e);
        }
    };

    let filename = match get_filename_from_url(url.clone().as_str()) {
        Ok(filename) => match process_filename(&base, &filename, &content_type).await {
            Ok(filename) => filename,
            Err(e) => return Err(e),
        },
        Err(_) => get_unique_filename(&base).await,
    };

    let image_content = response.bytes().await.unwrap();
    match save_image(base, image_content, filename.clone(), content_type).await {
        Ok(_) => Ok(filename),
        Err(e) => Err(anyhow!(e)),
    }
}

async fn process_filename(
    base: &String,
    filename: &String,
    content_type: &String,
) -> Result<String, anyhow::Error> {
    if has_extension(&filename) {
        match is_valid_extension(&filename, content_type.clone()) {
            Ok(true) => return Ok(get_checked_filename(&base, &filename, &content_type).await),
            Ok(false) => return Ok(format!("{}.{}", get_file_stem(&filename), content_type)),
            Err(e) => return Err(anyhow!(e)),
        };
    } else {
        match content_type.eq("") {
            true => return Ok(format!("{}.{}", get_file_stem(&filename), "png")),
            false => return Ok(format!("{}.{}", get_file_stem(&filename), content_type)),
        }
    }
}

async fn save_image(
    base: String,
    image_content: actix_web::web::Bytes,
    filename: String,
    content_type: String,
) -> Result<(), anyhow::Error> {
    match content_type.as_ref() {
        "webp" => {
            match write_webp_image_to_disk(base, image_content, filename).await {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            };
        }
        _ => {
            // Save other images
            match write_image_to_disk(base, image_content, filename).await {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }
}

async fn write_image_to_disk(
    base: String,
    content: actix_web::web::Bytes,
    filename: String,
) -> Result<(), anyhow::Error> {
    let path = format!("{}/original/{}", base, filename);
    let mut original = File::create(&path).unwrap();
    let _ = original.write_all(&content); // CATCH ERROR ON THIS
    let mut thumbnail = image::io::Reader::open(format!("{}/original/{}", base, filename))
        .unwrap()
        .decode()
        .unwrap();
    thumbnail = thumbnail.resize(100, 100, FilterType::Lanczos3);
    thumbnail
        .save(format!("{}/thumbnail/{}", base, filename))
        .unwrap();
    Ok(())
}

async fn write_webp_image_to_disk(
    base: String,
    content: actix_web::web::Bytes,
    filename: String,
) -> Result<(), anyhow::Error> {
    let path = format!("{}/original/{}", base, filename);
    let mut original = File::create(&path).unwrap();
    let _ = original.write_all(&content); // CATCH ERROR ON THIS
    let mut thumbnail = image::io::Reader::open(format!("{}/original/{}", base, filename))
        .unwrap()
        .decode()
        .unwrap();
    thumbnail = thumbnail.resize(100, 100, FilterType::Lanczos3);

    let encoder = match webp::Encoder::from_image(&thumbnail) {
        Ok(encoder) => encoder,
        Err(e) => return Err(anyhow!("".to_string())),
    };

    let memory = encoder.encode_lossless();
    let mut f = match File::create(format!("{}/thumbnail/{}", base, filename)) {
        Ok(f) => f,
        Err(e) => return Err(anyhow!(e)),
    };

    match f.write_all(memory.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(anyhow!(e)),
    };
    Ok(())
}
use mime::Mime;
use reqwest::header::CONTENT_TYPE;
use std::str::FromStr;

pub fn get_content_type(headers: &reqwest::header::HeaderMap) -> Result<String, anyhow::Error> {
    // Should add other mediatypes
    println!("{:?}", headers.get(CONTENT_TYPE));
    let content_type = match headers.get(CONTENT_TYPE) {
        None => "",
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let subtype: String = content_type.subtype().as_str().trim().to_lowercase();
            let media_type = match subtype.as_ref() {
                "png" => "png",
                "jpeg" => "jpg",
                "jpg" => "jpg",
                "gif" => "gif",
                "webp" => "webp",
                _ => "",
            };
            media_type
        }
    };
    Ok(content_type.to_string())
}

pub fn get_filename_from_url(url: &str) -> Result<String, &str> {
    url::Url::parse(url)
        .map_err(|_| "")?
        .path_segments()
        .ok_or("")?
        .last()
        .ok_or("")
        .map(String::from)
}

// for x in 0..get_snippet.snippet_types.len() {
//     q = q.bind(get_snippet.snippet_types[x].clone());
// }

use crate::lib::url::UrlType;
use rand::distributions::{Alphanumeric, DistString};
use std::path::Path;

pub async fn get_filename(
    url: &String,
    url_type: &UrlType,
    content_type: &String,
) -> Result<String, anyhow::Error> {
    // Get or generate filename
    let filename = match url_type {
        UrlType::COMMON => match get_filename_from_url(&url).await {
            Some(filename) => filename.replace("%", ""),
            None => get_random_filename().await,
        },
        UrlType::BASE64 => get_random_filename().await,
        UrlType::UNKNOWN => get_random_filename().await,
    };

    // This should be combined as we're checking for extension on names known to not have one along with others
    let filename_extension = get_filename_extension(&filename).await;

    let filename = match filename_extension.eq("") {
        true => match content_type.eq("") {
            true => format!("{}.png", filename),
            false => format!("{}.{}", filename, content_type),
        },
        false => match content_type.ne("") {
            true => format!("{}.{}", get_file_stem(&filename), content_type),
            false => filename,
        },
    };
    Ok(filename)
}

// pub async fn get_filename_from_url(url: &str) -> Result<String, &str> {
pub async fn get_filename_from_url(url: &str) -> Option<String> {
    let parsed = match url::Url::parse(url) {
        Ok(parsed) => parsed,
        Err(_) => return None,
    };

    let segments = match parsed.path_segments() {
        Some(parsed) => parsed,
        None => return None,
    };

    match segments.last() {
        Some(last) => Some(last.to_string()),
        None => return None,
    }
}

pub async fn get_random_filename() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 24)
}

pub async fn get_filename_extension(filename: &String) -> String {
    // raw extension here refers to whether extension can be correctly converted to unicdoe or not
    // - double check docs on .extension()
    let raw_extension = match Path::new(filename).extension() {
        Some(raw_extension) => raw_extension,
        None => return String::from(""),
    };

    match raw_extension.to_str() {
        Some(extension) => return extension.to_string(),
        None => return String::from(""),
    };
}

pub fn get_file_stem(filename: &String) -> Option<String> {
    Path::new(&filename)
        .file_stem()
        .map(|s| s.to_string_lossy().into())
        .unwrap_or(None)
}

pub fn does_file_exist() {}

pub async fn check_if_file_exists(
    base: &String,
    filename: String,
    content_type: &String,
) -> String {
    let mut count = 0;
    let file_stem = get_file_stem(&filename);

    if Path::new(&format!("{}/original/{}", base, filename)).exists() {
        count = count + 1;
        while Path::new(&format!(
            "{}/original/{}_{}.{}",
            base, file_stem, count, content_type
        ))
        .exists()
        {
            count = count + 1;
        }
    }
    if count > 0 {
        return format!("{}_{}.{}", file_stem, count, content_type);
    }
    filename
}

pub async fn get_favicon(url: String) -> String {
    let parse_url = Url::parse(&url).unwrap();
    let host = parse_url.host().unwrap();
    let favicon_url = format!("{}://{}/{}", parse_url.scheme(), host, "favicon.ico");
    println!("{}", favicon_url);
    let response = match reqwest::get(favicon_url).await {
        Ok(response) => Some(response),
        Err(_) => return String::from(""),
    };
    println!("Here");

    let base: String = String::from("./files/images");
    let filename = format!("{}", host);

    // need to check if already exists and then just get the existing file and add it
    // TODO: fix this for this instance let filename = check_if_file_exists(&base, filename, &String::from(".ico")).await;

    let path = format!("{}/original/{}.ico", base, filename);

    let content = response.unwrap().bytes().await.unwrap();
    let mut original = File::create(&path).unwrap();
    let _ = original.write_all(&content);

    "".into()
}

// match sqlx::query!(
//     r#"
//         INSERT INTO snippets (
//             date,
//             content,
//             code_type,
//             source,
//             description,
//             snippet_type,
//             mimetype)
//         VALUES ($1, $2, $3, $4, $5, $6, $7)
//         RETURNING id
//     "#,
//     timestamp,
//     quote.quote,
//     code_type,
//     quote.source,
//     description,
//     "quote",
//     "text/plain"
// )
// .fetch_one(db_pool.get_ref())
// .await
// {
//     // could return Ok(rec) => rec?
//     Ok(_) => (),
//     Err(_) => return web::Json(SnipMessage { msg: "Ok".into() }),
// };

//     image: web::Json<Image>,
//     db_pool: web::Data<PgPool>,
//     broadcaster: web::Data<Broadcaster>,
// ) -> impl Responder {
//     let base: String = String::from("./files/images");
//     let url = image.url.clone();

//     // Is reregular image or base64
//     let url_type = match get_url_type(&url).await {
//         Ok(url_type) => url_type,
//         Err(_) => return error_message("Error"),
//     };
//     println!("{:?}", url_type);

//     // Get http request
//     let response = match reqwest::get(url.clone()).await {
//         Ok(response) => Some(response),
//         Err(_) => None,
//     };

//     // Get contennt_type
//     let content_type = match get_content_type(&url, &url_type, &response).await {
//         Ok(content_type) => content_type,
//         Err(_) => return error_message("Error"),
//     };

//     // Get filename
//     let filename = match get_filename(&url, &url_type, &content_type).await {
//         Ok(content_type) => content_type,
//         Err(_) => return error_message("Error"),
//     };

//     // Modify filename if another file already exists
//     let filename = check_if_file_exists(&base, filename, &content_type).await;

//     // Get description
//     let description: String = match &image.description {
//         Some(d) => d.to_string(),
//         None => String::from(""),
//     };

//     // Save file
//     match url_type {
//         UrlType::COMMON => {
//             match save_common_image(&base, &filename, &content_type, response).await {
//                 Ok(_) => (),
//                 Err(_) => return error_message("Error"),
//             }
//         }
//         UrlType::BASE64 => match save_base64_image(&base, &filename, &url).await {
//             Ok(_) => (),
//             Err(_) => return error_message("Error"),
//         },
//         UrlType::UNKNOWN => return error_message("error"),
//     };

//     match add_to_db(
//         db_pool,
//         image.url.clone(),
//         image.source.clone(),
//         description,
//         base,
//         filename,
//         broadcaster,
//     )
//     .await
//     {
//         Ok(_) => return web::Json(SnipMessage { msg: "Ok".into() }),
//         Err(_) => return error_message("Error writing to the database"),
//     }
// }

// async fn add_to_db(
//     db_pool: web::Data<PgPool>,
//     content: String,
//     source: String,
//     description: String,
//     base: String,
//     filename: String,
//     broadcaster: web::Data<Broadcaster>,
// ) -> Result<(), anyhow::Error> {
//     // Add to DB
//     println!("ADD TO DB:");
//     let timestamp = Utc::now().naive_utc();
//     println!("{:?}, {:?}", base, filename);
//     match sqlx::query!(
//         r#"
//             INSERT INTO snippets (
//                 date,
//                 content,
//                 source,
//                 description,
//                 snippet_type,
//                 mimetype,
//                 image_path,
//                 thumbnail_path)
//             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
//             RETURNING id
//         "#,
//         timestamp,
//         content,
//         source,
//         description,
//         "image",
//         "image/png",
//         format!("{}/original/{}", base, filename),
//         format!("{}/thumbnail/{}", base, filename),
//     )
//     .fetch_one(db_pool.get_ref())
//     .await
//     {
//         Ok(_) => (),
//         Err(e) => {
//             println!("{:?}", e);
//             return Err(anyhow!(e));
//         }
//     }
//     broadcaster.broadcast("update".into(), "image".into()).await;
//     Ok(())
// }

// let params = get_snippet
//     .snippet_types
//     .clone()
//     .into_iter()
//     .map(|p| format!("\'{}\'", p))
//     .collect::<Vec<String>>()
//     .join(",");

// let query_count = format!(
//     "SELECT
//         COUNT(*)
//     FROM
//        {}
//     WHERE snippet_type in ({}) AND trashed={}
//     ",
//     params, params, get_snippet.trash
// );

// let res: i64 = sqlx::query_scalar::<sqlx::Postgres, i64>(&query_count)
//     .fetch_one(storage.db_pool.get_ref())
//     .await
//     .unwrap();
// println!("{}", res);

// let query_statement = format!(
//     "SELECT
//     id,
//         date,
//         content,
//         code_type,
//         source,
//         description,
//         image_path,
//         thumbnail_path,
//         snippet_type,
//         trashed
//         FROM
//             snippets
//         WHERE
//             snippet_type IN ({}) AND trashed={}
//             ORDER BY id DESC
//             LIMIT {} OFFSET {}
//         ",
//     params, get_snippet.trash, get_snippet.limit, get_snippet.offset
// );

// let query = sqlx::query_as::<sqlx::Postgres, Snippet>(&query_statement);
// let records = query.fetch_all(db_pool.get_ref()).await.unwrap();
// let snippets = ReturnSnippets {
//     snippets: records,
//     pages: (res / get_snippet.limit as i64) as usize + 1,
// };
// web::Json(snippets)
// web::Json(SnipMessage { msg: "Ok".into() })
use crate::lib::url::UrlType;
use anyhow::anyhow;
use mime::Mime;
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use std::str::FromStr;

pub async fn get_content_type(
    url: &String,
    url_type: &UrlType,
    response: &Option<reqwest::Response>,
) -> Result<String, anyhow::Error> {
    match url_type {
        UrlType::COMMON => match response {
            Some(response) => get_common_content_type(response).await,
            None => return Err(anyhow!("Couldn't unwrap response")),
        },
        UrlType::BASE64 => get_base64_content_type(url).await,
        _ => Ok(String::from("")),
    }
}

pub async fn get_common_content_type(
    response: &reqwest::Response,
) -> Result<String, anyhow::Error> {
    // Should add other mediatypes
    let header = response.headers();
    let content_type = match header.get(CONTENT_TYPE) {
        None => "",
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let subtype: String = content_type.subtype().as_str().trim().to_lowercase();
            let media_type = match subtype.as_ref() {
                "png" => "png",
                "jpeg" => "jpg",
                "jpg" => "jpg",
                "gif" => "gif",
                "webp" => "webp",
                _ => "",
            };
            media_type
        }
    };
    Ok(content_type.to_string())
}

pub async fn get_base64_content_type(url: &String) -> Result<String, anyhow::Error> {
    let re = match Regex::new(r"data:image/(.*);base64") {
        Ok(re) => re,
        Err(e) => {
            println!("{:?}", e);
            return Err(anyhow!(e));
        }
    };

    let content_type = match re.captures(&url) {
        Some(cap) => cap[1].to_string(),
        None => return Err(anyhow!("".to_string())),
    };
    Ok(content_type)
}

//         let filename = match url_type {
//             UrlType::COMMON => match get_filename_from_url(&url) {
//                 Some(filename) => filename,
//                 None => get_random_filename(),
//             },
//             UrlType::BASE64 => get_random_filename(),
//             UrlType::UNKNOWN => get_random_filename(),
//         };

//         let extension = get_filename_extension(&filename);

//         let response = match reqwest::get(url.clone()).await {
//             Ok(response) => response,
//             Err(_) => return Err(anyhow!("[file_download] - Download request failed")),
//         };

//         let content_type = match url_type {
//             UrlType::COMMON => get_common_content_type(&response),
//             UrlType::BASE64 => get_base64_content_type(&url),
//             UrlType::UNKNOWN => None,
//         };

//         let content = match get_content(&url, &url_type, response).await {
//             Ok(content) => content,
//             Err(e) => return Err(e),
//         };

//         Ok(Download {
//             url: url,
//             url_type: url_type,
//             filename: filename,
//             extension: extension,
//             content: content,
//             content_type: content_type,
//         })
//     }
// }

// async fn handle_common(url: String, url_type: UrlType) -> Result<Download, anyhow::Error> {
//             let filename = match get_filename_from_url(&url) {
//                 Some(filename) => filename,
//                 None => get_random_filename(),
//             };

//         let extension = get_filename_extension(&filename);

//         let response = match reqwest::get(url.clone()).await {
//             Ok(response) => response,
//             Err(_) => return Err(anyhow!("[file_download] - Download request failed")),
//         };

//         let content_type = get_common_content_type(&response);

//         let content = match get_common_content(&url, response).await {
//             Ok(content) => content,
//             Err(e) => return Err(e),
//         };

//         Ok(Download {
//             url: url,
//             url_type: url_type,
//             filename: filename,
//             extension: extension,
//             content: content,
//             content_type: content_type,
//         })

        let filename = match url_type {
            UrlType::COMMON => match get_filename_from_url(&url) {
                Some(filename) => filename,
                None => get_random_filename(),
            },
            UrlType::BASE64 => get_random_filename(),
            UrlType::UNKNOWN => get_random_filename(),
        };

        let extension = get_filename_extension(&filename);

        let response = match reqwest::get(url.clone()).await {
            Ok(response) => response,
            Err(_) => return Err(anyhow!("[file_download] - Download request failed")),
        };

        let content_type = match url_type {
            UrlType::COMMON => get_common_content_type(&response),
            UrlType::BASE64 => get_base64_content_type(&url),
            UrlType::UNKNOWN => None,
        };

        let content = match get_content(&url, &url_type, response).await {
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
}