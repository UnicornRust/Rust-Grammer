use std::marker::PhantomData;
use std::ops::Deref;

//
// PhantomData<T> 类似于生命周期的标注，主要记录一些类型之间的关联信息
//
struct WorkType<T> {
    data: *const T,
    _marker: PhantomData<T>,
}

impl <T>  WorkType<T>  {
    fn new(t: T) -> WorkType<T> {
        WorkType {
            data: &t,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for WorkType<T> {
    type Target: T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data}
    }
}


impl <T> Drop for WorkType<T> {
    fn drop(&mut self) {
        println!("dropping WorkType instance!")
    }
}

fn test_phantom_data() {
    let resource = WorkType::new(true);
    let another_resource = WorkType::new(32);

    // 发现打印出来的结果并不正确，这里其实已经产生了悬垂指针
    //
    println!("{:?}", unsafe { *(resource.data)});
    println!("{:?}", unsafe { *(another_resource.data)});
    // 执行到这里的时候 instance 将离开作用域被销毁，调用我们自定义的 Drop trait
}
