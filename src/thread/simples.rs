//
// 使用多种方式来完成 0 .. 1000000000 累加的计算
//

use std::{any, sync::{mpsc, Arc, Mutex}, thread, time::Instant};

const TOTAL: u64 = 1_000_000_000;

pub fn run_examples() {
    cal_time(serialize_calculate);
    cal_time(parallel_calculate);
    cal_time(scope_calculate);
    cal_time(channel_calculate);
}

// use channel to calculate 
fn channel_calculate() -> u64 {
    let chunk_size = TOTAL / 32;
    let (tx, rx) = mpsc::channel();

    for i in 0.. 32 {
        let start = i * chunk_size;
        let end = if i == 31 {
            TOTAL
        }else {
            (i + 1) * chunk_size
        };
        let tx = tx.clone();
        thread::spawn(move || {
            let mut local_sum = 0;
            for i in start..end {
                local_sum += i;
            }
            tx.send(local_sum).unwrap();
        });
    }

    // 因为生产者在每次分发的都复制了一份，这些被复制的部分在离开作用域之后会被回收,
    // 但是原始存在的一份没有被回收(走出作用域 -> 即需要等到程序结束才会回收)
    // 如果不主动回收这一份，接收者的会一直阻塞在等待消息
    drop(tx);

    let mut sum = 0;
    while let Ok(local_sum) = rx.recv() {
        sum += local_sum;
    }
    sum
}


// use scope thread to calculate
fn scope_calculate() -> u64 {
    let chunk_size = TOTAL / 32;
    let sum = Mutex::new(0);

    thread::scope(|scope|{
        for i in 0..32 {
            let start = i * chunk_size;
            let end = if i == 31 {
                TOTAL
            }else {
                (i + 1) * chunk_size
            };

            let sum_ref = &sum;

            // 使用 scope 来实现多线程, hold variable reference
            scope.spawn(move ||{
                let mut local_sum = 0;
                for i in start..end {
                    local_sum += i;
                }
                let mut sum = sum_ref.lock().unwrap();
                *sum += local_sum;
            });
        }
    });
    let sum = sum.lock().unwrap();
    *sum
}


// use parallel with mutex to calculate
fn parallel_calculate() -> u64 {
    let chunk_size = TOTAL / 32;
    let mut handles = Vec::new();
    let sum = Arc::new(Mutex::new(0));

    for i in 0..32 {
        let start = i * chunk_size;
        let end = if i == 31  {
            TOTAL
        }else {
            (i + 1) * chunk_size
        };
        let _sum = Arc::clone(&sum);
        let handle = thread::spawn(move|| {
            let mut local_sum = 0;
            for i in start .. end {
                local_sum += i;
            }
            let mut sum = _sum.lock().unwrap();
            *sum += local_sum;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let x = sum.lock().unwrap();
    *x
}


// use single thread to calculate
fn serialize_calculate() -> u64 { 
    let mut sum = 0;

    for i in 0..TOTAL {
        sum += i;
    }
    sum
}

// 计算耗时的函数
// 使用 FnOnce 来限定我们计算时间的函数，只要满足最低要求 FnOnce 即可
fn cal_time<F: FnOnce() -> u64>(f: F) { 

    // 类似于反射获取到名称(函数名)
    let name = any::type_name::<F>();

    let start = Instant::now();
    let sum = f();
    let dur = start.elapsed();
    println!("[{}] -> sum {} cost {} ms", name, sum, dur.as_millis());
}
