

//! Rayon  : 用于 Rust 的数据并行库
//!   - 它极轻量，可以轻松的将顺序计算转换为并行计算
//!   - 同时, 它还能保证数据竞争(data race)的安全性
//!   - 
//! > 底层原理:
//!   - rayon 会在底层创建一个与机器核心数相同的线程池
//!   - 还会创建一个支持 work stealing 的任务队列，防止线程处于空闲状态
//!   - 同时还支持了 subtask (子任务) 处理一个任务等待另外任务完成的场景
//!   - 

use std::time::Instant;

use rayon::prelude::*;

// 1. 并行的 api
pub fn run() {
    // sum_val();
    // find_primes();
    // custom_pool();
    // nest_scope();
    // broadcast();
    merge();
}


fn merge() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let fun = || println!("hello");
    // join 只能接收两个参数，等待结束
    pool.join(fun, fun);
}


fn broadcast() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    pool.scope(|s| {
        s.spawn_broadcast(move| _scope, ctx| {
            println!("Thread {} started", ctx.index());

        });

    })
}


fn nest_scope() {
    let outer_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    outer_pool.scope(|scope| { 
        for i in 0..2 { 
            scope.spawn(move |_| {
                println!("outer scope {} started", i);
                let inner_pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(2)
                    .build()
                    .unwrap();

                inner_pool.scope(|scope| {
                    for j in 0..2 {
                        scope.spawn(move |_| {
                            println!("\t-> inner scope {} finished", j);
                        });
                    }
                });
                println!("outer scope {} finished", i);
            });
        }
    });
}


// 自定义线程池
//
fn custom_pool() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)

        // 设置全局的线程池
        // .build_global()
        .build()
        .unwrap();


    let matrix = [
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];

    pool.scope(|scope| {
        for (i, row) in matrix.iter().enumerate() {
            scope.spawn(move |_|{
                let sum = row.iter().sum::<i32>();
                println!("sum of row {} is {}", i, sum);
            });
        }
    });
    println!("Main thread done");
}


// sum 0..1000000
fn sum_val() {
    let nums = (0..1_000_000).collect::<Vec<u64>>();

    // par_iter 用于不可变的并行迭代
    // par_iter_mut 用于可变的并行迭代
    // into_par_iter 用于获取所有权
    let sum = nums.par_iter().sum::<u64>();
    println!("sum = {}", sum);
}

fn find_primes() {
    let start = Instant::now();
    let nums = (0..1_000_000).collect::<Vec<u64>>();
    let mut primes = nums.par_iter().filter(|&n| is_prime(*n)).collect::<Vec<&u64>>();

    let duration = start.elapsed();

    primes.par_sort_unstable();
    println!("{primes:?}");
    println!("find {} primes in {}ms", primes.len(), duration.as_millis());
}

// 判断质数
fn is_prime(n: u64) -> bool {
    (2..=n/2).all(|i| n % i != 0)
}
