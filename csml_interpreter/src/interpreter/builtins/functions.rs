use crate::data::position::Position;
use crate::data::primitive::{
    PrimitiveArray, PrimitiveBoolean, PrimitiveFloat, PrimitiveInt, PrimitiveString,
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

use rand::seq::SliceRandom;
use rand::Rng;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn one_of(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("array", 0) {
        Some(literal) => {
            let res = Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
                flow_name,
                literal.interval,
                ERROR_ONE_OF.to_owned(),
            )?;
            match res.get(rand::thread_rng().gen_range(0..res.len())) {
                Some(lit) => Ok(lit.to_owned()),
                None => Err(gen_error_info(
                    Position::new(literal.interval, flow_name),
                    ERROR_ONE_OF.to_owned(),
                )),
            }
        }
        None => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_ONE_OF.to_owned(),
        )),
    }
}

pub fn or(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match (args.get("arg0", 0), args.get("arg1", 1)) {
        (Some(first_val), Some(optional_value)) => {
            match &first_val.additional_info {
                Some(map) if map.contains_key("error") => {
                    Ok(optional_value.to_owned())
                }
                _ => Ok(first_val.to_owned())
            }
        }
        _ => {
            Err(gen_error_info(
                Position::new(interval, flow_name),
                "ERROR_SHUFFLE".to_owned(),
            ))
        }
    }
}

pub fn shuffle(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("array", 0) {
        Some(literal) => {
            let res = Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
                flow_name,
                interval,
                ERROR_SHUFFLE.to_owned(),
            )?;
            let mut vec = res.to_owned();
            vec.shuffle(&mut rand::thread_rng());
            Ok(PrimitiveArray::get_literal(&vec, literal.interval))
        }
        None => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_SHUFFLE.to_owned(),
        )),
    }
}

pub fn length(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("length", 0) {
        Some(literal) => {
            if let Ok(res) = Literal::get_value::<Vec<Literal>>(
                &literal.primitive,
                flow_name,
                interval,
                ERROR_LENGTH.to_owned(),
            ) {
                return Ok(PrimitiveInt::get_literal(
                    res.len() as i64,
                    literal.interval,
                ));
            }
            if let Ok(res) =
                Literal::get_value::<String>(&literal.primitive, flow_name, interval, ERROR_LENGTH.to_owned())
            {
                return Ok(PrimitiveInt::get_literal(
                    res.len() as i64,
                    literal.interval,
                ));
            }

            Err(gen_error_info(
                Position::new(interval, flow_name),
                ERROR_LENGTH.to_owned(),
            ))
        }
        None => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_LENGTH.to_owned(),
        )),
    }
}

pub fn find(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut string = None;
    let mut case = false;

    if let Some(literal) = args.get("in", 1) {
        if let Ok(res) =
            Literal::get_value::<String>(&literal.primitive, flow_name, interval, ERROR_FIND.to_owned())
        {
            string = Some(res);
        }
    } else if string.is_none() {
        return Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_FIND.to_owned(),
        ));
    }

    if let Some(literal) = args.get("in", 1) {
        if let Ok(res) =
            Literal::get_value::<bool>(&literal.primitive, flow_name, interval, ERROR_FIND.to_owned())
        {
            case = *res;
        }
    }

    match (args.get("value", 0), string) {
        (Some(literal), Some(string)) => {
            let res =
                Literal::get_value::<String>(&literal.primitive, flow_name, interval, ERROR_FIND.to_owned())?;
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
        (_, _) => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_FIND.to_owned(),
        )),
    }
}

pub fn random(interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut rng = rand::thread_rng();

    let random: f64 = rng.gen();

    Ok(PrimitiveFloat::get_literal(random, interval))
}

pub fn floor(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("float", 0) {
        Some(literal) => {
            let res =
                Literal::get_value::<f64>(&literal.primitive, flow_name, interval, ERROR_FLOOR.to_owned())?;
            Ok(PrimitiveFloat::get_literal(res.floor(), literal.interval))
        }
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_FLOOR.to_owned(),
        )),
    }
}

pub fn uuid_command(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    if args.len() == 0 {
        return Ok(PrimitiveString::get_literal(
            &Uuid::new_v4().to_string(),
            interval,
        ));
    }

    match args.get("value", 0) {
        Some(literal) => {
            let arg =
                Literal::get_value::<String>(&literal.primitive, flow_name, interval, ERROR_FLOOR.to_owned())?;

            match arg {
                arg if arg == "v1" => {
                    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
                    let context = Context::new(rand::thread_rng().gen());
                    let ts = Timestamp::from_unix(&context, time.as_secs(), time.subsec_nanos());

                    let node_id = &[
                        rand::thread_rng().gen(),
                        rand::thread_rng().gen(),
                        rand::thread_rng().gen(),
                        rand::thread_rng().gen(),
                        rand::thread_rng().gen(),
                        rand::thread_rng().gen(),
                    ];
                    Ok(PrimitiveString::get_literal(
                        &Uuid::new_v1(ts, node_id)?.to_string(),
                        interval,
                    ))
                }
                arg if arg == "v4" => Ok(PrimitiveString::get_literal(
                    &Uuid::new_v4().to_string(),
                    interval,
                )),
                _ => Err(gen_error_info(
                    Position::new(interval, flow_name),
                    ERROR_UUID.to_owned(),
                )),
            }
        }
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_UUID.to_owned(),
        )),
    }
}
