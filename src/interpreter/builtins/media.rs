use crate::data::primitive::{PrimitiveObject, PrimitiveType};
use crate::data::{ast::Interval, tokens::DEFAULT, Literal};
use crate::error_format::*;
use std::collections::HashMap;

pub fn url(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut url: HashMap<String, Literal> = HashMap::new();

    match (args.get("url"), args.get(DEFAULT)) {
        (Some(href), ..) | (.., Some(href))
            if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            url.insert("url".to_owned(), href.clone());

            match args.get("title") {
                Some(title) => url.insert("title".to_owned(), title.to_owned()),
                None => url.insert("title".to_owned(), href.clone()),
            };

            match args.get("text") {
                Some(text) => url.insert("text".to_owned(), text.to_owned()),
                _ => url.insert("text".to_owned(), href.clone()),
            };

            let mut result = PrimitiveObject::get_literal(&url, interval);
            result.set_content_type("url");

            Ok(result)
        }
        _ => Err(gen_error_info(interval, ERROR_URL.to_owned())),
    }
}

pub fn img(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut img: HashMap<String, Literal> = HashMap::new();

    match (args.get("url"), args.get(DEFAULT)) {
        (Some(href), ..) | (.., Some(href))
            if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            img.insert("url".to_owned(), href.clone());

            let mut result = PrimitiveObject::get_literal(&img, interval);
            result.set_content_type("image");

            Ok(result)
        }
        _ => Err(gen_error_info(interval, ERROR_IMAGE.to_owned())),
    }
}

pub fn video(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut video: HashMap<String, Literal> = HashMap::new();

    match (args.get("url"), args.get(DEFAULT)) {
        (Some(href), ..) | (.., Some(href))
            if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            video.insert("url".to_owned(), href.clone());
            match args.get("service") {
                Some(value) if value.primitive.get_type() == PrimitiveType::PrimitiveString => {
                    video.insert("service".to_owned(), value.to_owned())
                }
                _ => None,
            };

            let mut result = PrimitiveObject::get_literal(&video, interval);
            result.set_content_type("video");

            Ok(result)
        }
        _ => Err(gen_error_info(interval, ERROR_VIDEO.to_owned())),
    }
}

pub fn audio(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut audio: HashMap<String, Literal> = HashMap::new();

    match (args.get("url"), args.get(DEFAULT)) {
        (Some(href), ..) | (.., Some(href))
            if href.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            audio.insert("url".to_owned(), href.clone());

            match args.get("service") {
                Some(value) if value.primitive.get_type() == PrimitiveType::PrimitiveString => {
                    audio.insert("service".to_owned(), value.to_owned())
                }
                _ => None,
            };

            let mut result = PrimitiveObject::get_literal(&audio, interval);
            result.set_content_type("audio");

            Ok(result)
        }
        _ => Err(gen_error_info(interval, ERROR_AUDIO.to_owned())),
    }
}
