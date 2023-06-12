struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    basic_structure();
    mutable_structure();
    let user = struct_function(String::from("user2"), String::from("user2@gmail.com"));
    println!("{}", user.email);
    let user2 = struct_function_with_shorcut(String::from("user2"), String::from("user2@gmail.com"));
    println!("{}", user2.email);
    no_struct_update_syntax();
    with_struct_update_syntax();
    tuple_struct();
    rectangle_1();
    rectangle_2();
    rectangle_3();
    print_rectangle();
    print_rectangle2();
    area_with_method();
}


fn basic_structure() {
    let user1: User = User {
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        active: true,
        sign_in_count: 1,
    }; // 인스턴스 생성 시 세미콜론 필요

}

fn mutable_structure() {
    // mut 키워드로 가변 인스턴스 생성
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        active: true,
        sign_in_count: 1,
    }; // 인스턴스 생성 시 세미콜론 필요

    user1.email = String::from("user1@naver.com"); // 변경 가능
    println!("{}", user1.email);
}

fn struct_function(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }   
}

fn struct_function_with_shorcut(email: String, username: String) -> User {
    User {
        username, // 변수명과 필드명이 같을 경우 생략 가능
        email, // 변수명과 필드명이 같을 경우 생략 가능
        active: true,
        sign_in_count: 1,
    }   
}

fn no_struct_update_syntax() {
    let user1: User = User {
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@gmail.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user2.sign_in_count);
}

fn with_struct_update_syntax() {
    let user1: User = User {
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@gmail.com"),
        ..user1 // 나머지 필드는 user1의 값으로 초기화
    };
    println!("{}", user2.sign_in_count);
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn rectangle_1() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area1(width1, height1));
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn rectangle_2() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area2(rect1));
}

fn area2(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn rectangle_3() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area3(&rect1));
}

// &Rectangle: Rectangle 인스턴스를 참조하는 불변 참조자
// 참조자를 사용하여 Rectangle 인스턴스를 전달하면, 인스턴스의 소유권을 넘기지 않고 Borrowing
// 이를 통해 main함수는 여전히 rect1에 대한 소유권을 가지며, 계속 사용할 수 있음
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn print_rectangle() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1); // {:?}를 사용하여 디버깅 출력
}

fn print_rectangle2() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?}", rect1); // {:?}를 사용하여 디버깅 출력
}

fn area_with_method() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}