pub mod pipe;
pub mod shmem;
pub mod unix;

pub fn run() {
    pipe::usage_pipe();
    shmem::usage_shmem();
    unix::usage_unix();
}
