use crate::data::{
    Literal, primitive::PrimitiveType,
};
use std::{collections::HashMap};


////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_date(args: &HashMap<String, Literal>) -> [i64; 7] {

    let mut date: [i64; 7] = [0; 7];

    // set default month, day, and hour to 1 year does not need to have a default
    // value because set_date_at expect at least the year value as parameter
    date[1] = 1; // month
    date[2] = 1; // day
    date[3] = 1; // hour

    let len = args.len();

    for index in 0..len {
        match args.get(&format!("arg{}", index)) {
            Some(lit) if lit.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                let value = serde_json::from_str(&lit.primitive.to_string()).unwrap();

                date[index] = value;
            },
            _ => {},
        }

    }

    date
}
