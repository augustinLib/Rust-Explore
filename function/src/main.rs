fn main() {
    expression();
    let x = return_function();
    println!("x: {}", x);
    let y = plus_one(5);
    println!("y: {}", y);
}

fn expression() {
    let x = 5;

    // {}는 범위를 표현하는 코드 블록이며, 이 역시 expression(표현식)이다.
    let y = {
        let x = 3;

        // expression(표현식)은 마지막에 ;를 붙이지 않는다.
        // ;를 붙이면 statement(구문)이 되어 값을 반환하지 않는다
        x + 1
    };

    println!("y: {}", y);
}

fn return_function() -> i32 {
    // expression(표현식)은 마지막에 ;를 붙이지 않는다.
    // ;를 붙이면 statement(구문)이 되어 값을 반환하지 않는다
    10
}

fn plus_one(x: i32) -> i32 {
    // expression(표현식)은 마지막에 ;를 붙이지 않는다.
    // ;를 붙이면 statement(구문)이 되어 값을 반환하지 않는다
    x + 1
}