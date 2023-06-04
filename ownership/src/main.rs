fn main() {
    variable_scope();
    string_type(); 
    clone();
    integer();
    ownership_function();
    ownership_return();
    return_tuple();
    with_reference();
    with_mut_reference();
    is_dangle();
}

fn integer() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn variable_scope() {
    let s = "hello"; // 변수 s는 이 지점부터 유효함

} // 여기서 변수 s는 scope 밖으로 나가서 더이상 유효하지 않음

fn string_type() {
    // String 타입은 힙에 할당되는 문자열 타입
    // String 타입은 가변적이다.
    // String::from() 함수를 사용하여 문자열 리터럴을 이용해 String 인스턴스 생성 가능
    let mut s = String::from("hello");

    // push_str() 함수는 리터럴을 String에 붙여주는 함수
    // python의 .join()과 비슷한 기능을 한다.
    s.push_str(", world!");

    println!("{}", s); // 이 코드는 "hello, world!"를 출력함
}

fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1의 값을 복사하여 s2에 할당함

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership_function() {
    let s = String::from("hello"); // 변수 s가 범위 내에 생성됨

    takes_ownership(s); // s의 값이 함수 안으로 이동함
    // 이 시점부터 변수 s는 유효하지 않음 (move)
    // 따라서 이 이후에 s를 사용하려고 하면 에러가 발생함 

    let x = 5; // 변수 x가 범위 내에 생성됨
    makes_copy(x); // x의 값이 함수 안으로 복사됨(copy)
    // i32 타입은 copy를 수행하기 때문에 이 시점 이후에도 변수 x는 유효함

}   // 이 지점에서 변수 x가 범위를 벗어난 후, 변수 s도 범위를 벗어남
    // 하지만 s는 이미 이동되었으므로, 별다른 일이 발생하지 않음 

fn takes_ownership(some_string: String) { // some_string이 scope 안으로 들어왔음
    println!("{}", some_string);
} // 여기서 some_string이 scope 밖으로 벗어났음. drop 함수가 호출됨
// some_string의 메모리는 해제됨 (힙)

fn makes_copy(some_integer: i32) { // some_integer가 scope 안으로 들어왔음
    println!("{}", some_integer);
} // 여기서 some_integer가 scope 밖으로 벗어났음. 아무 일도 일어나지 않음 (스택)

fn ownership_return() {
    let s1 = gives_ownership(); // gives_ownership 함수가 반환한 값을 s1으로 move함

    // 변수 s2가 scope 내에서 생성됨
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2의 값이 함수 안으로 이동되었고, s3으로 반환되었음
} // 이 시점에서 변수 s3이 scope 밖으로 벗어났고, drop 함수가 호출됨
// 변수 s2도 범위를 벗어나지만 take_and_gives_back 함수에 의해 이동되었으므로 아무 일도 일어나지 않음
// 변수 s1도 범위를 벗어났기 때문에 drop 함수가 호출됨

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string이 scope 내에서 생성됨

    some_string // some_string의 값이 반환되고, 호출된 함수로 move됨
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string의 값이 반환되고, 호출된 함수로 move됨
}

fn return_tuple() {
    let s1 = String::from("hello");

    // tuple을 통해 return되는 여러 값을 반환받을 수 있음
    let (s2, len) = calculate_length(s1); // s1의 값이 함수 안으로 move되었고, s2로 반환되었음
    // 이때, s1은 더이상 유효하지 않음
    
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 함수는 문자열의 길이를 반환함

    // tuple을 이용하여 여러 값을 반환할 수 있음
    // 이때, 함수에 전달했던 변수를 다시 사용하기 위해 수정된 값과 함께 반환함
    (s, length)
}

fn with_reference() {
    let s1 = String::from("hello");

    let len = calculate_length_reference(&s1); // &s1을 통해 s1의 참조자를 전달함

    println!("The length of '{}' is {}.", s1, len); // 참조자를 전달했기 때문에 여전히 s1을 사용할 수 있음
}

// & : 참조자(reference)
// 참조자를 이용하면 변수를 함수에 전달할 때, 변수의 소유권을 넘기지 않고도 변수를 참조할 수 있음
// 이렇게 함수 매개변수로 reference를 전달하는 것을 'borrowing'이라고 함
fn calculate_length_reference(s: &String) -> usize {
    s.len()
} // 여기서 s는 scope 밖으로 벗어나지만, 참조자이기 때문에 아무 일도 일어나지 않음


fn with_mut_reference() {
    // let mut 키워드로 mutable한 변수를 생성함
    let mut s = String::from("hello");

    change(&mut s); // &mut s를 통해 s의 가변 참조자를 전달함

    println!("{}", s); // 참조자를 전달했기 때문에 여전히 s를 사용할 수 있음
}

// &mut : 가변 참조자(mutable reference)
// 함수에서 &mut 키워드를 통해 가변 참조자를 전달받음
fn change(some_string: &mut String) {
    some_string.push_str(", world"); // 가변 참조자를 통해 s의 값을 변경함
}

fn is_dangle() {
    let reference_to_nothing = no_dangle();

    println!("{}", reference_to_nothing);
}

fn no_dangle() -> String { // String 타입을 반환함
    let s = String::from("hello");

    s // s를 반환함
}