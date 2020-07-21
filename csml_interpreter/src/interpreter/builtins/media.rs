// use crate::data::position::Position;
// use crate::data::primitive::{PrimitiveObject, PrimitiveType};
// use crate::data::{ast::Interval, tokens::DEFAULT, Literal, ArgsType};
// use crate::error_format::*;
// use std::collections::HashMap;

// pub fn url(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
//     let mut url: HashMap<String, Literal> = args.clone();

//     match (url.remove("url"), url.remove(DEFAULT)) {
//         (Some(href), ..) | (.., Some(href))
//             if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
//         {
//             url.insert("url".to_owned(), href.clone());

//             match args.get("title") {
//                 Some(title) => url.insert("title".to_owned(), title.to_owned()),
//                 None => url.insert("title".to_owned(), href.clone()),
//             };

//             match args.get("text") {
//                 Some(text) => url.insert("text".to_owned(), text.to_owned()),
//                 _ => url.insert("text".to_owned(), href.clone()),
//             };

//             let mut result = PrimitiveObject::get_literal(&url, interval);
//             result.set_content_type("url");

//             Ok(result)
//         }
//         _ => Err(gen_error_info(
//             Position::new(interval),
//             ERROR_URL.to_owned(),
//         )),
//     }
// }

// pub fn img(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
//     let mut img: HashMap<String, Literal> = args.clone();

//     match (img.remove("url"), img.remove(DEFAULT)) {
//         (Some(href), ..) | (.., Some(href))
//             if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
//         {
//             img.insert("url".to_owned(), href.clone());

//             let mut result = PrimitiveObject::get_literal(&img, interval);
//             result.set_content_type("image");

//             Ok(result)
//         }
//         _ => Err(gen_error_info(
//             Position::new(interval),
//             ERROR_IMAGE.to_owned(),
//         )),
//     }
// }

// pub fn video(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
//     let mut video: HashMap<String, Literal> = args.clone();

//     match (video.remove("url"), video.remove(DEFAULT)) {
//         (Some(href), ..) | (.., Some(href))
//             if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
//         {
//             video.insert("url".to_owned(), href.clone());

//             let mut result = PrimitiveObject::get_literal(&video, interval);

//             result.set_content_type("video");

//             Ok(result)
//         }
//         _ => Err(gen_error_info(
//             Position::new(interval),
//             ERROR_VIDEO.to_owned(),
//         )),
//     }
// }

// pub fn audio(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
//     let mut audio: HashMap<String, Literal> = args.clone();

//     match (audio.remove("url"), audio.remove(DEFAULT)) {
//         (Some(href), ..) | (.., Some(href))
//             if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
//         {
//             audio.insert("url".to_owned(), href.clone());

//             let mut result = PrimitiveObject::get_literal(&audio, interval);

//             result.set_content_type("audio");

//             Ok(result)
//         }
//         _ => Err(gen_error_info(
//             Position::new(interval),
//             ERROR_AUDIO.to_owned(),
//         )),
//     }
// }

// pub fn file(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
//     let mut file: HashMap<String, Literal> = args.clone();

//     match (file.remove("url"), file.remove(DEFAULT)) {
//         (Some(href), ..) | (.., Some(href))
//             if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
//         {
//             file.insert("url".to_owned(), href.clone());

//             let mut result = PrimitiveObject::get_literal(&file, interval);

//             result.set_content_type("file");

//             Ok(result)
//         }
//         _ => Err(gen_error_info(
//             Position::new(interval),
//             ERROR_FILE.to_owned(),
//         )),
//     }
// }
