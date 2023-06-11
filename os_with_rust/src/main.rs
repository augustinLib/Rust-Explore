use libc::{fork, wait, execlp, shm_open, ftruncate, mmap, sprintf, shm_unlink, pipe, close, read, write,
     c_char, c_int, pid_t,
     O_CREAT, O_RDWR, PROT_READ, PROT_WRITE, MAP_SHARED, O_RDONLY, c_void
    };

fn main() {
    // fork_example();
    // fork_with_wait();
    // multiple_fork();
    // multiple_fork2();
    // fork_execlp();
    // producer();
    // consumer();
    pipe_ex();
}

fn fork_example() {
    // fork() 함수 호출하기 위해 unsafe 블록으로 감싸준다.
    unsafe {
        // fork() 함수를 호출하면 child process가 생성되고,
        // fork()의 return값은 parents process의 경우 child process의 pid 값을,
        // child process의 경우 0이다.
        let pid: pid_t = fork();

        // 먼저 parents process가 실행되고, parents process가 exit되어 cpu로부터 release된 이후에는
        // ready queue에 있는 child process가 실행된다.
        // 결과적으로 child process에서의 fork()의 return값인 child process의 pid값이 출력되고, 이후 0이 출력된다. (총 2번 출력됨)
        println!("pid: {}", pid);
    }
}

fn fork_with_wait() {
    // fork() 함수 호출하기 위해 unsafe 블록으로 감싸준다.
    unsafe{
        let pid: pid_t = fork();
        if pid > 0 { // parents process
            // parents process는 child process가 종료되기를 기다림
            // wait() system call을 통해 parents process를 ready queue에서 wait queue로 이동시킨다.
            // wait queue에서 child process가 종료되어 interrupt를 걸어주길 기다림
            wait(0 as *mut c_int);
        }
        // child process 먼저 실행되고, child process가 exit되어 cpu로부터 release된 이후에는
        // ready queue에 있는 parents process가 실행된다.
        println!("pid: {}", pid);
    }
}

fn multiple_fork() {
    unsafe {
        for _ in 0..4 {
            fork();
        }
        println!("fork!")
    }

}

fn multiple_fork2() {
    unsafe {
        let mut value = 5;
        fork();
        value += 5;
        fork();
        value += 5;
        fork();
        value += 5;
        fork();
        value += 5;
        println!("value: {}", value);

    }
}

fn fork_execlp() {
    unsafe {
        let pid: pid_t;
        // fork를 통해 parent process의 address space를 복사하여 child process를 생성한다.
        pid = fork();

        if pid == 0 {
            // execlp() system call을 통해 child process의 address space를 ls의 address space로 덮어씌운다.
            execlp("/bin/ls\0".as_ptr() as *const c_char, "ls\0".as_ptr() as *const c_char, 0 as *mut c_char);
            // fork()를 통해 복사된 child process의 address space에 ls의 address space를 덮어씌웠기 때문에
            // 아래의 println!()은 실행되지 않는다.
            println!("Line J");
        }
        else if pid > 0 {
            // parents process는 child process가 종료되기를 기다림
            // wait() system call을 통해 parents process를 ready queue에서 wait queue로 이동시킨다.
            // wait queue에서 child process가 종료되어 interrupt를 걸어주길 기다림
            wait(0 as *mut c_int);
            println!("child complete");
        }
    }
}

fn producer() {
    unsafe {
    const SIZE: i64 = 4096;
    const NAME: &str = "OS";
    const MESSAGE_0: &str = "Hello";
    const MESSAGE_1: &str = "Shared Memory";

    let shm_fd:i32;
    let mut ptr:*mut i8;

        shm_fd = shm_open(NAME.as_ptr() as *const c_char, O_CREAT | O_RDWR, 0666);

        ftruncate(shm_fd, SIZE);
        ptr = mmap(std::ptr::null_mut(), SIZE as usize, PROT_READ | PROT_WRITE, MAP_SHARED, shm_fd, 0) as *mut i8;
        
        sprintf(ptr, MESSAGE_0.as_ptr() as *const c_char);
        ptr = ptr.offset(MESSAGE_0.len() as isize);
        sprintf(ptr, MESSAGE_1.as_ptr() as *const c_char);
        ptr = ptr.offset(MESSAGE_1.len() as isize);
    }
}

fn consumer() {
    unsafe {
    const SIZE: i64 = 4096;
    const NAME: &str = "OS";
    let shm_fd:i32;
    let ptr:*mut i8;
    
        shm_fd = shm_open(NAME.as_ptr() as *const c_char, O_RDONLY, 0666);
        ptr = mmap(std::ptr::null_mut(), SIZE as usize, PROT_READ, MAP_SHARED, shm_fd, 0) as *mut i8;
        println!("{}", *ptr);
        shm_unlink(NAME.as_ptr() as *const c_char);
    }
}

fn pipe_ex() {
    unsafe {
    const BUFFER_SIZE: usize = 25;
    const READ_END: usize = 0;
    const WRITE_END: usize = 1;

        let mut write_msg = String::from("Greetings");
        let mut read_msg: String = String::with_capacity(BUFFER_SIZE);

        let mut fd: [i32; 2] = [0; 2];
        let pid: pid_t;

        pipe(fd.as_mut_ptr() as *mut i32);

        pid = fork();

        if pid > 0 {
            close(fd[READ_END]);
            write(fd[WRITE_END], write_msg.as_mut_ptr() as *mut c_void, (write_msg.len()+1) as usize);
            close(fd[WRITE_END]);
        }
        else if pid == 0 {
            close(fd[WRITE_END]);
            read(fd[READ_END], read_msg.as_mut_ptr() as *mut c_void, BUFFER_SIZE);
            println!("read: {}", read_msg);
            close(fd[READ_END]);
        }
    }
}