use std::{borrow::Cow, collections::HashMap};

use nu_protocol::{
    ast::{Expr, Expression},
    Span, Type, Value,
};

#[derive(Clone, Debug)]
pub enum RecordValue {
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Boolean(bool),
    Null,
    Map(Vec<(RecordValue, RecordValue)>),
    List(Vec<RecordValue>),
}

impl RecordValue {
    /// Creates an expresion for this value
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
            RecordValue::Bytes(b) => Expr::Binary(b),
        }
    }

    /// Creates a new wrapped expression for this value
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

    /// Creates a nu protocol value from this value
    pub fn into_protocol_value(self) -> Value {
        let span = Span::new(0, 0);

        match self {
            RecordValue::Int(val) => Value::Int { val, span },
            RecordValue::Float(val) => Value::Float { val, span },
            RecordValue::String(val) => Value::String { val, span },
            RecordValue::Bytes(val) => Value::Binary { val, span },
            RecordValue::Boolean(val) => Value::Bool { val, span },
            RecordValue::Null => Value::Nothing { span },
            RecordValue::Map(m) => {
                let mut cols = Vec::new();
                let mut vals = Vec::new();
                for (key, val) in m {
                    cols.push(key.to_string());
                    vals.push(val.into_protocol_value());
                }
                Value::Record { cols, vals, span }
            }
            RecordValue::List(l) => Value::List {
                vals: l
                    .into_iter()
                    .map(RecordValue::into_protocol_value)
                    .collect(),
                span,
            },
        }
    }

    /// Returns the type of this value
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
            RecordValue::Bytes(_) => Type::Binary,
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
            RecordValue::Bytes(_) => String::new(),
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

impl From<Vec<u8>> for RecordValue {
    fn from(b: Vec<u8>) -> Self {
        Self::Bytes(b)
    }
}
