# Open-Script/开放脚本
OpenOS 开放操作系统 -- 开放贡献计划 <br>
OpenOS Open Operating System -- Open Contribution Program <br>
## 简介
Open-Script 是一款基于Rust语言开发的开源编程语言,是一款专为Open-OS开发的编程语言. <br>
Open-Script is an open-source programming language based on Rust, specially designed for developing Open-OS. <br>
## 语法
语法使用 标签语法（Tag Syntax）, 标签语法是Open-Script的语法核心, 标签语法的语法规则如下: <br>
Tag Syntax is the core syntax of Open-Script, and the syntax rules of Tag Syntax are as follows: <br>
```
<tag name>content</tag name>
```
在标签语法中, 标签名称是必需的, 标签名称必须以字母开头, 并且只能包含字母, 数字, 下划线和连字符. <br>
In Tag Syntax, the tag name is required, and the tag name must start with a letter and can only contain letters, numbers, underscores and hyphens. <br>
在 Open-Script 中,内置了很多标签, 这些标签可以直接使用, 也可以通过插件来扩展. <br>
In Open-Script, there are many built-in tags, which can be used directly or extended by plugins. <br>

## 标准脚本标签
### 框架
、、、
<filetype !standard-script!>
    <GUI>
        GUI code here
    </GUI>
    <script>
        Script code here
    </script>
</filetype>
、、、

### 内置脚本标签
#### 输出标签
输出标签用于在代码中插入输出, 输出标签的语法如下: <br>
Output tag is used to insert output in the code, and the syntax of output tag is as follows: <br>
、、、
<out>your output here</out>
、、、

#### 文本标签
文本标签用于在代码中插入文本, 文本标签的语法如下: <br>
Text tag is used to insert text in the code, and the syntax of text tag is as follows: <br>
、、、
<text>your text here</text>
、、、

如果需要加入到输出, 可以使用 <out> 标签, 例如: <br>
If you want to add to the output, you can use the <out> tag, for example: <br>
、、、
<out><text>your text here</text></out>
、、、

#### 注释标签
注释标签用于在代码中插入注释, 注释标签的语法如下: <br>
Comment tag is used to insert comments in the code, and the syntax of comment tag is as follows: <br>
、、、
<comment>your comment here</comment>
、、、

#### 变量标签
变量标签用于在代码中插入变量, 变量标签的语法如下: <br>
Variable tag is used to insert variables in the code, and the syntax of variable tag is as follows: <br>
、、、
<var>your variable here</var>
、、、

#### 函数标签
函数标签用于在代码中插入函数, 函数标签的语法如下: <br>
Function tag is used to insert functions in the code, and the syntax of function tag is as follows: <br>
、、、
<func name="your function name here">
    your code here
</func>
、、、

#### 条件标签
条件标签用于在代码中插入条件语句, 条件标签的语法如下: <br>
Condition tag is used to insert condition statements in the code, and the syntax of condition tag is as follows: <br>
、、、
<if condition="your condition here">
    your code here
</if>
<else if condition="your condition here">
    your code here
</else if>
<else>
    your code here
</else>
、、、

#### 循环标签
循环标签用于在代码中插入循环语句, 循环标签的语法如下: <br>
Loop tag is used to insert loop statements in the code, and the syntax of loop tag is as follows: <br>
、、、
<loop cnt=number-or-var>
    your code here
</loop>
、、、

#### 运算标签
运算标签用于在代码中插入运算语句, 运算标签的语法如下: <br>
Operation tag is used to insert operation statements in the code, and the syntax of operation tag is as follows: <br>
、、、
<op>your operation here</op>
、、、

如果需要加入到输出, 可以使用 <out> 标签, 例如: <br>
If you want to add to the output, you can use the <out> tag, for example: <br>
、、、
<out><op>your operation here</op></out>
、、、

#### 类型标签
类型标签用于在代码中插入类型声明, 类型标签的语法如下: <br>
Type tag is used to insert type declarations in the code, and the syntax of type tag is as follows: <br>
、、、
<type var="your-variable-here" type="your-type-here"></type>
、、、

#### 模块标签
模块标签用于在代码中插入模块声明, 模块标签的语法如下: <br>
Module tag is used to insert module declarations in the code, and the syntax of module tag is as follows: <br>
、、、
<module name="your-module-name-here"></module>
、、、

#### 包标签
包标签用于在代码中插入包声明, 包标签的语法如下: <br>
Package tag is used to insert package declarations in the code, and the syntax of package tag is as follows: <br>
、、、
<package name="your-package-name-here"></package>
、、、

#### 注解标签
注解标签用于在代码中插入注解声明, 注解标签的语法如下: <br>
Annotation tag is used to insert annotation declarations in the code, and the syntax of annotation tag is as follows: <br>
、、、
<annotation name="your-annotation-name-here"></annotation>
、、、

#### 宏标签
宏标签用于在代码中插入宏声明, 宏标签的语法如下: <br>
Macro tag is used to insert macro declarations in the code, and the syntax of macro tag is as follows: <br>
、、、
<macro name="your-macro-name-here"></macro>
、、、

#### 函数标签（增强版）
函数标签支持参数、返回值类型标注、以及调用表达式. <br>
Enhanced function tag supports typed parameters, return types, and call expressions. <br>
、、、
// 定义
<func name="add" params="a: int, b: int" return="int">
    <return><op>a + b</op></return>
</func>

// 调用
<call name="add" args="3, 5"></call>

// 无返回值
<func name="greet" params="name: str">
    <out>Hello, <var>name</var>!</out>
</func>
、、、

#### 闭包 / Lambda 标签
闭包标签用于创建匿名函数，可作为值传递. <br>
Closure tag is used to create anonymous functions that can be passed as values. <br>
、、、
// 闭包定义
<closure params="x: int, y: int" return="int">
    <op>x * y</op>
</closure>

// 赋值给变量
<var name="multiply">
    <closure params="a: int, b: int" return="int">
        <op>a * b</op>
    </closure>
</var>

// 立即调用的闭包
<call-closure params="10, 20">
    <closure params="m: int, n: int" return="int">
        <op>m + n</op>
    </closure>
</call-closure>
、、、

#### 循环标签（增强版）
增强循环标签支持 for-each 迭代和 while 条件循环. <br>
Enhanced loop tag supports for-each iteration and while conditional loops. <br>
、、、
// for-each 迭代集合
<for-each item="item" collection="my-list">
    <out><var>item</var></out>
</for-each>

// while 条件循环
<while condition="count > 0">
    <out><var>count</var></out>
    <op>count = count - 1</op>
</while>

// break 和 continue
<loop cnt="100">
    <if condition="i == 50">
        <break></break>
    </if>
</loop>
、、、

#### 匹配标签（模式匹配）
匹配标签用于多分支条件匹配，类似 switch/match. <br>
Match tag is used for multi-branch pattern matching, similar to switch/match. <br>
、、、
<match value="status-code">
    <case pattern="200">
        <out>OK</out>
    </case>
    <case pattern="404">
        <out>Not Found</out>
    </case>
    <case pattern="500">
        <out>Server Error</out>
    </case>
    <default>
        <out>Unknown Status</out>
    </default>
</match>

// 类型匹配
<match-type value="some-value">
    <case-type type="int">
        <out>Integer: <var>some-value</var></out>
    </case-type>
    <case-type type="str">
        <out>String: <var>some-value</var></out>
    </case-type>
    <case-type type="list[int]">
        <out>List with <var>some-value.length</var> items</out>
    </case-type>
</match-type>
、、、

#### 错误处理标签
错误处理标签用于捕获和处理异常. <br>
Error handling tag is used to catch and handle exceptions. <br>
、、、
<try>
    <call name="risky-operation"></call>
    <catch type="io-error">
        <out>IO Error occurred: <var>error.message</var></out>
    </catch>
    <catch type="parse-error">
        <out>Parse Error: <var>error.message</var></out>
    </catch>
    <catch type="*">
        <out>Unknown Error: <var>error.message</var></out>
    </catch>
    <finally>
        <out>Cleanup done.</out>
    </finally>
</try>

// 抛出错误
<throw type="io-error" message="File not found: config.os"></throw>
、、、

### 数据结构标签
#### 结构体标签
结构体标签用于定义复合数据类型. <br>
Struct tag is used to define composite data types. <br>
、、、
// 定义
<struct name="EditorConfig">
    <field name="tab-size" type="int" default="4"></field>
    <field name="font-size" type="int" default="14"></field>
    <field name="theme" type="str" default="dark"></field>
    <field name="auto-save" type="bool" default="true"></field>
</struct>

// 实例化
<new struct="EditorConfig" name="config">
    <set-field name="tab-size" value="2"></set-field>
    <set-field name="font-size" value="16"></set-field>
</new>

// 访问字段
<get-field struct="config" field="theme"></get-field>
<set-field struct="config" field="auto-save" value="false"></set-field>
、、、

#### 枚举标签
枚举标签用于定义枚举类型. <br>
Enum tag is used to define enumeration types. <br>
、、、
<enum name="TokenType">
    <variant name="Identifier" value="str"></variant>
    <variant name="Number" value="int"></variant>
    <variant name="String" value="str"></variant>
    <variant name="Operator" value="str"></variant>
    <variant name="Keyword" value="str"></variant>
    <variant name="EOF"></variant>
</enum>

<enum name="Result">
    <variant name="Ok" value="any"></variant>
    <variant name="Err" value="str"></variant>
</enum>
、、、

#### 数组 / 列表标签
数组标签用于创建和操作列表类型数据. <br>
Array tag is used to create and manipulate list-typed data. <br>
、、、
// 创建
<list name="tokens" type="TokenType">
    <push>token1</push>
    <push>token2</push>
</list>

// 字面量创建
<list name="numbers" values="1, 2, 3, 4, 5"></list>

// 操作
<push list="tokens" value="new-token"></push>
<pop list="tokens" into="last-token"></pop>
<get list="tokens" index="0" into="first-token"></get>
<set list="tokens" index="3" value="replaced-token"></set>
<length list="tokens" into="count"></length>
<contains list="tokens" value="search-token" into="found"></contains>
<slice list="tokens" start="1" end="3" into="sub-list"></slice>
<concat list="list-a" with="list-b" into="merged"></concat>
、、、

#### 映射 / 字典标签
映射标签用于创建和操作键值对类型数据. <br>
Map tag is used to create and manipulate key-value pair data. <br>
、、、
// 创建
<map name="config" key-type="str" value-type="str">
    <set key="theme" value="dark"></set>
    <set key="font" value="JetBrains Mono"></set>
</map>

// 操作
<get map="config" key="theme" into="current-theme"></get>
<set map="config" key="font-size" value="14"></set>
<has map="config" key="theme" into="exists"></has>
<delete map="config" key="font"></delete>
<keys map="config" into="all-keys"></keys>
<values map="config" into="all-values"></values>
<length map="config" into="map-size"></length>
、、、

### 文本处理标签
#### 字符串操作标签
字符串操作标签用于文本解析和处理（编辑器核心需求）. <br>
String operation tags for text parsing and processing (core editor requirement). <br>
、、、
// 基本操作
<str-concat a="Hello" b=" World" into="greeting"></str-concat>
<str-length str="hello" into="len"></str-length>
<str-substr str="Open-Script" start="0" end="4" into="part"></str-substr>
<str-index str="hello world" pattern="world" into="pos"></str-index>

// 分割与合并
<str-split str="a,b,c" separator="," into="parts"></str-split>
<str-join list="parts" separator="-" into="joined"></str-join>

// 替换与转换
<str-replace str="hello world" old="world" new="Open-Script" into="result"></str-replace>
<str-upper str="hello" into="HELLO"></str-upper>
<str-lower str="HELLO" into="hello"></str-lower>
<str-trim str="  hello  " into="hello"></str-trim>

// 正则表达式
<regex-match str="abc123" pattern="[a-z]+[0-9]+" into="matched"></regex-match>
<regex-replace str="abc123xyz" pattern="[0-9]+" replacement="#" into="abc#xyz"></regex-replace>

// 格式化
<str-format template="{}: {} -> {}" args="name, input, output" into="formatted"></str-format>
、、、

### 外部调用标签
#### FFI / Rust 互操作标签
用于从 Open-Script 调用 Rust 编写的底层函数（初代编辑器桥接）. <br>
Used to call low-level Rust functions from Open-Script (bridge for Gen 1 editor). <br>
、、、
// 声明外部函数签名
<extern name="rust_file_read" lib="os-runtime" return="str">
    <param name="path" type="str"></param>
</extern>

// 调用外部函数
<call-extern name="rust_file_read" args="'/path/to/file.os'"></call-extern>

// 批量导入 Rust 模块
<import-rust module="os_io::filesystem" alias="fs">
    <function name="read"></function>
    <function name="write"></function>
    <function name="exists"></function>
</import-rust>
<call-extern name="fs::read" args="'config.os'"></call-extern>

// 注册回调（编辑器事件）
<register-callback event="on-key-press" handler="handle-key-press"></register-callback>
<register-callback event="on-content-change" handler="handle-content-change"></register-callback>
、、、

### 迭代与流处理标签
#### 迭代器标签
迭代器标签用于函数式数据处理，适合编译器/解析器流程. <br>
Iterator tags for functional data processing, suitable for compiler/parser pipelines. <br>
、、、
// map - 转换每个元素
<map-into list="tokens" func="token-to-string" into="strings"></map-into>

// filter - 过滤元素
<filter-into list="tokens" func="is-keyword" into="keywords"></filter-into>

// reduce - 聚合
<reduce list="numbers" func="sum" init="0" into="total"></reduce>

// 链式操作
<chain from="source-code">
    <step type="tokenize"></step>
    <step type="parse"></step>
    <step type="analyze"></step>
    <step type="generate"></step>
    <collect into="compiled-output"></collect>
</chain>
、、、

### 图形界面标签
官方不定义图形界面标签, 需要自行开发图形界面配置文件(.sty). <br>
The official does not define the GUI tag, and the GUI configuration file (.sty) needs to be developed by itself. <br>

## 系统与标准库标签
系统标签提供操作系统级别的能力，是构建编辑器和实现自举的关键. <br>
System tags provide OS-level capabilities, essential for building editors and achieving bootstrapping. <br>

### 文件操作标签
文件操作标签用于读写和管理文件. <br>
File operation tags are used to read, write, and manage files. <br>
、、、
// 读取文件
<file-read path="source/tokenizer.os" into="source-code"></file-read>
<file-read-binary path="data.bin" into="raw-bytes"></file-read-binary>

// 写入文件
<file-write path="output/compiled.os" content="compiled-code"></file-write>
<file-append path="log.txt" content="Build completed at 14:30"></file-append>
<file-write-binary path="output.bin" content="binary-data"></file-write-binary>

// 文件信息与检查
<file-exists path="config.os" into="config-exists"></file-exists>
<file-size path="config.os" into="config-size"></file-size>
<file-modified path="config.os" into="last-modified"></file-modified>
<file-is-dir path="source/" into="is-dir"></file-is-dir>

// 文件操作
<file-copy from="old.os" to="new.os"></file-copy>
<file-move from="old.os" to="new-location.os"></file-move>
<file-delete path="temp.os"></file-delete>

// 批量文件操作
<file-glob pattern="source/**/*.os" into="all-sources"></file-glob>
<file-watch path="source/" event="on-change" handler="recompile"></file-watch>
、、、

### 目录操作标签
目录操作标签用于管理文件夹结构. <br>
Directory operation tags are used to manage folder structures. <br>
、、、
<dir-list path="source/" into="files"></dir-list>
<dir-list-recursive path="source/" into="all-files"></dir-list-recursive>
<dir-create path="output/build/"></dir-create>
<dir-delete path="temp/" recursive="true"></dir-delete>
<dir-current into="cwd"></dir-current>
<dir-change path="source/"></dir-change>
<dir-temp into="tmp-path"></dir-temp>
<dir-home into="home-path"></dir-home>
、、、

### 进程操作标签
进程操作标签用于编译、运行外部程序及管理子进程. <br>
Process tags are used for compiling, running external programs, and managing sub-processes. <br>
、、、
// 执行命令（同步）
<exec command="rustc" args="main.rs -o main" into-result="compile-result">
    <on-success>
        <out>Compilation successful</out>
    </on-success>
    <on-error>
        <out>Compilation failed: <var>compile-result.stderr</var></out>
    </on-error>
</exec>

// 异步执行（不阻塞UI）
<spawn command="cargo" args="build --release">
    <on-stdout>
        <append-each list="build-log"></append-each>
    </on-stdout>
    <on-stderr>
        <append-each list="build-errors"></append-each>
    </on-stderr>
    <on-exit>
        <out>Build process exited with code: <var>exit-code</var></out>
    </on-exit>
</spawn>

// 进程管理
<process-kill pid="child-pid"></process-kill>
<process-wait pid="child-pid" timeout="30000"></process-wait>
<process-list into="running-processes"></process-list>

// 运行 Open-Script 自身（自举关键）
<exec-self args="compile source/main.os -o target/main"></exec-self>
、、、

### 环境与参数标签
、、、
// 环境变量
<env-get name="HOME" into="home-var"></env-get>
<env-set name="OS_EDITOR_PATH" value="/usr/local/osedit"></env-set>
<env-list into="all-env"></env-list>

// 命令行参数
<arg-get index="0" into="script-name"></arg-get>
<arg-get index="1" into="first-arg"></arg-get>
<arg-count into="arg-count"></arg-count>

// 程序控制
<exit code="0"></exit>
<sleep ms="1000"></sleep>
、、、

### 网络操作标签
网络标签用于获取远程依赖、更新和协作. <br>
Network tags for fetching remote dependencies, updates, and collaboration. <br>
、、、
// HTTP 请求
<http-get url="https://api.example.com/packages" into="response" headers="auth-map"></http-get>
<http-post url="https://api.example.com/submit" body="payload-json" into="response"></http-post>
<http-download url="https://cdn.example.com/lib.os" to="libs/dependency.os"></http-download>

// WebSocket（实时协作编辑）
<ws-connect url="wss://collab.example.com/session" name="collab-socket">
    <on-message>
        <out><var>message.data</var></out>
    </on-message>
    <on-close>
        <out>Collaboration session ended</out>
    </on-close>
</ws-connect>
<ws-send socket="collab-socket" message="edit-data"></ws-send>
<ws-close socket="collab-socket"></ws-close>
、、、

### 解析器标签（编译器核心）
解析器标签为 Open-Script 自身的词法和语法分析提供内置支持. <br>
Parser tags provide built-in support for Open-Script's own lexing and parsing. <br>
、、、
// 词法分析
<tokenize source="source-code" into="tokens">
    <rule pattern="<[a-zA-Z][a-zA-Z0-9_-]*" type="TagOpen"></rule>
    <rule pattern="</[a-zA-Z][a-zA-Z0-9_-]*>" type="TagClose"></rule>
    <rule pattern="[a-zA-Z_][a-zA-Z0-9_]*" type="Identifier"></rule>
    <rule pattern="[0-9]+" type="Number"></rule>
    <rule pattern=""[^"]*"" type="StringLiteral"></rule>
    <rule pattern="<!--.*?-->" type="Comment"></rule>
</tokenize>

// 语法解析
<parse tokens="tokens" grammar="open-script-grammar" into="ast">
    <ast-node type="TagNode">
        <property name="tag-name" value="if"></property>
        <child type="AttributeNode"></child>
        <child type="BodyNode"></child>
    </ast-node>
</parse>

// AST 操作
<ast-walk node="root-ast" visitor="my-visitor"></ast-walk>
<ast-transform node="root-ast" rule="optimize"></ast-transform>
<ast-to-source ast="root-ast" into="re-generated-code"></ast-to-source>
、、、

### 编辑器专用标签
这些标签为编辑器功能提供原生支持（文本缓冲区、光标、选区、语法高亮）. <br>
These tags provide native support for editor functionality (text buffer, cursor, selection, syntax highlighting). <br>
、、、
// 文本缓冲区
<buffer-open path="file.os" into="buffer"></buffer-open>
<buffer-create text="hello world" into="buffer"></buffer-create>
<buffer-insert buffer="buf" pos="42" text="new code"></buffer-insert>
<buffer-delete buffer="buf" start="10" end="20"></buffer-delete>
<buffer-replace buffer="buf" start="0" end="5" text="replaced"></buffer-replace>
<buffer-text buffer="buf" into="full-text"></buffer-text>
<buffer-line buffer="buf" line="5" into="line-text"></buffer-line>
<buffer-line-count buffer="buf" into="lines"></buffer-line-count>
<buffer-save buffer="buf" path="file.os"></buffer-save>
<buffer-undo buffer="buf"></buffer-undo>
<buffer-redo buffer="buf"></buffer-redo>

// 光标操作
<cursor-move buffer="buf" line="10" column="5"></cursor-move>
<cursor-pos buffer="buf" into="cursor"></cursor-pos>
<cursor-line buffer="buf" into="current-line"></cursor-line>

// 选区操作
<selection-set buffer="buf" start-line="1" start-col="0" end-line="10" end-col="20"></selection-set>
<selection-text buffer="buf" into="selected-text"></selection-text>
<selection-replace buffer="buf" text="new block"></selection-replace>

// 语法高亮
<highlight-rule language="open-script">
    <token type="TagOpen" color="var(--accent)" weight="bold"></token>
    <token type="Identifier" color="var(--text-secondary)"></token>
    <token type="Keyword" color="var(--accent)"></token>
    <token type="StringLiteral" color="var(--color-success)"></token>
    <token type="Number" color="var(--color-warning)"></token>
    <token type="Comment" color="var(--text-tertiary)" style="italic"></token>
</highlight-rule>

// 事件绑定
<on-event event="buffer-changed" handler="on-content-change"></on-event>
<on-event event="cursor-moved" handler="on-cursor-move"></on-event>
<on-event event="file-opened" handler="on-file-open"></on-event>
、、、

## 图形界面配置文件标签
### 框架
、、、
<filetype !style-file!>
    UI Element NAME {
        Attribute NAME: Attribute VALUE;
        ...
    }
</filetype>
、、、

### 内置标签
#### 元素标签
元素标签用于定义图形界面元素, 元素标签的语法如下: <br>
Element tag is used to define GUI elements, and the syntax of element tag is as follows: <br>
、、、
UI Element NAME {
   Attribute code here;
}
、、、

#### 属性标签
属性标签用于定义图形界面元素的属性, 属性标签的语法如下: <br>
Attribute tag is used to define attributes of GUI elements, and the syntax of attribute tag is as follows: <br>
、、、
Attribute NAME: Attribute VALUE;
、、、

以下是所有可用属性的分类参考: <br>
The following is a categorized reference of all available attributes: <br>

#### 文本与字体 (Text & Font)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `color` | 文字颜色 | `red`、`#FF0000`、`rgb(255,0,0)` |
| `font-family` | 字体名称 | `"微软雅黑"`、`Arial, sans-serif` |
| `font-size` | 字号大小 | `16px`、`1.2rem`、`120%` |
| `font-weight` | 字重/粗细 | `normal`、`bold`、`400`、`700` |
| `font-style` | 字体样式 | `normal`、`italic`、`oblique` |
| `font-variant` | 字体变体 | `normal`、`small-caps` |
| `text-align` | 水平对齐 | `left`、`center`、`right`、`justify` |
| `text-align-last` | 末行对齐 | `left`、`center`、`right`、`justify` |
| `text-decoration` | 文字装饰线 | `none`、`underline`、`overline`、`line-through` |
| `text-decoration-color` | 装饰线颜色 | `red`、`#FF0000` |
| `text-decoration-line` | 装饰线类型 | `underline`、`overline`、`line-through` |
| `text-decoration-style` | 装饰线样式 | `solid`、`double`、`dotted`、`dashed`、`wavy` |
| `text-shadow` | 文字阴影 | `2px 2px 4px rgba(0,0,0,0.5)` |
| `text-transform` | 大小写转换 | `none`、`uppercase`、`lowercase`、`capitalize` |
| `text-indent` | 首行缩进 | `2em`、`32px` |
| `text-overflow` | 溢出文字处理 | `clip`、`ellipsis` |
| `text-justify` | 两端对齐方式 | `auto`、`inter-word`、`inter-character` |
| `word-spacing` | 单词间距 | `normal`、`4px` |
| `letter-spacing` | 字符间距 | `normal`、`2px` |
| `line-height` | 行高 | `1.5`、`24px`、`150%` |
| `white-space` | 空白符处理 | `normal`、`nowrap`、`pre`、`pre-wrap` |
| `word-break` | 单词换行规则 | `normal`、`break-all`、`keep-all` |
| `word-wrap` / `overflow-wrap` | 长单词换行 | `normal`、`break-word` |
| `direction` | 文字方向 | `ltr`、`rtl` |
| `unicode-bidi` | 双向文本控制 | `normal`、`embed`、`bidi-override` |
| `writing-mode` | 书写模式 | `horizontal-tb`、`vertical-rl`、`vertical-lr` |
| `text-orientation` | 文字朝向 | `mixed`、`upright`、`sideways` |
| `text-combine-upright` | 竖排合并 | `none`、`all`、`digits 2` |
| `hanging-punctuation` | 标点悬挂 | `none`、`first`、`last`、`allow-end` |
| `hyphens` | 连字符断词 | `none`、`manual`、`auto` |
| `tab-size` | Tab键宽度 | `4`、`8` |
| `font-kerning` | 字距调整 | `auto`、`normal`、`none` |
| `font-stretch` | 字体拉伸 | `normal`、`condensed`、`expanded` |

#### 背景 (Background)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `background-color` | 背景颜色 | `white`、`#F0F0F0`、`rgba(0,0,0,0.5)` |
| `background-image` | 背景图片 | `url("bg.png")`、`linear-gradient(red, blue)` |
| `background-repeat` | 平铺方式 | `repeat`、`no-repeat`、`repeat-x`、`repeat-y`、`space`、`round` |
| `background-position` | 位置 | `center`、`top right`、`10px 20px`、`50% 50%` |
| `background-size` | 尺寸 | `auto`、`cover`、`contain`、`100px 100px`、`50%` |
| `background-attachment` | 滚动行为 | `scroll`、`fixed`、`local` |
| `background-origin` | 背景定位原点 | `padding-box`、`border-box`、`content-box` |
| `background-clip` | 背景裁剪区域 | `border-box`、`padding-box`、`content-box`、`text` |
| `background-blend-mode` | 混合模式 | `normal`、`multiply`、`screen`、`overlay`、`darken`、`lighten` |
| `background` | 简写（一次性设置） | `#FFF url("bg.png") no-repeat center/cover` |

#### 盒子模型 (Box Model)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `width` | 内容宽度 | `200px`、`50%`、`100vw`、`auto`、`max-content`、`min-content` |
| `height` | 内容高度 | `100px`、`50vh`、`auto`、`max-content` |
| `min-width` | 最小宽度 | `100px`、`50%` |
| `max-width` | 最大宽度 | `800px`、`100%` |
| `min-height` | 最小高度 | `100px`、`50vh` |
| `max-height` | 最大高度 | `800px`、`100vh` |
| `block-size` | 逻辑块方向尺寸（代替height） | `100px`、`auto` |
| `inline-size` | 逻辑行方向尺寸（代替width） | `200px`、`auto` |
| `min-block-size` | 最小块方向尺寸 | `100px` |
| `max-block-size` | 最大块方向尺寸 | `800px` |
| `min-inline-size` | 最小行方向尺寸 | `100px` |
| `max-inline-size` | 最大行方向尺寸 | `800px` |
| `padding-top` | 上内边距 | `10px`、`1em`、`5%` |
| `padding-bottom` | 下内边距 | `10px`、`1em`、`5%` |
| `padding-left` | 左内边距 | `10px`、`1em`、`5%` |
| `padding-right` | 右内边距 | `10px`、`1em`、`5%` |
| `padding-block-start` | 逻辑块起始内边距 | `10px` |
| `padding-block-end` | 逻辑块结束内边距 | `10px` |
| `padding-inline-start` | 逻辑行起始内边距 | `10px` |
| `padding-inline-end` | 逻辑行结束内边距 | `10px` |
| `padding` | 简写（上右下左） | `10px`、`10px 20px`、`10px 20px 30px 40px` |
| `margin-top` | 上外边距 | `10px`、`auto` |
| `margin-bottom` | 下外边距 | `10px`、`auto` |
| `margin-left` | 左外边距 | `10px`、`auto` |
| `margin-right` | 右外边距 | `10px`、`auto` |
| `margin-block-start` | 逻辑块起始外边距 | `10px`、`auto` |
| `margin-block-end` | 逻辑块结束外边距 | `10px`、`auto` |
| `margin-inline-start` | 逻辑行起始外边距 | `10px`、`auto` |
| `margin-inline-end` | 逻辑行结束外边距 | `10px`、`auto` |
| `margin` | 简写（上右下左） | `10px`、`10px auto`、`10px 20px 30px 40px` |
| `border-width` | 边框宽度（四边统一） | `1px`、`2px`、`medium`、`thick` |
| `border-style` | 边框样式（四边统一） | `solid`、`dashed`、`dotted`、`double`、`none`、`hidden`、`groove`、`ridge`、`inset`、`outset` |
| `border-color` | 边框颜色（四边统一） | `black`、`#333`、`transparent` |
| `border-top` / `-bottom` / `-left` / `-right` | 单边边框简写 | `1px solid #000` |
| `border-block` | 逻辑块方向边框简写 | `1px solid #000` |
| `border-inline` | 逻辑行方向边框简写 | `1px solid #000` |
| `border-radius` | 圆角（四角统一） | `8px`、`50%`、`8px 16px` |
| `border-top-left-radius` | 左上圆角 | `8px`、`50%` |
| `border-top-right-radius` | 右上圆角 | `8px`、`50%` |
| `border-bottom-left-radius` | 左下圆角 | `8px`、`50%` |
| `border-bottom-right-radius` | 右下圆角 | `8px`、`50%` |
| `border-image` | 边框图片 | `url("border.png") 30 30 round` |
| `box-sizing` | 盒模型计算方式 | `content-box`（默认）、`border-box` |
| `outline` | 轮廓（不占空间） | `2px solid red` |
| `outline-color` | 轮廓颜色 | `red` |
| `outline-style` | 轮廓样式 | `solid`、`dashed`、`dotted` |
| `outline-width` | 轮廓宽度 | `2px`、`medium` |
| `outline-offset` | 轮廓偏移 | `2px` |

#### 显示与布局 (Display & Layout)

##### 核心显示模式
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `display` | 显示模式 | `block`、`inline`、`inline-block`、`flex`、`grid`、`table`、`none`、`contents` |
| `visibility` | 可见性（仍占空间） | `visible`、`hidden`、`collapse` |
| `opacity` | 透明度（0~1） | `0`、`0.5`、`1` |
| `overflow` | 溢出处理（四边统一） | `visible`、`hidden`、`scroll`、`auto` |
| `overflow-x` | 水平溢出 | `visible`、`hidden`、`scroll`、`auto` |
| `overflow-y` | 垂直溢出 | `visible`、`hidden`、`scroll`、`auto` |
| `overflow-block` | 逻辑块方向溢出 | `visible`、`hidden`、`scroll`、`auto` |
| `overflow-inline` | 逻辑行方向溢出 | `visible`、`hidden`、`scroll`、`auto` |
| `overflow-wrap` | 长单词换行 | `normal`、`break-word` |
| `clip-path` | 裁剪路径 | `inset(10px)`、`circle(50%)`、`polygon(...)` |
| `object-fit` | 替换元素内容适配 | `fill`、`contain`、`cover`、`none`、`scale-down` |
| `object-position` | 替换元素内容位置 | `center`、`50% 50%`、`top left` |

##### 定位 (Position)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `position` | 定位方式 | `static`、`relative`、`absolute`、`fixed`、`sticky` |
| `top` | 距上边距离 | `10px`、`10%`、`auto` |
| `bottom` | 距下边距离 | `10px`、`10%`、`auto` |
| `left` | 距左边距离 | `10px`、`10%`、`auto` |
| `right` | 距右边距离 | `10px`、`10%`、`auto` |
| `inset` | 简写（top/right/bottom/left） | `10px`、`10px 20px`、`10px 20px 30px 40px` |
| `inset-block-start` | 逻辑块起始偏移 | `10px`、`auto` |
| `inset-block-end` | 逻辑块结束偏移 | `10px`、`auto` |
| `inset-inline-start` | 逻辑行起始偏移 | `10px`、`auto` |
| `inset-inline-end` | 逻辑行结束偏移 | `10px`、`auto` |
| `z-index` | 层叠顺序 | `1`、`100`、`9999`、`auto` |

##### 浮动 (Float)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `float` | 浮动 | `none`、`left`、`right`、`inline-start`、`inline-end` |
| `clear` | 清除浮动 | `none`、`left`、`right`、`both`、`inline-start`、`inline-end` |

##### 弹性盒子 (Flexbox)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `flex-direction` | 主轴方向 | `row`、`row-reverse`、`column`、`column-reverse` |
| `flex-wrap` | 换行 | `nowrap`、`wrap`、`wrap-reverse` |
| `flex-flow` | 简写（direction + wrap） | `row wrap`、`column nowrap` |
| `flex` | 简写（grow + shrink + basis） | `1`、`1 0 auto`、`0 0 100px` |
| `flex-grow` | 扩张系数 | `0`、`1`、`2` |
| `flex-shrink` | 收缩系数 | `0`、`1`、`2` |
| `flex-basis` | 基础尺寸 | `auto`、`100px`、`50%` |
| `justify-content` | 主轴对齐 | `flex-start`、`flex-end`、`center`、`space-between`、`space-around`、`space-evenly` |
| `align-items` | 交叉轴对齐 | `flex-start`、`flex-end`、`center`、`baseline`、`stretch` |
| `align-content` | 多行交叉轴对齐 | `flex-start`、`flex-end`、`center`、`space-between`、`space-around`、`stretch` |
| `align-self` | 单个项目对齐（覆盖align-items） | `auto`、`flex-start`、`flex-end`、`center`、`baseline`、`stretch` |
| `order` | 排序 | `0`、`-1`、`1`、`2` |
| `gap` | 间距（行+列） | `10px`、`10px 20px` |
| `row-gap` | 行间距 | `10px` |
| `column-gap` | 列间距 | `10px` |

##### 网格 (Grid)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `grid-template-columns` | 列模板 | `1fr 2fr 1fr`、`repeat(3, 100px)`、`auto 1fr` |
| `grid-template-rows` | 行模板 | `100px auto 50px`、`repeat(2, 1fr)` |
| `grid-template-areas` | 区域模板 | `"header header" "sidebar main" "footer footer"` |
| `grid-template` | 简写（rows + columns + areas） | `"header" 100px / 1fr` |
| `grid-auto-columns` | 自动列尺寸 | `100px`、`1fr`、`minmax(100px, 1fr)` |
| `grid-auto-rows` | 自动行尺寸 | `100px`、`1fr`、`minmax(100px, 1fr)` |
| `grid-auto-flow` | 自动流方向 | `row`、`column`、`row dense`、`column dense` |
| `grid-column-start` | 列起始线 | `1`、`span 2`、`-1` |
| `grid-column-end` | 列结束线 | `3`、`span 2`、`-1` |
| `grid-column` | 简写（start / end） | `1 / 3`、`span 2 / -1` |
| `grid-row-start` | 行起始线 | `1`、`span 2` |
| `grid-row-end` | 行结束线 | `3`、`span 2` |
| `grid-row` | 简写（start / end） | `1 / 3`、`span 2 / -1` |
| `grid-area` | 区域（可配合grid-template-areas） | `"header"`、`1 / 1 / 3 / 3` |
| `justify-items` | 网格内项目行轴对齐 | `start`、`end`、`center`、`stretch` |
| `align-items` | 网格内项目列轴对齐 | `start`、`end`、`center`、`stretch` |
| `place-items` | 简写（align-items + justify-items） | `center stretch`、`start` |
| `justify-self` | 单个项目行轴对齐 | `start`、`end`、`center`、`stretch` |
| `align-self` | 单个项目列轴对齐 | `start`、`end`、`center`、`stretch` |
| `place-self` | 简写（align-self + justify-self） | `center stretch`、`start` |
| `justify-content` | 网格整体行轴对齐 | `start`、`end`、`center`、`stretch`、`space-between`、`space-around`、`space-evenly` |
| `align-content` | 网格整体列轴对齐 | `start`、`end`、`center`、`stretch`、`space-between`、`space-around`、`space-evenly` |
| `place-content` | 简写（align-content + justify-content） | `center stretch`、`start` |

##### 多列 (Multi-column)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `column-count` | 列数 | `2`、`3`、`auto` |
| `column-width` | 列宽 | `200px`、`auto` |
| `columns` | 简写（width + count） | `200px 3`、`auto 2` |
| `column-gap` | 列间距 | `20px`、`normal` |
| `column-rule` | 列间分割线简写 | `1px solid #ccc` |
| `column-rule-width` | 分割线宽度 | `1px`、`medium` |
| `column-rule-style` | 分割线样式 | `solid`、`dashed`、`dotted` |
| `column-rule-color` | 分割线颜色 | `#ccc`、`red` |
| `column-span` | 跨列 | `none`、`all` |
| `column-fill` | 列填充方式 | `balance`、`auto` |

##### 表格 (Table)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `table-layout` | 表格布局算法 | `auto`、`fixed` |
| `border-collapse` | 边框合并 | `collapse`、`separate` |
| `border-spacing` | 边框间距 | `2px`、`2px 4px` |
| `caption-side` | 表格标题位置 | `top`、`bottom`、`block-start`、`block-end`、`inline-start`、`inline-end` |
| `empty-cells` | 空单元格边框显示 | `show`、`hide` |

#### 视觉效果 (Effects)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `box-shadow` | 盒子阴影 | `2px 2px 8px rgba(0,0,0,0.3)`、`inset 0 0 10px #000` |
| `filter` | 滤镜 | `blur(5px)`、`brightness(150%)`、`grayscale(100%)`、`drop-shadow(2px 2px 4px #000)` |
| `backdrop-filter` | 背景滤镜（元素后面） | `blur(10px)`、`brightness(80%)` |
| `mix-blend-mode` | 元素混合模式 | `normal`、`multiply`、`screen`、`overlay`、`darken`、`lighten`、`difference`、`exclusion` |
| `isolation` | 隔离（创建独立堆叠上下文） | `auto`、`isolate` |
| `box-decoration-break` | 盒子装饰断裂处理 | `slice`、`clone` |
| `image-rendering` | 图像渲染算法 | `auto`、`crisp-edges`、`pixelated` |
| `image-orientation` | 图像方向 | `from-image`、`0deg`、`90deg` |
| `image-resolution` | 图像分辨率 | `from-image`、`300dpi` |
| `mask` | 遮罩（简写） | `url("mask.png") center/cover`、`linear-gradient(black, transparent)` |
| `mask-image` | 遮罩图片 | `url("mask.png")`、`linear-gradient(black, transparent)` |
| `mask-mode` | 遮罩模式 | `match-source`、`alpha`、`luminance` |
| `mask-repeat` | 遮罩平铺 | `repeat`、`no-repeat` |
| `mask-position` | 遮罩位置 | `center`、`50% 50%` |
| `mask-size` | 遮罩尺寸 | `cover`、`contain`、`100% 100%` |
| `mask-composite` | 遮罩组合 | `add`、`subtract`、`intersect`、`exclude` |
| `mask-border` | 遮罩边框 | `url("mask.png") 30 stretch` |

#### 过渡与动画 (Transitions & Animations)

##### 过渡 (Transition)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `transition-property` | 哪些属性产生过渡 | `all`、`opacity`、`transform, background`、`none` |
| `transition-duration` | 过渡持续时间 | `0.3s`、`300ms` |
| `transition-timing-function` | 缓动函数 | `ease`、`linear`、`ease-in`、`ease-out`、`ease-in-out`、`cubic-bezier(0.25,0.1,0.25,1)`、`steps(4, end)` |
| `transition-delay` | 过渡延迟时间 | `0s`、`200ms` |
| `transition` | 简写（property + duration + timing + delay） | `all 0.3s ease 0s`、`opacity 0.5s linear` |

##### 动画 (Animation)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `animation-name` | 动画名称（对应@keyframes） | `slideIn`、`fadeOut`、`none` |
| `animation-duration` | 单次动画时长 | `0.5s`、`1s`、`200ms` |
| `animation-timing-function` | 缓动函数 | 同transition |
| `animation-delay` | 动画延迟 | `0s`、`0.5s`、`-1s`（负值立即跳到中间） |
| `animation-iteration-count` | 重复次数 | `1`、`2`、`infinite` |
| `animation-direction` | 方向 | `normal`、`reverse`、`alternate`、`alternate-reverse` |
| `animation-fill-mode` | 填充模式 | `none`、`forwards`、`backwards`、`both` |
| `animation-play-state` | 运行状态 | `running`、`paused` |
| `animation` | 简写（全部） | `slideIn 0.5s ease 0s 1 normal forwards` |

##### @keyframes 关键帧语法
、、、
@keyframes slideIn {
    from {
        transform: translateX(-100%);
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}

@keyframes pulse {
    0% { transform: scale(1); }
    50% { transform: scale(1.1); }
    100% { transform: scale(1); }
}
、、、

#### 响应式与交互 (Responsive & Interactive)

##### @media 媒体查询语法
、、、
/* 根据视口宽度 */
@media (max-width: 768px) { ... }

/* 根据设备类型 */
@media (orientation: portrait) { ... }
@media (prefers-color-scheme: dark) { ... }
@media (prefers-reduced-motion: reduce) { ... }
@media (any-hover: hover) { ... }
@media (pointer: fine) { ... }
、、、

##### 交互相关属性
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `cursor` | 鼠标指针样式 | `pointer`、`default`、`wait`、`move`、`not-allowed`、`grab`、`zoom-in` |
| `pointer-events` | 鼠标事件是否穿透 | `auto`、`none` |
| `touch-action` | 触摸手势行为 | `auto`、`none`、`pan-x`、`pan-y`、`pinch-zoom`、`manipulation` |
| `user-select` | 是否允许选中文本 | `auto`、`none`、`text`、`all`、`contain` |
| `scroll-behavior` | 滚动行为 | `auto`、`smooth` |
| `overscroll-behavior` | 滚动边界行为 | `auto`、`contain`、`none` |
| `scroll-margin` | 滚动边距 | `10px`、`10px 20px` |
| `scroll-padding` | 滚动容器内边距 | `10px`、`10px 20px` |
| `scroll-snap-align` | 滚动吸附对齐 | `none`、`start`、`end`、`center` |
| `scroll-snap-type` | 滚动吸附类型 | `none`、`x`、`y`、`block`、`inline`、`both` |
| `scroll-snap-stop` | 滚动吸附停止 | `normal`、`always` |
| `resize` | 是否允许用户调整大小 | `none`、`both`、`horizontal`、`vertical` |
| `appearance` | 移除浏览器默认样式 | `none`、`auto`、`button`、`checkbox`、`listbox` |

##### 容器查询 (Container Queries)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `container-type` | 容器查询类型 | `normal`、`size`、`inline-size`、`block-size`、`style` |
| `container-name` | 容器名称 | `sidebar`、`main-content` |
| `container` | 简写（type + name） | `inline-size / sidebar` |

#### 打印与分页 (Print & Paged Media)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `page-break-before` | 分页前 | `auto`、`always`、`avoid`、`left`、`right` |
| `page-break-after` | 分页后 | `auto`、`always`、`avoid`、`left`、`right` |
| `page-break-inside` | 内部避免分页 | `auto`、`avoid` |
| `orphans` | 孤儿行（段落开头保留行数） | `2`、`3` |
| `widows` | 寡妇行（段落结尾保留行数） | `2`、`3` |

#### CSS 变量 (Custom Properties)
、、、
:root {
    --primary-color: #3498db;
    --spacing: 16px;
    --font-size: 14px;
}

Button {
    background: var(--primary-color);
    padding: var(--spacing);
    font-size: var(--font-size);
}

/* 带后备值 */
Text {
    color: var(--text-color, #333);
}
、、、

| 属性 | 作用 | 示例值 |
|------|------|--------|
| `var(--变量名)` | 引用CSS变量 | `var(--primary-color)` |
| `var(--变量名, 后备值)` | 引用变量并带后备 | `var(--text-color, #333)` |

#### 其他杂项 (Miscellaneous)
| 属性 | 作用 | 示例值 |
|------|------|--------|
| `content` | 伪元素内容（::before/::after） | `"→"`、`attr(data-label)`、`url("icon.png")`、`counter(step)` |
| `counter-increment` | 计数器递增 | `step 1`、`step -1`、`none` |
| `counter-reset` | 计数器重置 | `step 0`、`step`、`none` |
| `counter-set` | 计数器设置 | `step 5`、`none` |
| `quotes` | 引号样式 | `"“" "”" "‘" "’"` |
| `will-change` | 提示浏览器即将变化的属性 | `transform`、`opacity`、`scroll-position`、`contents` |
| `contain` | 隔离布局计算（性能优化） | `none`、`layout`、`paint`、`size`、`style`、`strict`、`content` |
| `content-visibility` | 延迟渲染（性能优化） | `visible`、`auto`、`hidden` |
| `all` | 重置所有属性 | `initial`、`inherit`、`unset`、`revert`、`revert-layer` |
| `unset` | 属性值（继承则inherit，否则initial） | 属性值之一 |
| `initial` | 属性值（恢复浏览器默认） | 属性值之一 |
| `inherit` | 属性值（强制继承父级） | 属性值之一 |
| `revert` | 属性值（恢复用户代理样式） | 属性值之一 |
| `revert-layer` | 属性值（恢复上一层级联） | 属性值之一 |

#### 属性分类统计
| 分类 | 数量（约） |
|------|-----------|
| 文本与字体 | 30 |
| 背景 | 10 |
| 盒子模型 | 40 |
| 显示与布局 | 70 |
| 视觉效果 | 20 |
| 过渡与动画 | 15 |
| 响应式与交互 | 15 |
| 打印与分页 | 5 |
| CSS变量 | 2 |
| 其他杂项 | 15 |
| **合计** | **约 220+ 个** |

---

## 自举路线图 (Bootstrapping Roadmap)

Open-Script 的终极目标是实现**自举（Self-hosting）**：用 Open-Script 语言自身编写 Open-Script 的编辑器和编译器. <br>
The ultimate goal of Open-Script is to achieve **self-hosting**: writing the Open-Script editor and compiler in Open-Script itself. <br>

### 三代编辑器演进
```
┌─────────────────────────────────────────────────────────────────┐
│  Gen 1 (Rust)          Gen 2 (Open-Script)      Gen 3 (Self)   │
│  ┌───────────┐         ┌───────────┐            ┌───────────┐  │
│  │ Rust 编写  │ ──编译──→│ .os 编写   │ ──自编译──→│ 完全自举   │  │
│  │ 初代编辑器 │         │ 二代编辑器  │            │ 三代编辑器  │  │
│  │           │         │ 功能完整    │            │ 进化可迭代  │  │
│  └───────────┘         └───────────┘            └───────────┘  │
│  功能:                 功能:                    功能:           │
│  • Open-Script 解释器  • 完全编辑器功能          • 即 N 代编辑器 │
│  • 基础代码编辑        • 语法高亮 + 补全         • 自身修改自身  │
│  • 语法高亮            • 项目管理                • 无外部依赖    │
│  • Rust FFI 桥接       • 可编译 .os → 可执行文件  • 真正的自举    │
│  • 编译 .os 文件        • 可运行并编译自身         • 语言持续进化  │
│  • 基础 GUI 渲染                                      │
└─────────────────────────────────────────────────────────────────┘
```

### Gen 1 — Rust 初代编辑器 (os-edit-rs)
**语言**: Rust  <br>
**目标**: 提供一个能编辑、解析、运行 Open-Script 的开发环境. <br>
**核心能力**: <br>
- Open-Script 词法解析器（Tokenizer）和语法解析器（Parser）<br>
- 标签语法 AST 构建与遍历 <br>
- 基础代码编辑器（文本编辑、保存、语法高亮）<br>
- Open-Script 解释器 / 字节码编译器 <br>
- Rust ↔ Open-Script FFI 桥接层 <br>
- NOTHING UI 风格 GUI (使用 egui/iced/wgpu) <br>
- 编译 `.os` 文件为可执行程序 <br>

```
Gen 1 的使命：让 Open-Script 可以运行，并能够编译 Open-Script 写的程序。
一旦 Gen 1 稳定，就可以用 Open-Script 语法编写 Gen 2 编辑器。
```

### Gen 2 — Open-Script 二代编辑器 (os-edit-os)
**语言**: Open-Script（由 Gen 1 编译） <br>
**目标**: 用 Open-Script 编写功能完整的编辑器，实现语言自我验证. <br>
**核心能力**: <br>
- 完整文本编辑器（多文件、多标签页、撤销/重做、搜索替换）<br>
- 自举编译器前身（可解析 Open-Script 源码，生成 Gen 1 兼容的 AST）<br>
- 项目管理（创建、导入、编译、运行项目）<br>
- 实时语法高亮与代码补全 <br>
- 集成终端（通过系统标签执行外部命令）<br>
- 插件系统基础框架 <br>
- 可通过 `<exec-self>` 编译自身 <br>

```
Gen 2 的使命：证明 Open-Script 语言是图灵完备的，可以编写真实世界的应用程序。
Gen 2 不是完全自举的——它仍依赖 Gen 1 的运行时。但它已经能够编译自身代码。
```

### Gen 3 — 完全自举编辑器 (os-edit-self)
**语言**: Open-Script（由 Gen 2 编译） <br>
**目标**: 实现真正的自举——修改自身源码 → 编译自身 → 替换正在运行的编辑器. <br>
**核心能力**: <br>
- Gen 2 的所有功能 <br>
- 自我编译（编译自身源码，生成自己的可执行文件）<br>
- 热重载（修改源码后即时生效，无需重启编辑器）<br>
- 自举编译器（完全用 Open-Script 编写的编译器，输出机器码/字节码）<br>
- 语言进化（通过编辑器自身的修改，迭代语言语法和特性）<br>
- 无 Rust 依赖（运行时也由 Gen 3 自身编译）<br>

```
Gen 3 的使命：实现真正的自举。
Open-Script 编辑器可以修改 Open-Script 编译器的源码，编译出一个新的编辑器，
然后用新编辑器替换掉正在运行的旧编辑器——语言通过自身来进化。
```

### 自举关键路径
| 阶段 | 里程碑 | 依赖 |
|------|--------|------|
| **Phase 0** | Rust 端实现完整的标签解析器和解释器 | Rust |
| **Phase 1** | Gen 1 Rust 编辑器可用：编辑+运行 .os 文件 | Phase 0 |
| **Phase 2** | 用 Open-Script 编写 Gen 2 编辑器，Gen 1 编译 | Phase 1 |
| **Phase 3** | Gen 2 能编译自身（通过 Gen 1 运行时） | Phase 2 |
| **Phase 4** | Gen 3 完全自举：编译器用 .os 编写，运行时不依赖 Rust | Phase 3 |
| **Phase 5** | 持续迭代：语言通过编辑器自身进化 | 永远 |

### 代码示例：自举中的"自我编译"
、、、
// 这是一个 Gen 3 时代的概念代码 —— 编译器编译自身

<func name="compile-self" return="Result">
    <comment>读取自身的源码文件</comment>
    <file-glob pattern="source/**/*.os" into="own-sources"></file-glob>

    <comment>逐个文件解析并编译</comment>
    <for-each item="file" collection="own-sources">
        <file-read path="file" into="source-code"></file-read>
        <tokenize source="source-code" into="tokens"></tokenize>
        <parse tokens="tokens" grammar="open-script-grammar" into="ast"></parse>
        <push list="all-asts" value="ast"></push>
    </for-each>

    <comment>编译所有 AST 为可执行文件</comment>
    <compile asts="all-asts" output="osedit-new" target="native"></compile>

    <comment>用新编译的编辑器替换当前运行的编辑器</comment>
    <exec command="osedit-new" args="--replace-self">
        <on-success>
            <out>Self-compilation successful! Editor updated.</out>
        </on-success>
    </exec>

    <return><result>Ok</result></return>
</func>
、、、

### 不为自举而自举
自举不是为了炫技，而是有实际意义: <br>
Bootstrapping is not for show, it has practical significance: <br>

1. **语言验证**: 如果 Open-Script 能写出自己的编辑器，说明语言已经完备且实用 <br>
2. **迭代进化**: 在编辑器中修改语言语法 → 编译新编译器 → 新语法立即可用 <br>
3. **社区贡献**: 任何人都可以用 Open-Script 本身为语言贡献新特性 <br>
4. **减少依赖**: 最终不再依赖 Rust 工具链，Open-Script 自成体系 <br>
5. **教育意义**: 展示一门语言从"借助宿主语言诞生"到"完全独立"的完整过程 <br>
