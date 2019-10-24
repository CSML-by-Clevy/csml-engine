use crate::parser::literal::Literal;

fn priority_match<'a>(name: &str, lit: &'a Literal) -> Option<&'a Literal> {
    match name {
        "button" => {
            if let Literal::ObjectLiteral { properties, .. } = lit {
                properties.get("accept")
            } else {
                Some(lit)
            }
        }
        _ => Some(lit),
    }
}

//TODO: Refactor with macros
pub fn match_obj(lit1: &Literal, lit2: &Literal) -> Literal {
    match (&lit1, &lit2) {
        (
            Literal::ObjectLiteral {
                properties: p1,
                interval,
            },
            Literal::ObjectLiteral { properties: p2, .. },
        ) => match (p1.get("button"), p2.get("button")) {
            (Some(l1), Some(l2)) => {
                match (priority_match("button", l1), priority_match("button", l2)) {
                    (Some(l1), Some(l2)) => match_obj(l1, l2),
                    (_, _) => Literal::boolean(false, interval.to_owned()),
                }
            }
            (_, _) => Literal::boolean(false, interval.to_owned()),
        },
        (
            Literal::ObjectLiteral {
                properties,
                interval,
            },
            lit,
        ) => match properties.get("button") {
            Some(l1) => match priority_match("button", l1) {
                Some(l1) => match_obj(l1, lit),
                _ => Literal::boolean(false, interval.to_owned()),
            },
            _ => Literal::boolean(false, interval.to_owned()),
        },
        (lit, Literal::ObjectLiteral { properties, .. }) => match properties.get("button") {
            Some(l2) => match priority_match("button", l2) {
                Some(l2) => match_obj(l2, lit),
                _ => Literal::boolean(false, lit.get_interval()),
            },
            _ => Literal::boolean(false, lit.get_interval()),
        },
        (
            Literal::FunctionLiteral {
                name: n1,
                value: v1,
                ..
            },
            Literal::FunctionLiteral {
                name: n2,
                value: v2,
                ..
            },
        ) if n1 == "button" && n2 == "button" => {
            match (priority_match(n1, v1), priority_match(n2, v2)) {
                (Some(l1), Some(l2)) => match_obj(l1, l2),
                (_, _) => Literal::boolean(false, v1.get_interval()),
            }
        }
        (
            Literal::FunctionLiteral {
                name: n1,
                value: v1,
                ..
            },
            lit,
        ) if n1 == "button" => match priority_match(n1, v1) {
            Some(l1) => match_obj(l1, lit),
            _ => Literal::boolean(false, v1.get_interval()),
        },
        (
            lit,
            Literal::FunctionLiteral {
                name: n2,
                value: v2,
                ..
            },
        ) if n2 == "button" => match priority_match(n2, v2) {
            Some(l2) => match_obj(lit, l2),
            _ => Literal::boolean(false, lit.get_interval()),
        },

        (
            Literal::ArrayLiteral {
                items: i1,
                interval,
            },
            Literal::ArrayLiteral { items: i2, .. },
        ) => Literal::boolean(i1 == i2, interval.to_owned()),
        (Literal::ArrayLiteral { items, interval }, lit) => {
            Literal::boolean(items.contains(lit), interval.to_owned())
        }
        (lit, Literal::ArrayLiteral { items, interval }) => {
            Literal::boolean(items.contains(lit), interval.to_owned())
        }
        (l1, l2) => Literal::boolean(l1 == l2, l1.get_interval()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::builtins::reserved_functions::button;
    use crate::parser::{ast::Interval, tokens::*};
    use std::collections::HashMap;

    fn gen_inter() -> Interval {
        Interval { line: 0, column: 0 }
    }

    fn gen_button(name: &str) -> Literal {
        let mut map = HashMap::new();
        let interval = gen_inter();

        map.insert(
            DEFAULT.to_owned(),
            Literal::string(name.to_owned(), interval.clone()),
        );

        match button(map, "button".to_owned(), &interval) {
            Ok(lit) => lit,
            Err(..) => panic!("gen button error"),
        }
    }

    fn gen_button_multi_accept(name: &str) -> Literal {
        let mut map = HashMap::new();
        let interval = gen_inter();

        map.insert(
            DEFAULT.to_owned(),
            Literal::string(name.to_owned(), interval.clone()),
        );
        map.insert(
            "accept".to_owned(),
            Literal::array(
                vec![
                    Literal::string("val1".to_owned(), interval.clone()),
                    Literal::string("val2".to_owned(), interval.clone()),
                    Literal::string("val3".to_owned(), interval.clone()),
                ],
                gen_inter(),
            ),
        );

        match button(map, "button".to_owned(), &interval) {
            Ok(lit) => lit,
            Err(..) => panic!("gen button error"),
        }
    }

    fn match_lit_ok(lit1: &Literal, lit2: &Literal) {
        match match_obj(&lit1, &lit2) {
            Literal::BoolLiteral { value: true, .. } => {}
            _ => panic!("\n\nlit1: {:?}\n\n lit2: {:?}\n", lit1, lit2),
        }
    }

    fn match_lit_err(lit1: &Literal, lit2: &Literal) {
        match match_obj(&lit1, &lit2) {
            Literal::BoolLiteral { value: true, .. } => {
                panic!("\n\nlit1: {:?}\n\n lit2: {:?}\n", lit1, lit2)
            }
            _ => {}
        }
    }

    #[test]
    fn ok_match_button_button() {
        let bt1 = gen_button("hola");
        let bt2 = gen_button("hola");

        match_lit_ok(&bt1, &bt2);
    }

    #[test]
    fn ok_match_button_str() {
        let bt1 = gen_button("hola");
        let bt2 = Literal::string("hola".to_owned(), gen_inter());

        match_lit_ok(&bt1, &bt2);
        match_lit_ok(&bt2, &bt1);
    }

    #[test]
    fn ok_match_barray_str() {
        let bt1 = gen_button_multi_accept("hola");
        let bt2 = Literal::string("hola".to_owned(), gen_inter());

        match_lit_ok(&bt1, &bt2);
        match_lit_ok(&bt2, &bt1);
    }

    #[test]
    fn err_match_button_button() {
        let bt1 = gen_button("hola");
        let bt2 = gen_button("nop");

        match_lit_err(&bt1, &bt2);
    }
}
