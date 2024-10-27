
use rs_grammer::variable::vstruct;

mod common;

#[test] 
fn test_vstruct() {
    common::setup();
    vstruct::struct_variable();
}
