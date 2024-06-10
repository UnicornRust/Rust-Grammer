use std::fs::{self, DirBuilder, DirEntry, File, OpenOptions, ReadDir};
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};

pub fn run() {
    println!("--------------------fs------------------");
    // 文件操作
    dir_operation();
    // 获取文件夹中的内容
    read_dir();
    // 获取元数据信息
    metadata();
    // 文件操作
    file_create();
    // 写入文件
    write();
    // 读取文件
    read();
}

// 文件目录操作
fn dir_operation() {
    //
    // DirBuilder 创建文件夹
    let path: &str = "temp/foo/bar";
    DirBuilder::new()
        .recursive(true) // 进行递归创建，多级文件夹可以同时创建，false 时只能创建最后一层文件夹
        .create(path)
        .unwrap();

    // 2. 使用其他创建目录的API
    // 创建多级文件夹
    fs::create_dir_all("temp/some/all").unwrap();
    // 创建单个文件夹(为了防止出错，我们先判断文件夹是否存在)
    match fs::metadata("temp/some/dir") {
        Ok(_) => println!("Folder already exists"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => fs::create_dir("temp/some/dir").unwrap(),
            _ => println!("获取文件信息失败"),
        },
    }
}

// 文件夹中的文件列表
fn read_dir() {
    let read_dir: ReadDir = fs::read_dir(".").unwrap();
    for entry in read_dir {
        let entry: DirEntry = entry.unwrap();
        // DirEntry 结构体代表文件夹里的一个子文件或子文件夹
        println!("{:#?}", entry.path());
    }
}

// 获取文件/文件夹的元数据信息
fn metadata() {
    let metadata = fs::metadata("temp/foo/bar").unwrap();
    dbg!(metadata.is_dir());
    dbg!(metadata.is_file());
    dbg!(metadata.len());
    dbg!(metadata.permissions());
    dbg!(metadata.created().unwrap());
    dbg!(metadata.modified().unwrap());
    dbg!(metadata.accessed().unwrap());
}

// 创建文件
fn file_create() {
    // 1.创建文件,使用文件选项(所有选项初始化为 false), 需要自己确定需要打开什么模式
    //   > read
    //   > write
    //   > append
    //   > truncate
    //   > create
    //   > create_new
    //   > custom_flags
    //   > mode
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp/foo.txt");

    // 2. 使用 create, 实际上是对 OpenOption 的封装
    //   > write(true)    -- 文件可写
    //   > create(true)   -- 文件不存在会自动创建
    //   > truncate(true) -- 文件打开后里面的内容会被清空

    let path = "temp/lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file:: {:?}", error);
        }
    };
    dbg!(output);

    // 打开一个文件
    let output2 = File::create("temp/rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("temp/rand.txt") {
                Ok(fc) => fc,
                Err(_e) => panic!("Can't create file: {:?}", error),
            },
            _ther_error => panic!("Problem opening file: {:?}", error),
        },
    };
    dbg!(output2);
}

// 写入文件
fn write() {
    let path = "temp/lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file:: {:?}", error);
        }
    };
    write!(output, "just some\n Random words").expect("Failed to write to file");
}

// 读取文件
fn read() {
    let path = "temp/lines.txt";
    //  open 仅仅 read 模式被打开
    let mut input = File::open(path).unwrap();

    // 1.读取文件 (每次读取一个字节都会进行系统调用，效率低下，耗费资源)
    let mut buffer = [0; 1]; // 每次读取一个字节
    while input.read(&mut buffer).unwrap() != 0 {
        // 处理字节
    }

    // 2. 借助缓冲区，会先缓冲一批数据到缓冲区，直到缓冲区数据不满足条件，才会进行下一次的系统调用
    //   使用缓冲区可以减少系统调用
    let mut file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1]; // 每次读取一个字节的需求不变
    while reader.read(&mut buffer).unwrap() != 0 {
        // 处理字节
    }

    // 3. 按行读取
    let mut file2 = File::open(path).unwrap();
    let buffered = BufReader::new(file2);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}
