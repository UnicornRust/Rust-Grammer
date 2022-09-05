// 引入需要使用的模块信息
use base_grammer::{
    branch_structure, function_structure, loop_structure, ownership_structure, variable_structure,
};

fn main() {
    loop_structure::loop_use::loop_grammer();
    branch_structure::branch_use::branch_grammer();
    variable_structure::vcommon::variable();
    variable_structure::vstring::string_variable();
    variable_structure::vnumber::number_variable();
    variable_structure::vbool::bool_variable();
    variable_structure::vchar::char_variable();
    variable_structure::vtuple::tuple_type();
    variable_structure::vslice::slice_reference();
    variable_structure::vstruct::struct_variable();
    ownership_structure::ownership::test_ownship();
    function_structure::func_use::func_detail();
}
