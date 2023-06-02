fn main() {
    immutable();
    constant();
    shadowing();
}


fn immutable() {
    // mutable한 변수 생성
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    // mutable한 변수이기에 값 변경 가능
    x = 6;
    println!("The value of x is: {}", x);
}

fn constant() {
    // 상수 선언 및 값 할당
    // const 키워드를 사용하여 타입을 명시하여 선언하였으며, 상수 표현식을 사용하여 값 할당
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn shadowing() {
    // 변수 x 초기 선언 및 할당
    let x = 5;

    // 변수 x 재선언 및 재할당
    let x = x + 1;

    // 변수 x 재선언 및 재할당
    let x = x * 2;
    
    println!("The value of x is: {}", x);
}