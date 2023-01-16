//!
//! ## 这是模块 或者 crate 注释，
//!
//! - 表达的含义是包含注释的项，而不是位于注释之后的项添加文档
//! - 通常用于 crate 根文件(src/lib.rs) 或模块的根文件
//!
//! - 可以使用 markdown 语法编写,

/*
*  ##  多行的模块注释写法
*
*  - 可以使用markdown 语法
*/

// test comment catagory
pub fn comment_category() {
    // single line comment
    single_line_comment();

    // block comment
    block_comment();

    // multi line comment
    let s = "hello";
    multi_line_comment(s);

    // 文档块注释
    doc_block_comment(s);
}

fn single_line_comment() {

    // single line comment used by code instruction
    // more comment on code
}

fn block_comment() {
    /*
     * block comment
     * ---------------------------------------
     * block comment used by code instruction complexable
     */
}

/// ## This is a H2 title
/// multi line comment used by document code instruction
/// which support markdown language
fn multi_line_comment(option: &str) {
    println!("{}", option);
}

/**
* 文档块注释, 可以使用 markdown 语法
*/
fn doc_block_comment(option: &str) {
    println!("{}", option)
}
