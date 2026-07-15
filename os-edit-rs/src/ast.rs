use std::collections::HashMap;

/// Open-Script AST 节点
#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    /// 节点类型
    pub kind: AstKind,
    /// 源码行号
    pub line: usize,
    /// 源码列号
    pub col: usize,
}

/// AST 节点类型
#[derive(Debug, Clone, PartialEq)]
pub enum AstKind {
    /// 根节点
    Root(Vec<AstNode>),

    // ---- 框架 ----
    /// <filetype !standard-script!> 脚本文件
    ScriptFile {
        gui_section: Option<Vec<AstNode>>,
        script_section: Vec<AstNode>,
    },
    /// <filetype !style-file!> 样式文件
    StyleFile(Vec<AstNode>),

    // ---- 内置脚本标签 ----
    /// <out>content</out> 输出
    Output(Vec<AstNode>),
    /// <text>content</text> 文本
    Text(String),
    /// <comment>content</comment> 注释 (编译器忽略)
    Comment(String),
    /// <var name="x">value</var> 变量声明
    VarDecl {
        name: String,
        var_type: Option<String>,
        value: Option<Box<AstNode>>,
    },
    /// <var>name</var> 变量引用
    VarRef(String),
    /// <func name="f" params="a:int,b:int" return="int">body</func> 函数定义
    FuncDef {
        name: String,
        params: Vec<(String, String)>, // (name, type)
        return_type: Option<String>,
        body: Vec<AstNode>,
    },
    /// <call name="f" args="1,2"/> 函数调用
    FuncCall {
        name: String,
        args: Vec<Expr>,
    },
    /// <return>value</return> 返回
    Return(Option<Box<AstNode>>),

    /// <if condition="x > 0">...</if> 条件
    If {
        condition: Expr,
        then_branch: Vec<AstNode>,
        else_ifs: Vec<(Expr, Vec<AstNode>)>,
        else_branch: Option<Vec<AstNode>>,
    },
    /// <loop cnt="10">...</loop> 计数循环
    Loop {
        count: Expr,
        body: Vec<AstNode>,
    },
    /// <while condition="x > 0">...</while> while 循环
    While {
        condition: Expr,
        body: Vec<AstNode>,
    },
    /// <for-each item="x" collection="list">...</for-each> for-each 循环
    ForEach {
        item: String,
        collection: Expr,
        body: Vec<AstNode>,
    },
    /// <break/> 跳出循环
    Break,
    /// <continue/> 继续循环
    Continue,

    /// <op>x + y</op> 操作/表达式
    Operation(Expr),

    /// <type var="x" type="int"/> 类型声明
    TypeDecl {
        var: String,
        var_type: String,
    },

    /// <match value="x">...</match> 模式匹配
    Match {
        value: Expr,
        cases: Vec<(String, Vec<AstNode>)>, // (pattern, body)
        default: Option<Vec<AstNode>>,
    },

    /// <try>...</try> 错误处理
    TryCatch {
        body: Vec<AstNode>,
        catches: Vec<(String, Vec<AstNode>)>, // (error_type, body)
        finally: Option<Vec<AstNode>>,
    },
    /// <throw type="err" message="msg"/> 抛出错误
    Throw {
        error_type: String,
        message: Option<String>,
    },

    // ---- 数据结构 ----
    /// <struct name="X">...</struct> 结构体定义
    StructDef {
        name: String,
        fields: Vec<StructField>,
    },
    /// <new struct="X" name="inst"/> 结构体实例化
    StructNew {
        struct_name: String,
        name: String,
        field_values: HashMap<String, Expr>,
    },
    /// <set-field struct="s" field="f" value="v"/> 设置字段
    SetField {
        struct_name: String,
        field: String,
        value: Expr,
    },
    /// <get-field struct="s" field="f"/> 获取字段
    GetField {
        struct_name: String,
        field: String,
    },

    /// <enum name="E">...</enum> 枚举定义
    EnumDef {
        name: String,
        variants: Vec<EnumVariant>,
    },

    /// <list name="tokens" values="1,2,3"/> 列表
    ListLiteral {
        values: Vec<Expr>,
    },
    /// <map name="m">...</map> 映射
    MapLiteral {
        entries: HashMap<String, Expr>,
    },

    /// <push list="l" value="v"/> 推入列表
    ListPush {
        list: String,
        value: Expr,
    },
    /// <pop list="l" into="x"/> 弹出列表
    ListPop {
        list: String,
        into: String,
    },
    /// <get list="l" index="i" into="x"/> 获取列表元素
    ListGet {
        list: String,
        index: Expr,
        into: String,
    },
    /// <set list="l" index="i" value="v"/> 设置列表元素
    ListSet {
        list: String,
        index: Expr,
        value: Expr,
    },
    /// <length list="l" into="x"/> 获取长度
    Length {
        target: String,
        into: String,
    },

    /// <set map="m" key="k" value="v"/> 设置映射
    MapSet {
        map: String,
        key: String,
        value: Expr,
    },
    /// <get map="m" key="k" into="x"/> 获取映射
    MapGet {
        map: String,
        key: String,
        into: String,
    },

    // ---- 字符串操作 ----
    StrConcat {
        a: String,
        b: String,
        into: String,
    },
    StrLength {
        str: String,
        into: String,
    },
    StrSubstr {
        str: String,
        start: Expr,
        end: Expr,
        into: String,
    },

    // ---- 文件操作 ----
    FileRead {
        path: Expr,
        into: String,
    },
    FileWrite {
        path: Expr,
        content: Expr,
    },

    /// <exec command="cmd" args="args"/> 进程执行
    Exec {
        command: Expr,
        args: Expr,
    },

    /// close / exit
    Closure {
        params: Vec<(String, String)>,
        return_type: Option<String>,
        body: Vec<AstNode>,
    },
    CallClosure {
        params: String,
        body: Vec<AstNode>,
    },

    /// 模块声明
    ModuleDecl { name: String },
    /// 包声明
    PackageDecl { name: String },
    /// 宏声明
    MacroDecl { name: String },

    // 节点列表 (容器)
    Block(Vec<AstNode>),
}

/// 结构体字段定义
#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub field_type: String,
    pub default: Option<String>,
}

/// 枚举变体定义
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub value_type: Option<String>,
}

/// 表达式 (用于属性和 <op> 标签)
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// 字面量整数
    IntLiteral(i64),
    /// 字面量浮点数
    FloatLiteral(f64),
    /// 字面量字符串
    StringLiteral(String),
    /// 字面量布尔
    BoolLiteral(bool),
    /// 变量引用
    Variable(String),
    /// 二元运算 (左, 运算符, 右)
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    /// 一元运算
    Unary(UnaryOp, Box<Expr>),
    /// 函数调用
    Call {
        name: String,
        args: Vec<Expr>,
    },
    /// 字段访问 a.b
    FieldAccess {
        object: String,
        field: String,
    },
    /// 裸文本 (用于 <text> 等)
    Raw(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Assign,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// 从属性值解析表达式
pub fn parse_expr(expr_str: &str) -> Expr {
    let trimmed = expr_str.trim();

    // 布尔
    if trimmed == "true" {
        return Expr::BoolLiteral(true);
    }
    if trimmed == "false" {
        return Expr::BoolLiteral(false);
    }

    // 字符串
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        return Expr::StringLiteral(trimmed[1..trimmed.len() - 1].to_string());
    }

    // 浮点数
    if let Ok(f) = trimmed.parse::<f64>() {
        if trimmed.contains('.') {
            return Expr::FloatLiteral(f);
        }
    }

    // 整数
    if let Ok(n) = trimmed.parse::<i64>() {
        return Expr::IntLiteral(n);
    }

    // 默认作为变量引用或原始文本
    if trimmed.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        Expr::Variable(trimmed.to_string())
    } else {
        // 复杂表达式需要进一步解析
        parse_complex_expr(trimmed)
    }
}

/// 解析复杂表达式 (含运算符)
fn parse_complex_expr(input: &str) -> Expr {
    // 尝试解析二元运算
    let ops = [
        ("==", BinaryOp::Eq),
        ("!=", BinaryOp::Neq),
        ("<=", BinaryOp::Le),
        (">=", BinaryOp::Ge),
        ("<", BinaryOp::Lt),
        (">", BinaryOp::Gt),
        ("+", BinaryOp::Add),
        ("-", BinaryOp::Sub),
        ("*", BinaryOp::Mul),
        ("/", BinaryOp::Div),
        ("%", BinaryOp::Mod),
        ("=", BinaryOp::Assign),
    ];

    // 从后往前找运算符 (处理优先级)
    let mut best_pos = None;
    let mut best_op = BinaryOp::Add;

    for (op_str, op) in &ops {
        if let Some(pos) = input.rfind(op_str) {
            if pos > 0 && pos < input.len() - op_str.len() {
                if best_pos.is_none() || pos > best_pos.unwrap() {
                    best_pos = Some(pos);
                    best_op = *op;
                }
            }
        }
    }

    if let Some(pos) = best_pos {
        let left = parse_expr(&input[..pos]);
        let right = parse_expr(&input[pos + match best_op {
            BinaryOp::Eq | BinaryOp::Neq | BinaryOp::Le | BinaryOp::Ge => 2,
            _ => 1,
        }..]);
        return Expr::Binary(Box::new(left), best_op, Box::new(right));
    }

    // 无法解析，作为裸字符串
    Expr::Raw(input.to_string())
}
