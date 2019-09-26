use crate::error_format::data::ErrorInfo;
use crate::interpreter::message::Message;
use crate::parser::{ast::Interval, tokens::NULL};
use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Add, BitAnd, BitOr, Div, Mul, Rem, Sub},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral {
        value: String,
        interval: Interval,
    },
    IntLiteral {
        value: i64,
        interval: Interval,
    },
    FloatLiteral {
        value: f64,
        interval: Interval,
    },
    BoolLiteral {
        value: bool,
        interval: Interval,
    },
    ArrayLiteral {
        items: Vec<Literal>,
        interval: Interval,
    },
    ObjectLiteral {
        properties: HashMap<String, Literal>,
        interval: Interval,
    },
    FunctionLiteral {
        name: String,
        value: Box<Literal>,
        interval: Interval,
    },
    Null {
        value: String,
        interval: Interval,
    },
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (
                Literal::StringLiteral { value: l1, .. },
                Literal::StringLiteral { value: l2, .. },
            ) => l1.partial_cmp(l2),

            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::StringLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => l1.partial_cmp(&value),
                Literal::FloatLiteral { value, .. } => l1.partial_cmp(&(value as i64)),
                _ => None,
            },
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => value.partial_cmp(l2),
                Literal::FloatLiteral { value, .. } => (value as i64).partial_cmp(l2),
                _ => None,
            },

            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::StringLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => l1.partial_cmp(&(value as f64)),
                Literal::FloatLiteral { value, .. } => l1.partial_cmp(&value),
                _ => None,
            },
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => (value as f64).partial_cmp(l2),
                Literal::FloatLiteral { value, .. } => value.partial_cmp(l2),
                _ => None,
            },

            (Literal::IntLiteral { value: l1, .. }, Literal::IntLiteral { value: l2, .. }) => {
                l1.partial_cmp(l2)
            }

            (Literal::FloatLiteral { value: l1, .. }, Literal::IntLiteral { value: l2, .. }) => {
                l1.partial_cmp(&(*l2 as f64))
            }

            (Literal::IntLiteral { value: l1, .. }, Literal::FloatLiteral { value: l2, .. }) => {
                (*l1 as f64).partial_cmp(l2)
            }

            (Literal::FloatLiteral { value: l1, .. }, Literal::FloatLiteral { value: l2, .. }) => {
                l1.partial_cmp(l2)
            }

            (Literal::BoolLiteral { value: l1, .. }, Literal::BoolLiteral { value: l2, .. }) => {
                l1.partial_cmp(l2)
            }
            _ => None,
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Literal::StringLiteral { value: l1, .. },
                Literal::StringLiteral { value: l2, .. },
            ) => l1 == l2,
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::StringLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => *l1 == value,
                Literal::FloatLiteral { value, .. } => *l1 == value as i64,
                _ => false,
            },
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => *l2 == value,
                Literal::FloatLiteral { value, .. } => *l2 == value as i64,
                _ => false,
            },

            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::StringLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l2, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => *l1 == value as f64,
                Literal::FloatLiteral { value, .. } => *l1 == value,
                _ => false,
            },
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => match Self::str_to_literal(l1, interval.to_owned()) {
                Literal::IntLiteral { value, .. } => *l2 == value as f64,
                Literal::FloatLiteral { value, .. } => *l2 == value,
                _ => false,
            },
            (Literal::IntLiteral { value: l1, .. }, Literal::IntLiteral { value: l2, .. }) => {
                l1 == l2
            }
            (Literal::FloatLiteral { value: l1, .. }, Literal::IntLiteral { value: l2, .. }) => {
                *l1 == *l2 as f64
            }
            (Literal::IntLiteral { value: l1, .. }, Literal::FloatLiteral { value: l2, .. }) => {
                *l1 as f64 == *l2
            }
            (Literal::FloatLiteral { value: l1, .. }, Literal::FloatLiteral { value: l2, .. }) => {
                l1 == l2
            }
            (Literal::BoolLiteral { value: l1, .. }, Literal::BoolLiteral { value: l2, .. }) => {
                l1 == l2
            }
            (Literal::ArrayLiteral { items: l1, .. }, Literal::ArrayLiteral { items: l2, .. }) => {
                l1 == l2
            }
            (
                Literal::FunctionLiteral { name: l1, .. },
                Literal::FunctionLiteral { name: l2, .. },
            ) => l1 == l2,
            _ => false,
        }
    }
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::StringLiteral { value, .. } => value.to_owned(),
            Literal::IntLiteral { value, .. } => value.to_string(),
            Literal::FloatLiteral { value, .. } => value.to_string(),
            Literal::BoolLiteral { value, .. } => value.to_string(),
            Literal::ArrayLiteral { .. }
            | Literal::ObjectLiteral { .. }
            | Literal::FunctionLiteral { .. } => Message::lit_to_json(self.to_owned()).to_string(),
            Literal::Null { value, .. } => value.to_owned(),
        }
    }

    pub fn is_valid(&self) -> Self {
        match self {
            Literal::BoolLiteral {
                value: false,
                interval,
            } => Self::boolean(false, interval.to_owned()),
            Literal::BoolLiteral {
                value: true,
                interval,
            } => Self::boolean(true, interval.to_owned()),
            Literal::StringLiteral { .. }
            | Literal::IntLiteral { .. }
            | Literal::FloatLiteral { .. }
            | Literal::ArrayLiteral { .. }
            | Literal::ObjectLiteral { .. }
            | Literal::FunctionLiteral { .. } => self.to_owned(),
            Literal::Null { interval, .. } => Self::boolean(false, interval.to_owned()),
        }
    }

    pub fn type_to_string(&self) -> String {
        match self {
            Literal::StringLiteral { .. } => "string".to_owned(),
            Literal::IntLiteral { .. } => "int".to_owned(),
            Literal::FloatLiteral { .. } => "float".to_owned(),
            Literal::BoolLiteral { .. } => "bool".to_owned(),
            Literal::ArrayLiteral { .. } => "array".to_owned(),
            Literal::ObjectLiteral { .. } => "object".to_owned(),
            Literal::FunctionLiteral { name, .. } => name.to_owned(),
            Literal::Null { value, .. } => value.to_owned(),
        }
    }

    pub fn str_to_literal(stirng: &str, interval: Interval) -> Self {
        match (i64::from_str(stirng), f64::from_str(stirng)) {
            (Ok(int), _) => Self::int(int, interval),
            (_, Ok(float)) => Self::float(float, interval),
            (_, _) => Self::string(stirng.to_owned(), interval),
        }
    }

    pub fn get_interval(&self) -> Interval {
        match self {
            Literal::StringLiteral { interval, .. } => interval.to_owned(),
            Literal::IntLiteral { interval, .. } => interval.to_owned(),
            Literal::FloatLiteral { interval, .. } => interval.to_owned(),
            Literal::BoolLiteral { interval, .. } => interval.to_owned(),
            Literal::ArrayLiteral { interval, .. } => interval.to_owned(),
            Literal::ObjectLiteral { interval, .. } => interval.to_owned(),
            Literal::FunctionLiteral { interval, .. } => interval.to_owned(),
            Literal::Null { interval, .. } => interval.to_owned(),
        }
    }

    pub fn is_string(&self) -> bool {
        if let Literal::StringLiteral { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn float(value: f64, interval: Interval) -> Self {
        Self::FloatLiteral { value, interval }
    }

    pub fn int(value: i64, interval: Interval) -> Self {
        Self::IntLiteral { value, interval }
    }

    pub fn boolean(value: bool, interval: Interval) -> Self {
        Self::BoolLiteral { value, interval }
    }

    pub fn string(value: String, interval: Interval) -> Self {
        Self::StringLiteral { value, interval }
    }

    pub fn array(items: Vec<Self>, interval: Interval) -> Self {
        Self::ArrayLiteral { items, interval }
    }

    pub fn object(properties: HashMap<String, Self>, interval: Interval) -> Self {
        Self::ObjectLiteral {
            properties,
            interval,
        }
    }

    pub fn name_object(name: String, value: &Self, interval: Interval) -> Self {
        Self::FunctionLiteral {
            name,
            value: Box::new(value.to_owned()),
            interval,
        }
    }

    pub fn lit_to_obj(properties: Self, name: String, interval: Interval) -> Self {
        let mut obj: HashMap<String, Self> = HashMap::new();

        obj.insert(name, properties);
        Self::object(obj, interval)
    }

    pub fn null(interval: Interval) -> Self {
        Literal::Null {
            value: NULL.to_owned(),
            interval,
        }
    }
}

fn convert_to_numeric(var: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    let lit = Literal::str_to_literal(var, interval);
    match lit {
        Literal::StringLiteral { .. } => Err(ErrorInfo {
            message: "Illegal operation between types".to_owned(),
            interval: lit.get_interval(),
        }),
        lit => Ok(lit),
    }
}

impl Add for Literal {
    type Output = Result<Self, ErrorInfo>;

    fn add(self, other: Self) -> Result<Self, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral {
                    value: l1,
                    interval: interval1,
                },
                Literal::StringLiteral {
                    value: l2,
                    interval: interval2,
                },
            ) => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok((lit1 + lit2)?)
            }
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                l2,
            ) => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok((lit + l2)?)
            }
            (
                l1,
                Literal::StringLiteral {
                    value: l2,
                    interval,
                },
            ) => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok((l1 + lit)?)
            }
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 + l2 as f64, interval.to_owned())),
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 as f64 + l2, interval.to_owned())),
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 + l2, interval.to_owned())),
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::int(l1 + l2, interval.to_owned())),
            (
                Literal::BoolLiteral {
                    value: l1,
                    interval,
                },
                Literal::BoolLiteral { value: l2, .. },
            ) => Ok(Self::int(l1 as i64 + l2 as i64, interval.to_owned())),
            (l1, _) => Err(ErrorInfo {
                message: "Illegal operation + between types".to_owned(),
                interval: l1.get_interval(),
            }),
        }
    }
}

impl Sub for Literal {
    type Output = Result<Self, ErrorInfo>;

    fn sub(self, other: Self) -> Result<Self, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral {
                    value: l1,
                    interval: interval1,
                },
                Literal::StringLiteral {
                    value: l2,
                    interval: interval2,
                },
            ) => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok((lit1 - lit2)?)
            }
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                l2,
            ) => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok((lit - l2)?)
            }
            (
                l1,
                Literal::StringLiteral {
                    value: l2,
                    interval,
                },
            ) => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok((l1 - lit)?)
            }
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 - l2 as f64, interval.to_owned())),
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 as f64 - l2, interval.to_owned())),
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 - l2, interval.to_owned())),

            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::int(l1 - l2, interval.to_owned())),
            (
                Literal::BoolLiteral {
                    value: l1,
                    interval,
                },
                Literal::BoolLiteral { value: l2, .. },
            ) => Ok(Self::int(l1 as i64 - l2 as i64, interval.to_owned())),
            (l1, _) => Err(ErrorInfo {
                message: "Illegal operation - between types".to_owned(),
                interval: l1.get_interval(),
            }),
        }
    }
}

impl Div for Literal {
    type Output = Result<Self, ErrorInfo>;

    fn div(self, other: Self) -> Result<Self, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral {
                    value: l1,
                    interval: interval1,
                },
                Literal::StringLiteral {
                    value: l2,
                    interval: interval2,
                },
            ) => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok((lit1 / lit2)?)
            }
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                l2,
            ) => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok((lit / l2)?)
            }
            (
                l1,
                Literal::StringLiteral {
                    value: l2,
                    interval,
                },
            ) => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok((l1 / lit)?)
            }
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => {
                if l2 == 0 {
                    return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    });
                }
                Ok(Self::float(l1 / l2 as f64, interval.to_owned()))
            }
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => {
                if l2 == 0.0 {
                    return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    });
                }
                Ok(Self::float(l1 as f64 / l2, interval.to_owned()))
            }
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => {
                if l2 == 0.0 {
                    return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    });
                }
                Ok(Self::float(l1 / l2, interval.to_owned()))
            }
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => {
                if l2 == 0 {
                    return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    });
                }
                Ok(Self::float(l1 as f64 / l2 as f64, interval.to_owned()))
            }
            (
                Literal::BoolLiteral {
                    value: l1,
                    interval,
                },
                Literal::BoolLiteral { value: l2, .. },
            ) => {
                if !l2 {
                    return Err(ErrorInfo {
                        message: "Cannot divide by zero-valued".to_owned(),
                        interval: interval.to_owned(),
                    });
                }
                Ok(Self::int(l1 as i64 / l2 as i64, interval.to_owned()))
            }
            (l1, _) => Err(ErrorInfo {
                message: "Illegal operation / between types".to_owned(),
                interval: l1.get_interval(),
            }),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Self, ErrorInfo>;

    fn mul(self, other: Self) -> Result<Self, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral {
                    value: l1,
                    interval: interval1,
                },
                Literal::StringLiteral {
                    value: l2,
                    interval: interval2,
                },
            ) => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok((lit1 * lit2)?)
            }
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                l2,
            ) => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok((lit * l2)?)
            }
            (
                l1,
                Literal::StringLiteral {
                    value: l2,
                    interval,
                },
            ) => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok((l1 * lit)?)
            }
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 * l2 as f64, interval.to_owned())),
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 as f64 * l2, interval.to_owned())),
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 * l2, interval.to_owned())),
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::int(l1 * l2, interval.to_owned())),
            (
                Literal::BoolLiteral {
                    value: l1,
                    interval,
                },
                Literal::BoolLiteral { value: l2, .. },
            ) => Ok(Self::int(l1 as i64 * l2 as i64, interval.to_owned())),
            (l1, _) => Err(ErrorInfo {
                message: "Illegal operation * between types".to_owned(),
                interval: l1.get_interval(),
            }),
        }
    }
}

impl Rem for Literal {
    type Output = Result<Self, ErrorInfo>;

    fn rem(self, other: Self) -> Result<Self, ErrorInfo> {
        match (self, other) {
            (
                Literal::StringLiteral {
                    value: l1,
                    interval: interval1,
                },
                Literal::StringLiteral {
                    value: l2,
                    interval: interval2,
                },
            ) => {
                let lit1 = convert_to_numeric(&l1, interval1)?;
                let lit2 = convert_to_numeric(&l2, interval2)?;
                Ok((lit1 % lit2)?)
            }
            (
                Literal::StringLiteral {
                    value: l1,
                    interval,
                },
                l2,
            ) => {
                let lit = convert_to_numeric(&l1, interval)?;
                Ok((lit % l2)?)
            }
            (
                l1,
                Literal::StringLiteral {
                    value: l2,
                    interval,
                },
            ) => {
                let lit = convert_to_numeric(&l2, interval)?;
                Ok((l1 % lit)?)
            }
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 % l2 as f64, interval.to_owned())),
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 as f64 % l2, interval.to_owned())),
            (
                Literal::FloatLiteral {
                    value: l1,
                    interval,
                },
                Literal::FloatLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 % l2, interval.to_owned())),
            (
                Literal::IntLiteral {
                    value: l1,
                    interval,
                },
                Literal::IntLiteral { value: l2, .. },
            ) => Ok(Self::float(l1 as f64 % l2 as f64, interval.to_owned())),
            (
                Literal::BoolLiteral {
                    value: l1,
                    interval,
                },
                Literal::BoolLiteral { value: l2, .. },
            ) => Ok(Self::int(l1 as i64 % l2 as i64, interval.to_owned())),
            (l1, _) => Err(ErrorInfo {
                message: "Illegal operation % between types".to_owned(),
                interval: l1.get_interval(),
            }),
        }
    }
}

impl BitAnd for Literal {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (
                Literal::BoolLiteral {
                    value: false,
                    interval,
                },
                _,
            )
            | (
                _,
                Literal::BoolLiteral {
                    value: false,
                    interval,
                },
            ) => Self::boolean(false, interval.to_owned()),
            (Literal::Null { interval, .. }, _) | (_, Literal::Null { interval, .. }) => {
                Self::boolean(false, interval.to_owned())
            }
            (l1, ..) => Self::boolean(true, l1.get_interval()),
        }
    }
}

impl BitOr for Literal {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (
                Literal::BoolLiteral {
                    value: false,
                    interval,
                },
                Literal::BoolLiteral { value: false, .. },
            )
            | (
                Literal::BoolLiteral {
                    value: false,
                    interval,
                },
                Literal::Null { .. },
            ) => Self::boolean(false, interval.to_owned()),
            (Literal::Null { interval, .. }, Literal::Null { .. })
            | (Literal::Null { interval, .. }, Literal::BoolLiteral { value: false, .. }) => {
                Self::boolean(false, interval.to_owned())
            }
            (l1, ..) => Self::boolean(true, l1.get_interval()),
        }
    }
}
