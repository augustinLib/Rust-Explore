fn main() {
    for_loop();
    for_loop_ref();
    for_loop_ref_mut();
    for_anonymous();
    index_loop();
    continue_loop();
    basic_while();
    if_condition();
    else_if();
    if_expression_with_let();
    loop_return();
}


fn for_loop() {
    let container: [i32; 5] = [1, 2, 3, 4, 5];
    for item in container {
        println!("{}", item)
        // do something
    }
}

fn for_loop_ref() {
    let container: [i32; 5] = [1, 2, 3, 4, 5];
    // &를 사용하여 참조를 생성
    for item in &container {
        // do something
        println!("{}", item)
    }
}

fn for_loop_ref_mut() {
    let mut container: [i32; 5] = [1, 2, 3, 4, 5];
    for item in &mut container {
        // do something
        println!("{}", item)
    }
}

fn for_anonymous() {
    // 0..10 : 배제 범위(exclusive range), 값을 포함하지 않는 범위
    // 0..=10 : 포함 범위(inclusive range), 값을 포함하는 범위
    for _ in 0..10 {
        // do something
    }
}

fn index_loop() {
    let collection = [1, 2, 3, 4, 5];
    // array.len() : 배열의 길이
    // 0..collection.len() : 배제 범위(exclusive range), 값을 포함하지 않는 범위
    for i in 0..collection.len() {
        println!("{}", collection[i]);
    }
}

fn continue_loop() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}

fn basic_while() {
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
}

fn if_condition() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn else_if() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_expression_with_let() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}

// fn loop_basic() {
//     loop {
//         println!("again!");
//     }
// }

fn loop_return() {
    let mut counter = 0;

    // loop문에서 반환된 값을 result에 할당
    let result = loop {
        counter += 1;
        if counter == 10 {
            // break 키워드를 사용하여 반복문을 종료하고 값을 반환
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}