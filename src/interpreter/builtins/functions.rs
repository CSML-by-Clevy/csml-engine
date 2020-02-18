use crate::data::primitive::{
    array::PrimitiveArray, boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt,
};
use crate::data::{ast::Interval, tokens::*, Literal};
use crate::error_format::ErrorInfo;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn one_of(
    args: HashMap<String, Literal>,
    one_of_inter: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                Ok(res) => {
                    match res.get(rand::thread_rng().gen_range(0, res.len())) {
                        Some(lit) => Ok(lit.to_owned()),
                        None => Err(ErrorInfo{
                            message: "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )".to_owned(),
                            interval: literal.interval,
                        })
                    }
                }
                Err(_) => Err(ErrorInfo{
                        message: "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )".to_owned(),
                        interval: literal.interval,
                    })
            }
        }
        None => Err(ErrorInfo {
            message:
                "ERROR: Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )"
                    .to_owned(),
            interval: one_of_inter,
        }),
    }
}

pub fn shuffle(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                Ok(res) => {
                    let mut vec = res.to_owned();
                    vec.shuffle(&mut rand::thread_rng());

                    Ok(PrimitiveArray::get_literal("array", &vec, literal.interval))
                }
                Err(_) => Err(ErrorInfo{
                        message: "ERROR: Builtin Shuffle expect one value of type Array | example: Shuffle( [1, 2, 3] )".to_owned(),
                        interval
                })
            }
        }
        None => Err(ErrorInfo{
                message: "ERROR: Builtin Shuffle expect one value of type Array | example: Shuffle( [1, 2, 3] )".to_owned(),
                interval
        })
    }
}

pub fn length(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            if let Ok(res) = Literal::get_value::<Vec<Literal>>(&literal.primitive) {
                return Ok(PrimitiveInt::get_literal("int", res.len() as i64, literal.interval))
            }
            if let Ok(res) = Literal::get_value::<String>(&literal.primitive) {
                return Ok(PrimitiveInt::get_literal("int", res.len() as i64, literal.interval))
            }

            Err(ErrorInfo{
                message: "ERROR: Builtin Length expect one value of type Array or String | example: Length( value )".to_owned(),
                interval
            })
        }
        None => Err(ErrorInfo{
            message: "ERROR: Builtin Length expect one value of type Array or String | example: Length( value )".to_owned(),
            interval
        })
    }
}

pub fn find(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut string = None;
    let mut case = false;

    if let Some(literal) = args.get("in") {
        if let Ok(res) = Literal::get_value::<String>(&literal.primitive) {
            string = Some(res);
        }
    } else if string.is_none() {
        return Err(ErrorInfo{
            message: "ERROR: Builtin Find expect in to be of type String | example: Contain(value, in = \"hola\", case_sensitive = true)".to_owned(),
            interval
        });
    }

    if let Some(literal) = args.get("in") {
        if let Ok(res) = Literal::get_value::<bool>(&literal.primitive) {
            case = *res;
        }
    }

    match (args.get(DEFAULT), string) {
        (Some(literal), Some(string)) => {
            match Literal::get_value::<String>(&literal.primitive) {
                Ok(res) => {
                    if case {
                        Ok(PrimitiveBoolean::get_literal("boolean", string.contains(res), interval))
                    } else {
                        Ok(PrimitiveBoolean::get_literal("boolean", string.to_lowercase().contains(&res.to_lowercase()), interval))
                    }
                }
                Err(_) => Err(ErrorInfo{
                    message: "ERROR: Builtin Find expect value to be of type String | example: Find(value, in = \"hola\", case_sensitive = true)".to_owned(),
                    interval
                })
            }
        }
        (_, _) => Err(ErrorInfo{
            message: "ERROR: Builtin Find expect value to be of type String | example: Find(value, in = \"hola\", case_sensitive = true)".to_owned(),
            interval
        })
    }
}

pub fn random(interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut rng = rand::thread_rng();

    let random: f64 = rng.gen();

    Ok(PrimitiveFloat::get_literal("float", random, interval))
}

pub fn floor(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => match Literal::get_value::<f64>(&literal.primitive) {
            Ok(res) => Ok(PrimitiveFloat::get_literal(
                "float",
                res.floor(),
                literal.interval,
            )),
            Err(_) => Err(ErrorInfo {
                message:
                    "ERROR: Builtin Floor expect one argument of type float| example: Floor(4.2)"
                        .to_owned(),
                interval,
            }),
        },
        _ => Err(ErrorInfo {
            message: "ERROR: Builtin Floor expect one argument of type float| example: Floor(4.2)"
                .to_owned(),
            interval,
        }),
    }
}
