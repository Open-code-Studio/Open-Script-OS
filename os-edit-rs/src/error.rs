use thiserror::Error;

/// 所有 Open-Script 相关错误类型
#[derive(Error, Debug, Clone)]
pub enum OsError {
    #[error("词法错误 (行 {line}, 列 {col}): {message}")]
    LexerError {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("语法错误 (行 {line}, 列 {col}): {message}")]
    ParserError {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("运行时错误: {message}")]
    RuntimeError { message: String },

    #[error("类型错误: 期望 {expected}, 实际 {actual}")]
    TypeError {
        expected: String,
        actual: String,
    },

    #[error("未定义变量: {name}")]
    UndefinedVariable { name: String },

    #[error("未定义函数: {name}")]
    UndefinedFunction { name: String },

    #[error("参数错误 (函数 {func}): {message}")]
    ArgumentError { func: String, message: String },

    #[error("IO 错误: {0}")]
    IoError(String),

    #[error("除零错误")]
    DivisionByZero,

    #[error("索引越界: 索引 {index}, 长度 {len}")]
    IndexOutOfBounds { index: i64, len: usize },

    #[error("不支持的标签: {tag}")]
    UnsupportedTag { tag: String },

    #[error("break")]
    Break,

    #[error("continue")]
    Continue,

    #[error("return: {0:?}")]
    Return(Option<crate::value::Value>),

    #[error("{0}")]
    Generic(String),
}

impl OsError {
    pub fn lexer(line: usize, col: usize, message: impl Into<String>) -> Self {
        OsError::LexerError {
            line,
            col,
            message: message.into(),
        }
    }

    pub fn parser(line: usize, col: usize, message: impl Into<String>) -> Self {
        OsError::ParserError {
            line,
            col,
            message: message.into(),
        }
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        OsError::RuntimeError {
            message: message.into(),
        }
    }
}

pub type OsResult<T> = Result<T, OsError>;
