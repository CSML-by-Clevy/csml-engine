use crate::data::{ast::Interval, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::builtins::tools::*;
use crate::data::primitive::{PrimitiveObject, PrimitiveType};
use std::collections::HashMap;

pub fn url(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut url: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
         Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {

            url.insert("url".to_owned(), href.clone());

            if let Ok(title) = search_or_default(&args, "title", interval, Some(href.clone())) {
                url.insert("title".to_owned(), title);
            }
            if let Ok(text) = search_or_default(&args, "text", interval, Some(href.clone())) {
                url.insert("text".to_owned(), text);
            }

            Ok(PrimitiveObject::get_literal("url", &url, interval))
        },
        _ => Err(ErrorInfo{
                message: "Builtin Url expect one argument of type string and 2 optional string argmuments: text, title | example: Url(href = \"hola\", text = \"text\", title = \"title\")".to_owned(),
                interval
        })
    }
}

pub fn img(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut img: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
        Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {
            img.insert("url".to_owned(), href.clone());

            Ok(PrimitiveObject::get_literal("image", &img, interval))
        }
        _ => Err(ErrorInfo {
            message: "Builtin Image expect one argument of type string | example: Image(\"hola\")"
                .to_owned(),
            interval,
        }),
    }
}

pub fn video(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut video: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
        Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {

            video.insert("url".to_owned(), href.clone());
            match args.get("service") {
                Some(value) if value.primitive.get_type() == PrimitiveType::PrimitiveString => {
                    video.insert("service".to_owned(), value.to_owned())
                }
                _ => None
            };

            Ok(PrimitiveObject::get_literal("video", &video, interval))
        },
        _ => Err(ErrorInfo{
                message: "Builtin Video expect one argument of type string and 1 optional 'service' argument of type string | example: Video(url = \"hola\", service = \"text\")".to_owned(),
                interval
        })
    }
}

pub fn audio(
    args: HashMap<String, Literal>,
    _name: String,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut audio: HashMap<String, Literal> = HashMap::new();

    match &search_or_default(&args, "url", interval, None) {
        Ok(href) if href.primitive.get_type() == PrimitiveType::PrimitiveString => {

            audio.insert("url".to_owned(), href.clone());

            match args.get("service") {
                Some(value) if value.primitive.get_type() == PrimitiveType::PrimitiveString => {
                    audio.insert("service".to_owned(), value.to_owned())
                }
                _ => None
            };

            Ok(PrimitiveObject::get_literal("audio", &audio, interval))

        },
        _ => Err(ErrorInfo{
                message: "Builtin Audio expect one argument of type string and 1 optional 'service' argument of type string | example: Audio(url = \"hola\", service = \"text\")".to_owned(),
                interval
        })
    }
}
