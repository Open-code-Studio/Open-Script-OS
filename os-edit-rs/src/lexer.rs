/// Token 类型
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // XML-like 标签结构
    TagOpen,           // <tagname
    TagCloseStart,     // </tagname (开始)
    TagClose,          // > (结束)
    SelfClosing,       // />
    // 属性
    AttrName,          // 属性名
    Equals,            // =
    AttrValue,         // "value" 或 'value'
    // 内容
    Text,              // 标签间文本
    // 特殊
    Comment,           // <comment>...</comment> 或 // ...
    BlockDelimiter,    // 、、、 (文档代码块分隔符，可选)
    ExclamationMark,   // ! 用于 filetype 标签
    // 表达式片段 (在 <op> 标签内部)
    OpContent,         // <op> 内的操作表达式
    // 文件结束
    EOF,
}

/// Token 结构
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
}

/// 词法分析器
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    tokens: Vec<Token>,
    /// 当前是否在 <op> 标签内部
    in_op_tag: bool,
    /// 当前是否在标签属性区域 (已遇到 TagOpen 但未遇到 > 或 />)
    in_attr_area: bool,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            tokens: Vec::new(),
            in_op_tag: false,
            in_attr_area: false,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, crate::error::OsError> {
        while !self.is_eof() {
            self.skip_whitespace_and_newlines();
            if self.is_eof() {
                break;
            }

            let ch = self.peek();

            match ch {
                '<' => {
                    // 可能是标签、注释或其他
                    if self.peek_ahead(1) == Some('/') {
                        // 结束标签 </tagname>
                        self.tokenize_closing_tag()?;
                    } else if self.peek_ahead(1) == Some('!') && self.peek_ahead(2) == Some('-')
                        && self.peek_ahead(3) == Some('-')
                    {
                        // HTML 风格注释 <!-- ... -->
                        self.tokenize_html_comment()?;
                    } else {
                        // 开始标签或自闭合标签 <tagname ...> or <tagname ... />
                        self.tokenize_opening_tag()?;
                    }
                }
                '/' => {
                    // 可能的 // 单行注释
                    if self.peek_ahead(1) == Some('/') {
                        self.tokenize_line_comment()?;
                    } else {
                        self.advance(); // 跳过 /
                        self.tokens.push(Token {
                            token_type: TokenType::Text,
                            lexeme: "/".to_string(),
                            line: self.line,
                            col: self.col - 1,
                        });
                    }
                }
                '、' => {
                    // 可能的 、、、 代码块分隔符
                    if self.peek_ahead(1) == Some('、') && self.peek_ahead(2) == Some('、') {
                        self.advance();
                        self.advance();
                        self.advance();
                        self.tokens.push(Token {
                            token_type: TokenType::BlockDelimiter,
                            lexeme: "、、、".to_string(),
                            line: self.line,
                            col: self.col - 3,
                        });
                    } else {
                        let c = self.advance();
                        self.tokens.push(self.make_text(&c.to_string()));
                    }
                }
                _ => {
                    // 普通文本
                    let text = self.read_until(&['<', '\n']);
                    if !text.is_empty() {
                        self.tokens.push(self.make_text(&text));
                    }
                }
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            line: self.line,
            col: self.col,
        });

        Ok(self.tokens.clone())
    }

    /// Tokenize 开始标签: <tagname ...> 或 <tagname .../>
    fn tokenize_opening_tag(&mut self) -> Result<(), crate::error::OsError> {
        self.advance(); // consume '<'

        // 检查是否是感叹号标签 <!...> (如 filetype)
        let is_bang = self.peek() == '!';
        if is_bang {
            self.advance(); // consume '!'
        }

        // 读取标签名
        let tag_name = self.read_tag_name();

        // 构建完整的 lexeme
        let full_lexeme = if is_bang {
            format!("<!{}", tag_name)
        } else {
            format!("<{}", tag_name)
        };

        self.tokens.push(Token {
            token_type: TokenType::TagOpen,
            lexeme: full_lexeme,
            line: self.line,
            col: self.col - tag_name.len() - if is_bang { 2 } else { 1 },
        });

        self.in_attr_area = true;

        // 跟踪当前是否在 <op> 标签中
        if tag_name == "op" && !is_bang {
            self.in_op_tag = true;
        }

        // 读取属性和闭合
        loop {
            self.skip_whitespace();

            if self.is_eof() {
                return Err(crate::error::OsError::lexer(
                    self.line,
                    self.col,
                    "未闭合的标签",
                ));
            }

            let ch = self.peek();

            if ch == '/' && self.peek_ahead(1) == Some('>') {
                // 自闭合: />
                self.advance(); // '/'
                self.advance(); // '>'
                self.tokens.push(Token {
                    token_type: TokenType::SelfClosing,
                    lexeme: "/>".to_string(),
                    line: self.line,
                    col: self.col - 2,
                });
                self.in_attr_area = false;
                self.in_op_tag = false;
                break;
            } else if ch == '>' {
                // 闭合: >
                self.advance();
                self.tokens.push(Token {
                    token_type: TokenType::TagClose,
                    lexeme: ">".to_string(),
                    line: self.line,
                    col: self.col - 1,
                });
                self.in_attr_area = false;
                break;
            } else {
                // 属性
                self.tokenize_attribute()?;
            }
        }

        Ok(())
    }

    /// Tokenize 结束标签: </tagname>
    fn tokenize_closing_tag(&mut self) -> Result<(), crate::error::OsError> {
        let start_col = self.col;
        self.advance(); // '<'
        self.advance(); // '/'

        let mut lexeme = String::from("</");

        // 读取标签名
        let tag_name = self.read_tag_name();
        if tag_name.is_empty() {
            // 可能是自闭合简写: </>
            if self.peek() == '>' {
                self.advance();
                lexeme.push('>');
                self.tokens.push(Token {
                    token_type: TokenType::SelfClosing,
                    lexeme,
                    line: self.line,
                    col: start_col,
                });
                self.in_op_tag = false;
                return Ok(());
            }
            return Err(crate::error::OsError::lexer(
                self.line,
                self.col,
                "无效的闭合标签",
            ));
        }

        lexeme.push_str(&tag_name);

        // 检查是否是 </op>
        if tag_name == "op" {
            self.in_op_tag = false;
        }

        // 期望 >
        self.skip_whitespace();
        if self.peek() == '>' {
            self.advance();
            lexeme.push('>');
        } else {
            return Err(crate::error::OsError::lexer(
                self.line,
                self.col,
                format!("期望 '>' 在闭合标签 </{}> 后", tag_name),
            ));
        }

        self.tokens.push(Token {
            token_type: TokenType::TagCloseStart,
            lexeme,
            line: self.line,
            col: start_col,
        });

        Ok(())
    }

    /// Tokenize 属性: name="value" 或 name='value'
    fn tokenize_attribute(&mut self) -> Result<(), crate::error::OsError> {
        // 读取属性名
        let name = self.read_attr_name();
        if name.is_empty() {
            return Err(crate::error::OsError::lexer(
                self.line,
                self.col,
                "期望属性名",
            ));
        }

        let name_col = self.col - name.len();
        self.tokens.push(Token {
            token_type: TokenType::AttrName,
            lexeme: name,
            line: self.line,
            col: name_col,
        });

        self.skip_whitespace();

        // 可选的 =
        if self.peek() == '=' {
            self.advance();
            self.tokens.push(Token {
                token_type: TokenType::Equals,
                lexeme: "=".to_string(),
                line: self.line,
                col: self.col - 1,
            });

            self.skip_whitespace();

            // 读取值 (引号内)
            let value = self.read_quoted_string()?;
            self.tokens.push(Token {
                token_type: TokenType::AttrValue,
                lexeme: value,
                line: self.line,
                col: self.col - value.len() - 2, // 减去引号
            });
        }

        Ok(())
    }

    /// 读取引号内的字符串 "..." 或 '...'
    fn read_quoted_string(&mut self) -> Result<String, crate::error::OsError> {
        let quote = self.peek();
        if quote != '"' && quote != '\'' {
            return Err(crate::error::OsError::lexer(
                self.line,
                self.col,
                "期望引号 (\" 或 ') 开始字符串",
            ));
        }

        self.advance(); // 跳过开引号
        let mut result = String::new();

        while !self.is_eof() && self.peek() != quote {
            if self.peek() == '\\' {
                self.advance(); // 跳过转义符
                if !self.is_eof() {
                    let escaped = self.advance();
                    result.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        c if c == quote => quote,
                        c => c,
                    });
                }
            } else {
                result.push(self.advance());
            }
        }

        if self.is_eof() {
            return Err(crate::error::OsError::lexer(
                self.line,
                self.col,
                "未终止的字符串",
            ));
        }

        self.advance(); // 跳过闭引号
        Ok(result)
    }

    /// 读取标签名 (字母数字下划线连字符)
    fn read_tag_name(&mut self) -> String {
        let mut result = String::new();
        while !self.is_eof() {
            let ch = self.peek();
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '.' {
                result.push(self.advance());
            } else {
                break;
            }
        }
        result
    }

    /// 读取属性名
    fn read_attr_name(&mut self) -> String {
        let mut result = String::new();
        while !self.is_eof() {
            let ch = self.peek();
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == ':' {
                result.push(self.advance());
            } else {
                break;
            }
        }
        result
    }

    /// 读取直到遇到指定字符之一
    fn read_until(&mut self, delimiters: &[char]) -> String {
        let mut result = String::new();
        while !self.is_eof() {
            let ch = self.peek();
            if delimiters.contains(&ch) {
                break;
            }
            result.push(self.advance());
        }
        result
    }

    /// Tokenize 单行注释 //
    fn tokenize_line_comment(&mut self) -> Result<(), crate::error::OsError> {
        let start_col = self.col;
        self.advance(); // /
        self.advance(); // /

        let mut comment = String::from("//");
        while !self.is_eof() && self.peek() != '\n' {
            comment.push(self.advance());
        }

        self.tokens.push(Token {
            token_type: TokenType::Comment,
            lexeme: comment,
            line: self.line,
            col: start_col,
        });

        Ok(())
    }

    /// Tokenize HTML 风格注释 <!-- ... -->
    fn tokenize_html_comment(&mut self) -> Result<(), crate::error::OsError> {
        let start_col = self.col;
        let start_line = self.line;
        self.advance(); // <
        self.advance(); // !
        self.advance(); // -
        self.advance(); // -

        let mut comment = String::from("<!--");

        while !self.is_eof() {
            if self.peek() == '-' && self.peek_ahead(1) == Some('-') && self.peek_ahead(2) == Some('>')
            {
                self.advance(); // -
                self.advance(); // -
                self.advance(); // >
                comment.push_str("-->");
                break;
            }
            comment.push(self.advance());
        }

        self.tokens.push(Token {
            token_type: TokenType::Comment,
            lexeme: comment,
            line: start_line,
            col: start_col,
        });

        Ok(())
    }

    // ----- 辅助方法 -----

    fn make_text(&self, text: &str) -> Token {
        Token {
            token_type: TokenType::Text,
            lexeme: text.to_string(),
            line: self.line,
            col: self.col - text.len(),
        }
    }

    fn skip_whitespace_and_newlines(&mut self) {
        while !self.is_eof() {
            let ch = self.peek();
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
                self.advance();
            } else if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            '\0'
        } else {
            self.source[self.pos]
        }
    }

    fn peek_ahead(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.source.len() {
            Some(self.source[self.pos + offset])
        } else {
            None
        }
    }

    fn advance(&mut self) -> char {
        if self.is_eof() {
            return '\0';
        }
        let ch = self.source[self.pos];
        self.pos += 1;
        self.col += 1;
        ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tag() {
        let mut lexer = Lexer::new("<out>Hello</out>");
        let tokens = lexer.tokenize().unwrap();
        let types: Vec<TokenType> = tokens.iter().map(|t| t.token_type.clone()).collect();
        assert_eq!(
            types,
            vec![
                TokenType::TagOpen,
                TokenType::TagClose,
                TokenType::Text,
                TokenType::TagCloseStart,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn test_tag_with_attributes() {
        let mut lexer = Lexer::new("<func name=\"add\" params=\"a:int,b:int\">");
        let tokens = lexer.tokenize().unwrap();
        let attr_names: Vec<&str> = tokens
            .iter()
            .filter(|t| t.token_type == TokenType::AttrName)
            .map(|t| t.lexeme.as_str())
            .collect();
        assert_eq!(attr_names, vec!["name", "params"]);
    }

    #[test]
    fn test_line_comment() {
        let mut lexer = Lexer::new("// this is a comment\n<out>hi</out>");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Comment);
        assert_eq!(tokens[1].token_type, TokenType::TagOpen);
    }
}
