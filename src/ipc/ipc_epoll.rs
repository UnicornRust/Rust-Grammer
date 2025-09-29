use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::poll::PollTimeout;
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags};
use nix::sys::socket::{
    accept, bind, listen, socket, AddressFamily, Backlog, SockFlag, SockType, UnixAddr,
};
use nix::unistd::{close, read, write};
use std::mem;
use std::os::fd::{AsRawFd, BorrowedFd, RawFd};
use std::{error, fs, path::Path};

const SOCKET_PATH: &str = "/tmp/ipc_example_sock";
const BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone, Copy)]
enum FdType {
    Listener,
    Connector,
}

#[derive(Debug, Clone, Copy)]
struct EventData {
    fd: RawFd,
    fd_type: FdType,
}
// 编译期断言确保类型转换安全
const _: () = assert!(mem::size_of::<EventData>() == mem::size_of::<u64>());

impl From<EventData> for u64 {
    fn from(data: EventData) -> Self {
        unsafe { mem::transmute(data) }
    }
}

impl From<u64> for EventData {
    fn from(data: u64) -> Self {
        unsafe { mem::transmute(data) }
    }
}

// ipc server
pub fn ipc_server() -> Result<(), Box<dyn error::Error>> {
    // 服务端程序启动之前可以先删除可能残留的 socket 临时文件
    let socket_path = Path::new(SOCKET_PATH);
    if socket_path.exists() {
        let _ = fs::remove_file(socket_path)?;
    }

    // 申明 socket
    let sockfd = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::SOCK_NONBLOCK, // 非阻塞
        None,
    )?;

    println!("create soket file: {:?}", sockfd);

    // 绑定 socket
    let addr = UnixAddr::new(socket_path)?;
    bind(sockfd.as_raw_fd(), &addr)?;

    // 开始监听
    //
    listen(&sockfd, Backlog::new(10)?)?; // 最大队列长度为 10
    println!("start listen");

    let listen_data = EventData {
        fd: sockfd.as_raw_fd(),
        fd_type: FdType::Listener,
    };

    // event 定义, Event data 会转换为 u64,(通过 mem::transmute 将 EventData 转换为 u64 存储)
    //
    let event = EpollEvent::new(EpollFlags::EPOLLIN, u64::from(listen_data));

    let epoll_fd = Epoll::new(EpollCreateFlags::empty())?;

    epoll_fd.add(&sockfd, event)?;

    // 事件组
    let mut events = [EpollEvent::empty(); 10];
    loop {
        println!("epoll wait");
        let nfds = epoll_fd.wait(&mut events, PollTimeout::NONE)?;
        for i in 0..nfds {
            let event = &events[i];
            let data = EventData::from(event.data());
            match data.fd_type {
                FdType::Listener => {
                    // 处理连接时间
                    if let Ok(conn_fd) = accept(data.fd) {
                        println!("new connection: {}", conn_fd);
                        let conn_as_fd = unsafe { BorrowedFd::borrow_raw(conn_fd) };
                        fcntl(conn_as_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;
                        let conne_data = EventData {
                            fd: conn_fd,
                            fd_type: FdType::Connector,
                        };
                        let conn_event = EpollEvent::new(
                            EpollFlags::EPOLLIN
                                | EpollFlags::EPOLLET
                                | EpollFlags::EPOLLHUP
                                | EpollFlags::EPOLLERR,
                            u64::from(conne_data),
                        );
                        epoll_fd.add(conn_as_fd, conn_event)?;
                    }
                }
                FdType::Connector => {
                    let conn_fd = unsafe { BorrowedFd::borrow_raw(data.fd) };
                    let flags = event.events();
                    if flags.contains(EpollFlags::EPOLLHUP) || flags.contains(EpollFlags::EPOLLERR)
                    {
                        println!("connection closed: {:?}", conn_fd);
                        epoll_fd.delete(conn_fd)?;
                        close(data.fd)?;
                        continue;
                    }

                    if flags.contains(EpollFlags::EPOLLET) || flags.contains(EpollFlags::EPOLLIN) {
                        let mut buffer = [0; BUFFER_SIZE];
                        match read(conn_fd, &mut buffer) {
                            Ok(0) => {
                                println!("connection closed: {:?}", conn_fd);
                                epoll_fd.delete(conn_fd)?;
                                close(data.fd)?;
                            }
                            Ok(n) => {
                                let msg = String::from_utf8_lossy(&buffer[..n]);
                                println!("received message: {}", msg);
                                write(conn_fd, &buffer[..n])?;
                            }
                            Err(e) => {
                                println!("read error: {}", e);
                                epoll_fd.delete(conn_fd)?;
                                close(data.fd)?;
                            }
                        }
                    }
                }
            }
        }
    }
}
