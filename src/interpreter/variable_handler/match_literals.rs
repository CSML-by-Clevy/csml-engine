use crate::parser::ast::*;

fn priority_match<'a>(name: &str, lit: &'a Literal) -> Option<&'a Literal>{
    match name {
        "button" => {
            if let Literal::ObjectLiteral{properties, ..} = lit {
                properties.get("accept")
            } else {
                Some(lit)
            }
        },
        _   => Some(lit)
    }
}

//TODO: add warning when comparing some objects
pub fn match_obj(lit1: &Literal, lit2: &Literal) -> Literal {
    match (&lit1, &lit2) {
        (Literal::ObjectLiteral{properties: p1, interval}, Literal::ObjectLiteral{properties: p2, ..}) => {
            match (p1.get("button"), p2.get("button")) {
                (Some(l1), Some(l2)) => match (priority_match("button", l1), priority_match("button", l2)) {
                    (Some(l1), Some(l2)) => match_obj(l1, l2),
                    (_, _) => Literal::boolean(false, interval.to_owned())
                },
                (_, _) => Literal::boolean(false, interval.to_owned())
            }
        },
        (Literal::ObjectLiteral{properties, interval}, lit) => {
            match properties.get("button") {
                Some(l1) => match priority_match("button", l1) {
                    Some(l1) => match_obj(l1, lit),
                    _ => Literal::boolean(false, interval.to_owned())
                },
                _ => Literal::boolean(false, interval.to_owned())
            }
        },
        (lit, Literal::ObjectLiteral{properties, ..}) => {
            match properties.get("button") {
                Some(l2) => match priority_match("button", l2) {
                    Some(l2) => match_obj(l2, lit),
                    _ => Literal::boolean(false, lit.get_interval())
                },
                _ => Literal::boolean(false, lit.get_interval())
            }
        },
        (Literal::ArrayLiteral{items: i1, interval}, Literal::ArrayLiteral{items: i2, ..}) => Literal::boolean(i1 == i2, interval.to_owned()),
        (Literal::ArrayLiteral{items, interval}, lit) => Literal::boolean(items.contains(lit), interval.to_owned()),
        (lit, Literal::ArrayLiteral{items, interval}) => Literal::boolean(items.contains(lit), interval.to_owned()),
        (l1, l2) => Literal::boolean(l1 == l2, l1.get_interval())
    }
}
