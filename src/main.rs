// 引入需要使用的模块信息
use rs_grammer::{
    advance, blog, branch, closures, command, eventloop, function, generic, handlerr, intellpoint, iofs, ipc, iterator, loops, ownership, pattern, thread, variable
};

fn main() {

    thread::run();
}

fn premain() {

    // 测试 ipc 通信
    ipc::run();

    // 测试数据类型的使用
    variable::run();

    // 测试所有权
    ownership::run();

    // 测试方法结构
    function::run();

    // 测试程序分支结构
    loops::run();

    // 测试程序循环结构
    branch::run();
    
    // 测试 io 流
    iofs::run();

    // 测试错误处理
    handlerr::run();

    // 测试泛型与声明周期
    generic::run();

    // 测试闭包
    closures::run();

    // 测试迭代器
    iterator::run();

    // 智能指针box 的使用
    intellpoint::run();

    // 线程使用

    // rust 面向对象,设计模式
    blog::run();

    // rust 模式匹配
    pattern::run();

    // rust 高级部分
    advance::run();

    // 事件循环
    eventloop::run();

    // 测试调用系统命令
    command::run();
}
