// 引入需要使用的模块信息
use base_grammer::{
    branch, closures, command, function, generic, handlerr, intellpoint, iofs, iterator, loops,
    ownership, thread, variable,
};

fn main() {
    // intellpoint::boxtype::box_use();
    // command::syscall::call()
    thread::thread::run();
}

fn premain() {
    // 测试数据类型的使用
    variable::vcommon::variable();
    variable::vstring::string_variable();
    variable::vnumber::number_variable();
    variable::vbool::bool_variable();
    variable::vchar::char_variable();
    variable::vtuple::tuple_type();
    variable::vslice::slice_reference();
    variable::vstruct::struct_variable();
    variable::venum::test_enum();
    variable::vvec::vec_type();
    variable::vhashmap::hashmap_type();

    // 测试所有权
    ownership::ownership::test_ownship();

    // 测试方法结构
    function::func_use::func_detail();

    // 测试程序分支结构
    loops::loop_use::loop_grammer();

    // 测试程序循环结构
    branch::branch_use::branch_grammer();

    // 测试 io 流
    iofs::stdio::io_test();

    // 测试错误处理
    // handlerr::panic_macro::handle_error();
    handlerr::result::use_result();

    // 测试泛型与声明周期
    generic::generic_type::generic_use();
    generic::trait_type::trait_use();
    generic::lifetimes::use_lifetimes();

    // 测试闭包
    closures::closure::test_closure();

    // 测试迭代器
    iterator::iter::iter_test();
    iterator::iter::iter_test();

    // 智能指针box 的使用
}
