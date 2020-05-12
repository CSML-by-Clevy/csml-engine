use crate::data::primitive::{
    array::PrimitiveArray, boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt,
};
use crate::data::{ast::Interval, tokens::*, Literal};
use crate::error_format::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use crate::data::position::Position;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn one_of(
    args: HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            let res = Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
                literal.interval,
                ERROR_ONE_OF.to_owned(),
            )?;
            match res.get(rand::thread_rng().gen_range(0, res.len())) {
                Some(lit) => Ok(lit.to_owned()),
                None => Err(gen_error_info(Position::new(literal.interval), ERROR_ONE_OF.to_owned())),
            }
        }
        None => Err(gen_error_info(Position::new(interval), ERROR_ONE_OF.to_owned())),
    }
}

pub fn shuffle(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            let res = Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
                interval,
                ERROR_SHUFFLE.to_owned(),
            )?;
            let mut vec = res.to_owned();
            vec.shuffle(&mut rand::thread_rng());
            Ok(PrimitiveArray::get_literal(&vec, literal.interval))
        }
        None => Err(gen_error_info(Position::new(interval), ERROR_SHUFFLE.to_owned())),
    }
}

pub fn length(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            if let Ok(res) = Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
                interval,
                ERROR_LENGTH.to_owned(),
            ) {
                return Ok(PrimitiveInt::get_literal(
                    res.len() as i64,
                    literal.interval,
                ));
            }
            if let Ok(res) =
                Literal::get_value::<String>(&literal.primitive, interval, ERROR_LENGTH.to_owned())
            {
                return Ok(PrimitiveInt::get_literal(
                    res.len() as i64,
                    literal.interval,
                ));
            }

            Err(gen_error_info(Position::new(interval), ERROR_LENGTH.to_owned()))
        }
        None => Err(gen_error_info(Position::new(interval), ERROR_LENGTH.to_owned())),
    }
}

pub fn find(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut string = None;
    let mut case = false;

    if let Some(literal) = args.get("in") {
        if let Ok(res) =
            Literal::get_value::<String>(&literal.primitive, interval, ERROR_FIND.to_owned())
        {
            string = Some(res);
        }
    } else if string.is_none() {
        return Err(gen_error_info(Position::new(interval), ERROR_FIND.to_owned()));
    }

    if let Some(literal) = args.get("in") {
        if let Ok(res) =
            Literal::get_value::<bool>(&literal.primitive, interval, ERROR_FIND.to_owned())
        {
            case = *res;
        }
    }

    match (args.get(DEFAULT), string) {
        (Some(literal), Some(string)) => {
            let res =
                Literal::get_value::<String>(&literal.primitive, interval, ERROR_FIND.to_owned())?;
            if case {
                Ok(PrimitiveBoolean::get_literal(
                    string.contains(res),
                    interval,
                ))
            } else {
                Ok(PrimitiveBoolean::get_literal(
                    string.to_lowercase().contains(&res.to_lowercase()),
                    interval,
                ))
            }
        }
        (_, _) => Err(gen_error_info(Position::new(interval), ERROR_FIND.to_owned())),
    }
}

pub fn random(interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut rng = rand::thread_rng();

    let random: f64 = rng.gen();

    Ok(PrimitiveFloat::get_literal(random, interval))
}

pub fn floor(args: HashMap<String, Literal>, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get(DEFAULT) {
        Some(literal) => {
            let res =
                Literal::get_value::<f64>(&literal.primitive, interval, ERROR_FLOOR.to_owned())?;
            Ok(PrimitiveFloat::get_literal(res.floor(), literal.interval))
        }
        _ => Err(gen_error_info(Position::new(interval), ERROR_FLOOR.to_owned())),
    }
}
