use std::{env, fs, net::Ipv4Addr, process::Command, str::FromStr};


pub fn call() {
    // parse_linux_ip_with_address();
    //
    find_command_path("bat");
}

// 使用 ip address 进行 ip 解析
fn parse_linux_ip_with_address() {
    let output: Vec<u8> = Command::new("ip")
        .arg("address")
        .output()
        .expect(&format!("exec : {}, command error!!", "ip address"))
        .stdout;
    let text = String::from_utf8(output).expect("decode utf-8 failed");
    println!("{}", text);
    println!("{}", "---------------------");

    let ipv4 = get_linux_ip(&text);
    println!("{:?}", ipv4);
}

// 根据命令来获取其中的结果
fn get_linux_ip(output: &str) -> Ipv4Addr {
    output
        .lines()
        .map(|l| l.trim_start())
        .filter_map(|l| l.strip_prefix("inet "))
        .filter_map(|l| l.find(" ").map(|x| &l[0..x]))
        .filter_map(|l| l.find("/").map(|x| &l[0..x]))
        .filter_map(|l| Ipv4Addr::from_str(l).ok())
        .find(|o| !o.is_loopback())
        .unwrap()
}

// 在 path 中查找某个命令的位置
fn find_command_path(cmd: &str) {
    println!("开始查找命令: {}", cmd);
    let path = env::var("PATH").unwrap();
    println!("{}", path);
    println!("{}", "---------------------------");

    path.split(":")
        .filter(|l| fs::read_dir(l).is_ok())
        .map(|e| fs::read_dir(e).unwrap())
        .for_each(|item| {
            item.for_each(|o| {
                let filepath = o.unwrap().path();
                let pattern = format!("/{}$", cmd);
                let ok = filepath.to_str().unwrap().find(&pattern).is_some();
                if ok {
                    println!("找到了 {}, {}", cmd, filepath.display());
                }
            })
        });
    //.filter_map(|l|  )
}
