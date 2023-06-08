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