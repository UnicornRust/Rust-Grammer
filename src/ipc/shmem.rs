use std::{ptr, thread, time::Duration};

use libc::{shmat, shmctl, shmget, IPC_CREAT, IPC_RMID};

const SHM_SIZE: usize = 1024;

/* 
pub fn usage_shmem() {
    let key = 1234;

    let shmid = unsafe { shmget(key, SHM_SIZE, 0o666 | IPC_CREAT) };
    if shmid == -1 {
        panic!("shmget failed");
    }

    let shm = unsafe {
        shmat(shmid, ptr::null(), 0) 
    } as *mut u8;

    if shm == ptr::null_mut() {
        panic!("shmat failed");
    }

    let handle = thread::spawn( move || {
        thread::sleep(Duration::from_secs(1));
        unsafe {
            let data = b"Hello from writer";
            ptr::copy_nonoverlapping(data.as_ptr(), shm, data.len());
        }
    });

    thread::sleep(Duration::from_secs(2));

    unsafe {
        println!("Reader received: {}", String::from_utf8_lossy(
            std::slice::from_raw_parts(shm, 17)
        ));
        shmctl(shmid, IPC_RMID, ptr::null_mut());
    }
    handle.join().unwrap();
}

*/
