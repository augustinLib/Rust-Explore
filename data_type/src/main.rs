// use키워드를 통해 std::convert::TryInto 트레이트를 지역 범위로 가져옴
use std::convert::TryInto;
use num::complex::Complex;

fn main() {
    basic_num();
    bi_oc_hex();
    type_compare();
    new_type_compare();
    complex();
    boolean();
    charater();
    tuple();
    unpack_tuple();
    ref_tuple();
}

fn basic_num() {
    // let 키워드를 사용하여 변수를 선언
    // Rust에서는 타입이 명시되지 않은 경우 컴파일러가 타입을 추론
    let twenty = 20;

    // 타입 애너테이션을 사용하여 타입을 명시적으로 지정할 수 있음
    let twenty_one: i32 = 21;

    // 타입 접미사를 통해 타입을 명시적으로 지정할 수 있음
    // 숫자 리터럴 내에서 _를 사용할 수 있으며, 가독성 향상 이외에 다른 의미는 없음 (컴파일러는 이를 무시함)
    let twenty_two = 22_i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;

    // 숫자는 메서드를 가짐
    println!("{}", one_million.pow(2));

    // 배열은 모두 같은 타입이어야 하며 대괄호로 묶어서 생성함
    let forty_twos = [
        // 명시적인 타입 애너테이션이 없는 부동 소수점 리터럴은 상황에 따라 32비트 혹은 64비트가 됨
        42.0,

        // 부동 소수점 리터럴도 타입 접미사가 붙을 수 있음
        42f32,

        // 부동 소수점 리터럴과 타입 접미사 사이에 추가적인 밑줄도 쓸 수 있음
        42.0_f32,
    ];
    // 배열은 인덱스를 통해 접근할 수 있음
    println!("{:02}", forty_twos[0]);
}


fn bi_oc_hex() {
    // 2진수, 8진수, 16진수 리터럴을 사용할 수 있음
    // 0b : 2진수, 0o : 8진수, 0x : 16진수
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}


fn type_compare() {
    let a: i32 = 10;
    let b: u32 = 20;

    // Rust에서는 타입이 다른 변수 간의 비교를 허용하지 않음
    // 따라서 형변환을 통해 타입을 맞춰줘야 함

    if a < (b as i32) {
        println!("Ten is less than twenty!")
    }
}

fn new_type_compare() {
    let a: i32 = 10;
    let b: u32 = 20;
    
    // b.try_into()는 Result 안에 i32값을 감싸 반환함
    // Result는 성공값 또는 오륫값을 포함할 수 있음
    // unwrap() 메서드는 성공값을 처리하며, b의 값을 i32로 반환
    // u32에서 i32로 변환에 실패할 경우 unwrap()이 호출되고 프로그램이 중단됨
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than twenty!")
    }
}

fn complex() {
    // Rust에는 생성자가 없는 대신 모든 타입에 리터럴 형태가 존재함
    // 복소수는 실수부와 허수부로 이루어져 있으며, re는 실수부, im은 허수부를 나타냄
    let a = Complex { re: 2.1, im: -1.2 };

    // 대부분의 데이터 타입은 정적 메서드 new()를 구현함
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

fn boolean() {
    let t = true;
    let f: bool = false; // 명시적 타입 지정

    println!("t = {}, f = {}", t, f);
}

fn charater() {
    let c = 'c';
    println!("c = {}", c)
}

fn tuple() {
    // tuple의 선언 및 값 할당
    // 소괄호로 값을 묶어서 선언하며, 각각의 값은 콤마로 구분함
    // annotation의 경우에도 동일하게 적용됨
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
}

fn unpack_tuple() {
    let tup = (500, 6.4, 1);
    // tuple의 값을 해체하여 변수에 할당
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn ref_tuple() {
    let tup = (500, 6.4, 1);
    // tuple의 값을 참조하여 변수에 할당
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x = {}, y = {}, z = {}", x, y, z);
}