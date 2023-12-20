// 引入需要使用的模块信息
use base_grammer::{
    branch, closures, command, function, generic, handlerr, intellpoint, iofs, iterator, loops,
    ownership, thread, variable,
};

fn main() {
    // intellpoint::boxtype::run();
    intellpoint::rctype::run();
    // command::syscall::call();
    // thread::thread::run();
    // thread::channel::run();
    // thread::mutex::run();
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
    ownership::ownership::run();

    // 测试方法结构
    function::func_use::run();

    // 测试程序分支结构
    loops::loop_use::run();

    // 测试程序循环结构
    branch::branch_use::run();

    // 测试 io 流
    iofs::stdio::run();

    // 测试错误处理
    // handlerr::panic_macro::run();
    handlerr::result::run();

    // 测试泛型与声明周期
    generic::gentype::run();
    generic::gentrait::run();
    generic::lifetimes::run();

    // 测试闭包
    closures::closure::run();

    // 测试迭代器
    iterator::iter::run();

    // 智能指针box 的使用
}
