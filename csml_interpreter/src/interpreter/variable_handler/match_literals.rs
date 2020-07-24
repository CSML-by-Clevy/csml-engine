// use crate::error_format::ErrorInfo;
use crate::data::primitive::{PrimitiveArray, PrimitiveObject, PrimitiveString};
use crate::data::Literal;

fn get_accept(lit: &Literal) -> Option<&Literal> {
    let val = lit
        .primitive
        .as_any()
        .downcast_ref::<PrimitiveObject>()?
        .value
        .get("accepts")?;
    Some(val)
}

fn contains(array_lit: &Literal, key: &Literal) -> bool {
    let key_string = key.primitive.as_any().downcast_ref::<PrimitiveString>();
    match (
        array_lit
            .primitive
            .as_any()
            .downcast_ref::<PrimitiveArray>(),
        key_string,
    ) {
        (Some(array), None) => array.value.contains(key),
        (Some(array), Some(string)) => {
            for elem in array.value.iter() {
                match elem.primitive.as_any().downcast_ref::<PrimitiveString>() {
                    Some(val)
                        if val.value.to_ascii_lowercase() == string.value.to_ascii_lowercase() =>
                    {
                        return true
                    }
                    _ => continue,
                }
            }
            false
        }
        (None, ..) => false,
    }
}

pub fn match_obj(lit1: &Literal, lit2: &Literal) -> bool {
    match (&lit1.content_type, &lit2.content_type) {
        (b1, b2) if (b1 == "button" || b1 == "object") && (b2 == "button" || b2 == "object") => {
            match (get_accept(lit1), get_accept(lit2)) {
                (Some(l1), Some(l2)) => match_obj(l1, l2),
                (_, _) => false,
            }
        }

        (.., button) if (button == "button" || button == "object") => match get_accept(lit2) {
            Some(l2) => match_obj(lit1, l2),
            None => false,
        },
        (button, ..) if (button == "button" || button == "object") => match get_accept(lit1) {
            Some(l1) => match_obj(l1, lit2),
            None => false,
        },

        (array1, array2) if array1 == "array" && array2 == "array" => lit1 == lit2,
        (.., array) if array == "array" => contains(lit2, lit1),
        (array, ..) if array == "array" => contains(lit1, lit2),
        (..) => &lit1.primitive == &lit2.primitive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::primitive::array::PrimitiveArray;
    use crate::data::primitive::string::PrimitiveString;
    use crate::data::{ast::Interval, ArgsType};
    use crate::interpreter::{
        builtins::components::read, variable_handler::gen_generic_component::gen_generic_component,
    };
    use std::collections::HashMap;

    fn gen_inter() -> Interval {
        Interval { line: 0, column: 0 }
    }

    fn gen_button(name: &str) -> Literal {
        let mut map = HashMap::new();
        let interval = gen_inter();

        map.insert(
            "title".to_owned(),
            PrimitiveString::get_literal(name, interval),
        );

        let native_component = read().unwrap();

        if let Some(component) = native_component.get("Button") {
            match gen_generic_component("Button", &interval, &ArgsType::Named(map), component) {
                Ok(lit) => lit,
                Err(..) => panic!("gen button error"),
            }
        } else {
            panic!("error in native_component")
        }
    }

    fn gen_button_multi_accept(name: &str) -> Literal {
        let mut map = HashMap::new();
        let interval = gen_inter();

        map.insert(
            "title".to_owned(),
            PrimitiveString::get_literal(name, interval),
        );
        map.insert(
            "accepts".to_owned(),
            PrimitiveArray::get_literal(
                &vec![
                    PrimitiveString::get_literal("toto", interval),
                    PrimitiveString::get_literal("plop", interval),
                    PrimitiveString::get_literal("TEST", interval),
                ],
                gen_inter(),
            ),
        );

        let native_component = read().unwrap();

        if let Some(component) = native_component.get("Button") {
            match gen_generic_component("Button", &interval, &ArgsType::Named(map), component) {
                Ok(lit) => lit,
                Err(..) => panic!("gen button error"),
            }
        } else {
            panic!("error in native_component")
        }
    }

    fn match_lit_true(lit1: &Literal, lit2: &Literal) {
        match match_obj(&lit1, &lit2) {
            boolean if boolean => {}
            _ => panic!("\n\nlit1: {:?}\n\n lit2: {:?}\n", lit1, lit2),
        }
    }

    fn match_lit_false(lit1: &Literal, lit2: &Literal) {
        match match_obj(&lit1, &lit2) {
            boolean if !boolean => {}
            _ => panic!("\n\nlit1: {:?}\n\n lit2: {:?}\n", lit1, lit2),
        }
    }

    fn match_lit_err(lit1: &Literal, lit2: &Literal) {
        match match_obj(&lit1, &lit2) {
            boolean if boolean => panic!("\n\n lit1: {:#?}\n\n lit2: {:#?}\n", lit1, lit2),
            _ => {}
        }
    }

    #[test]
    fn ok_match_button_button() {
        let bt1 = gen_button("hola");
        let bt2 = gen_button("hola");

        match_lit_true(&bt1, &bt2);
    }

    #[test]
    fn ok_match_array_str() {
        let bt1 = PrimitiveArray::get_literal(
            &[PrimitiveString::get_literal("hola", gen_inter())],
            gen_inter(),
        );
        let bt2 = PrimitiveString::get_literal("hola", gen_inter());

        match_lit_true(&bt1, &bt2);
        match_lit_true(&bt2, &bt1);
    }

    #[test]
    fn ok_match_button_str() {
        let bt1 = gen_button("hola");
        let bt2 = PrimitiveString::get_literal("hola", gen_inter());

        match_lit_true(&bt1, &bt2);
        match_lit_true(&bt2, &bt1);
    }

    #[test]
    fn ok_match_button_str2() {
        let bt1 = gen_button_multi_accept("hola");
        let bt2 = PrimitiveString::get_literal("toTo", gen_inter());

        match_lit_true(&bt1, &bt2);
        match_lit_true(&bt2, &bt1);
    }

    #[test]
    fn ok_match_button_str3() {
        let bt1 = gen_button_multi_accept("hola");
        let bt2 = PrimitiveString::get_literal("test", gen_inter());

        match_lit_true(&bt1, &bt2);
        match_lit_true(&bt2, &bt1);
    }

    #[test]
    fn ok_not_match_button_str() {
        let bt1 = gen_button("hola");
        let bt2 = PrimitiveString::get_literal("not hola", gen_inter());

        match_lit_false(&bt1, &bt2);
        match_lit_false(&bt2, &bt1);
    }

    #[test]
    fn err_match_button_button() {
        let bt1 = gen_button("hola");
        let bt2 = gen_button("nop");

        match_lit_err(&bt1, &bt2);
    }
}
