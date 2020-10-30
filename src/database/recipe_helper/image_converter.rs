extern crate rustc_serialize;
// use crate::error::*;

use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::ToHex;

pub fn get_file_type(hex: &str) -> &str {
    match &hex[..8] {
        r"ffd8ffe0" => "jpeg",
        r"ffd8ffe1" => "jpeg",
        r"89504e47" => "png",
        r"47494638" => "gif",
        _ => panic!("invalid file type"),
    }
}

pub fn imageurl_to_base64(image_url: &str) -> String {
    let result = reqwest::blocking::get(image_url).unwrap().bytes().unwrap();

    let base64 = result.to_base64(MIME);
    let hex = result.to_hex();

    format!(
        "data:image/{};base64,{}",
        get_file_type(&hex),
        base64.replace("\r\n", "")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    //     // #[test]
    //     fn test_load_image_png() {
    //         let test_image = load_image("https://via.placeholder.com/5.png").unwrap();
    //         assert_eq!(test_image.content_type, rocket::http::ContentType::PNG);
    //     }
    // #[test]
    // fn test_load_image_jpg() {
    //     let test_image = imageurl_to_base64("https://via.placeholder.com/5.jpg");

    //     // assert_eq!(test_image.content_type, rocket::http::ContentType::JPEG);
    // }
}
