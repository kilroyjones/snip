use anyhow::anyhow;
use image::imageops::FilterType;
use image::EncodableLayout;
use std::fs::File;
use std::io::Write;

async fn write_image_to_disk(
    base: &String,
    content: actix_web::web::Bytes,
    filename: &String,
) -> Result<(), anyhow::Error> {
    // use std::env;
    // println!("{}", env::current_dir().unwrap().to_string_lossy());
    let path = format!("{}/original/{}", base, filename);
    // println!("{:?}", path);
    let mut original = File::create(&path).unwrap();
    let _ = original.write_all(&content); // CATCH ERROR ON THIS
    let mut thumbnail = image::io::Reader::open(format!("{}/original/{}", base, filename))
        .unwrap()
        .decode()
        .unwrap();
    thumbnail = thumbnail.resize(200, 200, FilterType::Lanczos3);
    thumbnail
        .save(format!("{}/thumbnail/{}", base, filename))
        .unwrap();
    Ok(())
}

async fn write_webp_image_to_disk(
    base: &String,
    content: actix_web::web::Bytes,
    filename: &String,
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
        Err(_) => return Err(anyhow!("".to_string())),
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
