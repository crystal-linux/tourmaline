use std::{borrow::Cow, collections::HashMap};

use nu_protocol::{
    ast::{Expr, Expression},
    Span, Type,
};

#[derive(Clone, Debug)]
pub enum RecordValue {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Map(Vec<(RecordValue, RecordValue)>),
    List(Vec<RecordValue>),
}

impl RecordValue {
    pub fn into_expr(self) -> Expr {
        match self {
            RecordValue::Int(i) => Expr::Int(i),
            RecordValue::Float(f) => Expr::Float(f),
            RecordValue::String(s) => Expr::String(s),
            RecordValue::Boolean(b) => Expr::Bool(b),
            RecordValue::Null => Expr::Nothing,
            RecordValue::Map(m) => Expr::Record(
                m.into_iter()
                    .map(|(k, v)| (k.into_expression(), v.into_expression()))
                    .collect(),
            ),
            RecordValue::List(l) => {
                Expr::List(l.into_iter().map(RecordValue::into_expression).collect())
            }
        }
    }

    pub fn into_expression(self) -> Expression {
        let nu_type = self.get_type();
        let expr = self.into_expr();

        Expression {
            expr,
            span: Span::new(0, 0),
            ty: nu_type,
            custom_completion: None,
        }
    }

    fn get_type(&self) -> Type {
        match &self {
            RecordValue::Int(_) => Type::Int,
            RecordValue::Float(_) => Type::Float,
            RecordValue::String(_) => Type::String,
            RecordValue::Boolean(_) => Type::Bool,
            RecordValue::Null => Type::Nothing,
            RecordValue::Map(m) => {
                let type_map = m
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.get_type()))
                    .collect();
                Type::Record(type_map)
            }
            RecordValue::List(l) => {
                let list_type = if let Some(first) = l.first() {
                    first.get_type()
                } else {
                    Type::Nothing
                };
                Type::List(Box::new(list_type))
            }
        }
    }
}

impl ToString for RecordValue {
    fn to_string(&self) -> String {
        match self {
            RecordValue::Int(i) => i.to_string(),
            RecordValue::Float(f) => f.to_string(),
            RecordValue::String(s) => s.clone(),
            RecordValue::Boolean(b) => b.to_string(),
            RecordValue::Null => String::new(),
            RecordValue::Map(_) => String::new(),
            RecordValue::List(_) => String::new(),
        }
    }
}

impl From<i64> for RecordValue {
    fn from(num: i64) -> Self {
        Self::Int(num)
    }
}

impl From<f64> for RecordValue {
    fn from(num: f64) -> Self {
        Self::Float(num)
    }
}

impl From<String> for RecordValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl<'a> From<Cow<'a, str>> for RecordValue {
    fn from(s: Cow<'a, str>) -> Self {
        Self::String(s.into_owned())
    }
}

impl From<&str> for RecordValue {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

impl From<bool> for RecordValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl<T: Into<RecordValue>> From<Option<T>> for RecordValue {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => val.into(),
            None => Self::Null,
        }
    }
}

impl<T1: Into<RecordValue>, T2: Into<RecordValue>> From<HashMap<T1, T2>> for RecordValue {
    fn from(map: HashMap<T1, T2>) -> Self {
        let map = map.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Self::Map(map)
    }
}

impl<T: Into<RecordValue>> From<Vec<T>> for RecordValue {
    fn from(list: Vec<T>) -> Self {
        let list = list.into_iter().map(|l| l.into()).collect();
        Self::List(list)
    }
}
