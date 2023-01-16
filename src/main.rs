// 引入需要使用的模块信息
use base_grammer::{
    branch_structure, closures, command, function_structure, generic_structure, handle_error,
    io_structure, iterator, loop_structure, ownership_structure, variable_structure,
};

fn main() {
    command::syscall::call()
}

fn premain() {
    // 测试数据类型的使用
    variable_structure::vcommon::variable();
    variable_structure::vstring::string_variable();
    variable_structure::vnumber::number_variable();
    variable_structure::vbool::bool_variable();
    variable_structure::vchar::char_variable();
    variable_structure::vtuple::tuple_type();
    variable_structure::vslice::slice_reference();
    variable_structure::vstruct::struct_variable();
    variable_structure::venum::test_enum();
    variable_structure::vvec::vec_type();
    variable_structure::vhashmap::hashmap_type();

    // 测试所有权
    ownership_structure::ownership::test_ownship();

    // 测试方法结构
    function_structure::func_use::func_detail();

    // 测试程序分支结构
    loop_structure::loop_use::loop_grammer();

    // 测试程序循环结构
    branch_structure::branch_use::branch_grammer();

    // 测试 io 流
    io_structure::stdio::io_test();

    // 测试错误处理
    // handle_error::panic_macro::handle_error();
    handle_error::result::use_result();

    // 测试泛型与声明周期
    generic_structure::generic_type::generic_use();
    generic_structure::trait_type::trait_use();
    generic_structure::lifetimes::use_lifetimes();

    // 测试闭包
    closures::closure::test_closure();

    // 测试迭代器
    iterator::iter::iter_test();
    iterator::iter::iter_test();
}
