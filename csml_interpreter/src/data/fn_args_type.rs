use crate::data::Literal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ArgsType {
    Named(HashMap<String, Literal>),
    Normal(HashMap<String, Literal>)
}

impl ArgsType {
    pub fn get<'a>(&'a self, key: &str, index: usize) -> Option<&'a Literal> {
        match self {
            Self::Named(var) => {
                match (var.get(key), index) {
                    (Some(val), _) => Some(val),
                    // tmp ?
                    (None, 0) => var.get(&format!("arg{}", index)),
                    (None, _) => None
                }
            }
            Self::Normal(var) => {
                var.get(&format!("arg{}", index))
            }
        }
    }

    pub fn populate(&self, map: &mut HashMap<String, Literal>, vec: &[&str]) {
        match self {
            Self::Named(var) => {
                for (key , value) in var.iter() {
                    if !vec.contains(&(key as &str)) && key != "arg0" {
                        map.insert(key.to_owned(), value.to_owned());
                    }
                }
            }
            Self::Normal(var) => {
                if vec.len() < var.len() {
                    panic!("_-_")
                }
            }
        }
    }

    pub fn populate_json_to_literal(&self, map: &mut HashMap<String, Literal>, value_key: &str, len: usize) {
        match self {
            Self::Named(var) => {
                for (key , value) in var.iter() {

                    if value_key != key && "arg0" != key {
                        map.insert(key.to_owned(), value.to_owned());
                    }
                }
            }
            Self::Normal(var) => {
                if len < var.len() {
                    panic!("_-_")
                }
            }
        }
    }
}