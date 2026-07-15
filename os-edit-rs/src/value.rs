use std::{collections::HashMap, fmt, rc::Rc};

/// Open-Script 运行时值类型
#[derive(Debug, Clone)]
pub enum Value {
    /// 整数
    Int(i64),
    /// 浮点数
    Float(f64),
    /// 字符串
    Str(String),
    /// 布尔值
    Bool(bool),
    /// 列表
    List(Vec<Value>),
    /// 映射 (字典)
    Map(HashMap<String, Value>),
    /// 函数引用 (函数名)
    Function(String),
    /// 闭包 (参数列表, 函数体AST节点ID, 捕获的环境)
    Closure {
        params: Vec<(String, String)>, // (name, type)
        body: Vec<crate::ast::AstNode>,
        capture: HashMap<String, Value>,
    },
    /// 结构体实例
    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },
    /// 枚举变体
    EnumVariant {
        enum_name: String,
        variant: String,
        inner: Option<Box<Value>>,
    },
    /// 空值
    Null,
}

impl Value {
    /// 类型名称
    pub fn type_name(&self) -> &str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Str(_) => "str",
            Value::Bool(_) => "bool",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Function(_) => "function",
            Value::Closure { .. } => "closure",
            Value::Struct { .. } => "struct",
            Value::EnumVariant { .. } => "enum",
            Value::Null => "null",
        }
    }

    /// 转为整数
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            Value::Float(f) => Some(*f as i64),
            Value::Str(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// 转为浮点数
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(n) => Some(*n as f64),
            Value::Str(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// 转为字符串
    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// 转为布尔
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// 是否为真值 (truthy)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Null => false,
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(v) => write!(f, "{}", v),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Function(name) => write!(f, "<function {}>", name),
            Value::Closure { .. } => write!(f, "<closure>"),
            Value::Struct { name, .. } => write!(f, "<struct {}>", name),
            Value::EnumVariant {
                enum_name, variant, ..
            } => write!(f, "<enum {}::{}>", enum_name, variant),
            Value::Null => write!(f, "null"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Int(a), Value::Float(b)) => *a as f64 == *b,
            (Value::Float(a), Value::Int(b)) => *a == *b as f64,
            _ => false,
        }
    }
}
