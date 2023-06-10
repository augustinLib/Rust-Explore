use libc::{fork, wait, execlp, c_char, c_int, pid_t};

fn main() {
    // fork_example();
    // fork_with_wait();
    // multiple_fork();
    // multiple_fork2();
    fork_execlp();
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