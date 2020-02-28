use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::{ast::Infix, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::variable_handler::match_literals::match_obj;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn evaluate(
    infix: &Infix,
    lhs: Result<Literal, ErrorInfo>,
    rhs: Result<Literal, ErrorInfo>,
) -> Result<Literal, ErrorInfo> {
    match (infix, lhs, rhs) {
        (Infix::Equal, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive == rhs.primitive,
            lhs.interval,
        )),
        (Infix::NotEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive != rhs.primitive,
            lhs.interval,
        )),
        (Infix::GreaterThanEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive >= rhs.primitive,
            lhs.interval,
        )),
        (Infix::LessThanEqual, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive <= rhs.primitive,
            lhs.interval,
        )),
        (Infix::GreaterThan, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive > rhs.primitive,
            lhs.interval,
        )),
        (Infix::LessThan, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive < rhs.primitive,
            lhs.interval,
        )),

        (Infix::Addition, Ok(lhs), Ok(rhs)) => {
            let primitive = (lhs.primitive + rhs.primitive)?;
            Ok(Literal {
                content_type: primitive.get_type().to_string(),
                primitive,
                interval: lhs.interval,
            })
        }
        (Infix::Subtraction, Ok(lhs), Ok(rhs)) => {
            let primitive = (lhs.primitive - rhs.primitive)?;

            Ok(Literal {
                content_type: primitive.get_type().to_string(),
                primitive,
                interval: lhs.interval,
            })
        }
        (Infix::Divide, Ok(lhs), Ok(rhs)) => {
            let primitive = (lhs.primitive / rhs.primitive)?;

            Ok(Literal {
                content_type: primitive.get_type().to_string(),
                primitive,
                interval: lhs.interval,
            })
        }

        (Infix::Multiply, Ok(lhs), Ok(rhs)) => {
            let primitive = (lhs.primitive * rhs.primitive)?;
            Ok(Literal {
                content_type: primitive.get_type().to_string(),
                primitive,
                interval: lhs.interval,
            })
        }
        (Infix::Remainder, Ok(lhs), Ok(rhs)) => {
            let primitive = (lhs.primitive % rhs.primitive)?;
            Ok(Literal {
                content_type: primitive.get_type().to_string(),
                primitive,
                interval: lhs.interval,
            })
        }

        (Infix::Or, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive.as_bool() | rhs.primitive.as_bool(),
            lhs.interval,
        )),
        (Infix::And, Ok(lhs), Ok(rhs)) => Ok(PrimitiveBoolean::get_literal(
            lhs.primitive.as_bool() & rhs.primitive.as_bool(),
            lhs.interval,
        )),
        (Infix::Match, Ok(ref lhs), Ok(ref rhs)) => Ok(match_obj(lhs, rhs)),
        // TODO: [+] Handle Infix::NOT as Prefix !
        (Infix::Not, Ok(lhs), ..) => Ok(PrimitiveBoolean::get_literal(
            !lhs.primitive.as_bool(),
            lhs.interval,
        )),
        (_, Err(e), ..) | (.., Err(e)) => Err(e),
    }
}

////////////////////////////////////////////////////////////////////////////////
// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test_operation {
    use crate::data::ast::Interval;
    use crate::data::primitive::{
        array::PrimitiveArray, boolean::PrimitiveBoolean, float::PrimitiveFloat, int::PrimitiveInt,
        null::PrimitiveNull, object::PrimitiveObject, string::PrimitiveString, PrimitiveType,
    };
    use std::collections::HashMap;

    use super::*;

    ////////////////////////////////////////////////////////////////////////////
    // PartialEq
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn equal_array_array() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };

        let literal_0 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_1 = PrimitiveString::get_literal("Hello", interval.to_owned());

        let literal_2 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_3 = PrimitiveString::get_literal("Hello", interval.to_owned());

        let lhs = PrimitiveArray::get_literal(
            &vec![literal_0.to_owned(), literal_1.to_owned()],
            interval.to_owned(),
        );
        let rhs = PrimitiveArray::get_literal(
            &vec![literal_2.to_owned(), literal_3.to_owned()],
            interval.to_owned(),
        );

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_bool_bool() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveBoolean::get_literal(true, interval.to_owned());
        let rhs = PrimitiveBoolean::get_literal(true, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_float_float() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_int_int() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveInt::get_literal(42, interval.to_owned());
        let rhs = PrimitiveInt::get_literal(42, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_null_null() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveNull::get_literal(interval.to_owned());
        let rhs = PrimitiveNull::get_literal(interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_object_object() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };

        let mut map: HashMap<String, Literal> = HashMap::new();

        let literal_0 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_1 = PrimitiveInt::get_literal(42, interval.to_owned());

        let literal_2 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_3 = PrimitiveInt::get_literal(42, interval.to_owned());

        map.insert("literal_0".to_string(), literal_0);
        map.insert("literal_1".to_string(), literal_1);
        map.insert("literal_2".to_string(), literal_2);
        map.insert("literal_3".to_string(), literal_3);

        let lhs = PrimitiveObject::get_literal(&map, interval.to_owned());
        let rhs = PrimitiveObject::get_literal(&map, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_string_string() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("Hello", interval.to_owned());
        let rhs = PrimitiveString::get_literal("Hello", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_int_float() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveInt::get_literal(42, interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_float_int() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());
        let rhs = PrimitiveInt::get_literal(42, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn equal_null_string() {
        let infix = Infix::Equal;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveNull::get_literal(interval.to_owned());
        let rhs = PrimitiveString::get_literal("Hello", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, false);
    }

    ////////////////////////////////////////////////////////////////////////////
    // PartialOrd
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn greather_array_array() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };

        let literal_0 = PrimitiveString::get_literal("b", interval.to_owned());

        let literal_1 = PrimitiveString::get_literal("a", interval.to_owned());

        let lhs = PrimitiveArray::get_literal(&vec![literal_0.to_owned()], interval.to_owned());
        let rhs = PrimitiveArray::get_literal(&vec![literal_1.to_owned()], interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn greather_than_bool_bool_0() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveBoolean::get_literal(true, interval.to_owned());
        let rhs = PrimitiveBoolean::get_literal(true, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, false);
    }

    #[test]
    fn greather_than_bool_bool_1() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveBoolean::get_literal(true, interval.to_owned());
        let rhs = PrimitiveBoolean::get_literal(false, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn greather_float_float() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveFloat::get_literal(42.4, interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn greater_int_int() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveInt::get_literal(43, interval.to_owned());
        let rhs = PrimitiveInt::get_literal(42, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn greather_null_null() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveNull::get_literal(interval.to_owned());
        let rhs = PrimitiveNull::get_literal(interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, false);
    }

    #[test]
    fn greather_object_object() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };

        let mut map: HashMap<String, Literal> = HashMap::new();

        let literal_0 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_1 = PrimitiveInt::get_literal(42, interval.to_owned());

        let literal_2 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_3 = PrimitiveInt::get_literal(42, interval.to_owned());

        map.insert("literal_0".to_string(), literal_0);
        map.insert("literal_1".to_string(), literal_1);
        map.insert("literal_2".to_string(), literal_2);
        map.insert("literal_3".to_string(), literal_3);

        let lhs = PrimitiveObject::get_literal(&map, interval.to_owned());
        let rhs = PrimitiveObject::get_literal(&map, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, false);
    }

    #[test]
    fn greather_string_string() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("b", interval.to_owned());
        let rhs = PrimitiveString::get_literal("a", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn greather_int_float() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveInt::get_literal(43, interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn greather_float_int() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveFloat::get_literal(43.5, interval.to_owned());
        let rhs = PrimitiveInt::get_literal(42, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, true);
    }

    #[test]
    fn greather_null_string() {
        let infix = Infix::GreaterThan;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveNull::get_literal(interval.to_owned());
        let rhs = PrimitiveString::get_literal("Hello", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs))
            .unwrap()
            .primitive
            .as_bool();

        assert_eq!(result, false);
    }

    ////////////////////////////////////////////////////////////////////////////
    // NumericalOps
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn add_array_array() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };

        let literal_0 = PrimitiveString::get_literal("b", interval.to_owned());

        let literal_1 = PrimitiveString::get_literal("a", interval.to_owned());

        let lhs = PrimitiveArray::get_literal(&vec![literal_0.to_owned()], interval.to_owned());
        let rhs = PrimitiveArray::get_literal(&vec![literal_1.to_owned()], interval.to_owned());

        match evaluate(&infix, Ok(lhs), Ok(rhs)) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn add_bool_bool() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveBoolean::get_literal(true, interval.to_owned());
        let rhs = PrimitiveBoolean::get_literal(true, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<i64>(&result.primitive).unwrap();

        assert_eq!(*result, 2);
    }

    #[test]
    fn add_float_float() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(42.0, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 84.0);
    }

    #[test]
    fn add_int_int() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveInt::get_literal(42, interval.to_owned());
        let rhs = PrimitiveInt::get_literal(42, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<i64>(&result.primitive).unwrap();

        assert_eq!(*result, 84);
    }

    #[test]
    fn add_null_null() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveNull::get_literal(interval.to_owned());
        let rhs = PrimitiveNull::get_literal(interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();

        if result.primitive.get_type() == PrimitiveType::PrimitiveNull {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn add_object_object() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };

        let mut map: HashMap<String, Literal> = HashMap::new();

        let literal_0 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_1 = PrimitiveInt::get_literal(42, interval.to_owned());

        let literal_2 = PrimitiveString::get_literal("Hello", interval.to_owned());
        let literal_3 = PrimitiveInt::get_literal(42, interval.to_owned());

        map.insert("literal_0".to_string(), literal_0);
        map.insert("literal_1".to_string(), literal_1);
        map.insert("literal_2".to_string(), literal_2);
        map.insert("literal_3".to_string(), literal_3);

        let lhs = PrimitiveObject::get_literal(&map, interval.to_owned());
        let rhs = PrimitiveObject::get_literal(&map, interval.to_owned());

        match evaluate(&infix, Ok(lhs), Ok(rhs)) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn add_int_float() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveInt::get_literal(2, interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(42.4, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_float_int() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveFloat::get_literal(42.4, interval.to_owned());
        let rhs = PrimitiveInt::get_literal(2, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_null_string() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveNull::get_literal(interval.to_owned());
        let rhs = PrimitiveString::get_literal("Hello", interval.to_owned());

        match evaluate(&infix, Ok(lhs), Ok(rhs)) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn add_int_string_int() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42", interval.to_owned());
        let rhs = PrimitiveInt::get_literal(2, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<i64>(&result.primitive).unwrap();

        assert_eq!(*result, 44);
    }

    #[test]
    fn add_float_string_int() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42.4", interval.to_owned());
        let rhs = PrimitiveInt::get_literal(2, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_err_string_int() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("yolo", interval.to_owned());
        let rhs = PrimitiveInt::get_literal(2, interval.to_owned());

        match evaluate(&infix, Ok(lhs), Ok(rhs)) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn add_int_string_float() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42", interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(2.4, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_float_string_float() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42.4", interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(2.0, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_err_string_float() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("yolo", interval.to_owned());
        let rhs = PrimitiveFloat::get_literal(2.2, interval.to_owned());

        match evaluate(&infix, Ok(lhs), Ok(rhs)) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn add_int_string_int_1() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let rhs = PrimitiveString::get_literal("42", interval.to_owned());
        let lhs = PrimitiveInt::get_literal(2, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<i64>(&result.primitive).unwrap();

        assert_eq!(*result, 44);
    }

    #[test]
    fn add_float_string_int_1() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let rhs = PrimitiveString::get_literal("42.4", interval.to_owned());
        let lhs = PrimitiveInt::get_literal(2, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_err_string_int_1() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let rhs = PrimitiveString::get_literal("yolo", interval.to_owned());
        let lhs = PrimitiveInt::get_literal(2, interval.to_owned());

        match evaluate(&infix, Ok(lhs), Ok(rhs)) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn add_int_string_float_1() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let rhs = PrimitiveString::get_literal("42", interval.to_owned());
        let lhs = PrimitiveFloat::get_literal(2.4, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_float_string_float_1() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let rhs = PrimitiveString::get_literal("42.4", interval.to_owned());
        let lhs = PrimitiveFloat::get_literal(2.0, interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_err_string_float_1() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let rhs = PrimitiveString::get_literal("yolo", interval.to_owned());
        let lhs = PrimitiveFloat::get_literal(2.2, interval.to_owned());

        match evaluate(&infix, Ok(lhs), Ok(rhs)) {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn add_string_string_int_int() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42", interval.to_owned());
        let rhs = PrimitiveString::get_literal("2", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<i64>(&result.primitive).unwrap();

        assert_eq!(*result, 44);
    }

    #[test]
    fn add_string_string_float_float() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42.0", interval.to_owned());
        let rhs = PrimitiveString::get_literal("2.4", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_string_string_int_float() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42", interval.to_owned());
        let rhs = PrimitiveString::get_literal("2.4", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }

    #[test]
    fn add_string_string_float_int() {
        let infix = Infix::Addition;
        let interval = Interval { column: 0, line: 0 };
        let lhs = PrimitiveString::get_literal("42.4", interval.to_owned());
        let rhs = PrimitiveString::get_literal("2", interval.to_owned());

        let result = evaluate(&infix, Ok(lhs), Ok(rhs)).unwrap();
        let result = Literal::get_value::<f64>(&result.primitive).unwrap();

        assert_eq!(*result, 44.4);
    }
}
