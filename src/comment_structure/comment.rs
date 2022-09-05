// test comment catagory
pub fn comment_category() {
    // single line comment
    single_line_comment();

    // block comment
    block_comment();

    // multi line comment
    let s = "hello";
    multi_line_comment(s);
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
