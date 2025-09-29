pub mod pipe;
pub mod shmem;
pub mod unix;
pub mod ipc_epoll;

pub fn run() {
    // pipe::usage_pipe();
    // shmem::usage_shmem();
    // unix::usage_unix();
    ipc_epoll::ipc_server().expect("ipc server start error");
}
