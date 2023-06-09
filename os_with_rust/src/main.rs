// cargo add libc
use libc::fork;

fn main() {
    unsafe {
        let pid = fork();
        println!("pid: {}", pid);
    }
}

fn fork_example() {
    // fork() 함수 호출하기 위해 unsafe 블록으로 감싸준다.
    unsafe {
        // fork() 함수를 호출하면 child process가 생성되고,
        // fork()의 return값은 parents process의 경우 child process의 pid 값을,
        // child process의 경우 0이다.
        let pid = fork();

        // 먼저 parents process가 실행되고, parents process가 exit되어 cpu로부터 release된 이후에는
        // ready queue에 있는 child process가 실행된다.
        // 결과적으로 child process에서의 fork()의 return값인 child process의 pid값이 출력되고, 이후 0이 출력된다. (총 2번 출력됨)
        println!("pid: {}", pid);
    }
}
