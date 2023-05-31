fn main() {
    for_loop();
    for_loop_ref();
    for_loop_ref_mut();
    for_anonymous();
    index_loop();
    continue_loop();
    basic_while();
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