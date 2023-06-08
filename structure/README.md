# Structure
Structure(구조체)의 정의는 `struct`키워드 다음에 구조체에 부여할 이름을 지정해주면 된다.  
그런 다음 중괄호 안에 구조체가 저장할 데이터의 **타입과 이름**을 나열하면 된다.  
이때, 이 데이터들을 field(필드)라고 부른다.  
이는 아래의 예제와 같다.  
```rust
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
}
```
구조체를 정의한 후 이를 사용하려면 각 필드에 저장할 값을 명시하여 구조체의 instance(인스턴스)를 만들면 된다.  
구조체의 인스턴스를 생성하려면, 해당 구조체의 이름과 중괄호를 사용하여 **키 값**의 쌍으로 각 필드의 값을 지정해주면 된다.  
이는 아래의 예제와 같다.  
```rust
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        active: true,
        sign_in_count: 1,
    }; // 인스턴스 생성 시 세미콜론 필요
}
```
구조체에서 원하는 값을 가져오려면, `.`을 사용하면 된다. (python과 동일)  
만약 인스턴스가 가변 인스턴스라면, 이를 이용해 필드에 새로운 값을 대입할 수 있다.  
```rust
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

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
```
가변 인스턴스(mutable instance)를 생성할 때 주의해야 할 점은, **Rust는 몇몇 필드만을 가변 데이터로 표시하는 것을 지원하지 않으며, 가변 인스턴스로 생성된 인스턴스 자체가 가변이라는 것**이다.  

이에 더하여, 다른 표현식과 마찬가지로 구조체의 새로운 인스턴스도 함수를 이용해 생성할 수 있다.  
이때, 함수의 **마지막 표현식(exression)은 묵시적으로 새 인스턴스를 반환**해야 한다.
```rust
fn struct_function(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }   
}
```
위 예제를 보면, 함수의 parameter와 구조체의 필드의 이름이 같다.  
이 경우 field init shorthand syntax(필드 초기화 단축 문법)을 사용하여 `email`이나 `username`등 변수 이름을 일일히 반복해서 입력하지 않아도 같은 결과를 얻을 수 있다.  
아래의 예제는 field init shorthand syntax가 적용된 예제이다.
```rust
fn struct_function_with_shorcut(email: String, username: String) -> User {
    User {
        username, // 변수명과 필드명이 같을 경우 생략 가능
        email, // 변수명과 필드명이 같을 경우 생략 가능
        active: true,
        sign_in_count: 1,
    }   
}
```
## 기존 instance로부터 새로운 instance 생성하기
이미 존재하는 instance로부터 몇 가지 필드의 값만 수정한 상태로 새로운 instance를 생성할 수 있다.  
이는 struct update syntax(구조체 갱신 문법)을 사용하면 된다.  
우선, struct update syntax를 사용하지 않고 구현한 예제를 살펴보자.  
```rust
fn main() {
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
}
```
struct update syntax를 사용하지 않는 경우, 위 예제에서와 같이 `.`을 이용하여 user1 인스턴스의 필드에 접근하여, 해당 값을 user2 인스턴스의 필드에 대입해주어야 한다.  
struct update syntax를 사용하는 경우, 아래와 같이 간략히 표현할 수 있다.  
```rust
fn main() {
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
}
```
`..`문법을 통해 나머지 필드는 user1의 값으로 초기화하였다.  
이러한 `..`은 명시적으로 나열하지 않은 나머지 필드에 대해서는 기존의 인스턴스의 필드와 같은 값을 사용하라는 의미를 담고 있다.  

## Tuple Structs(튜플 구조체)
Rust에서는 tuple과 유사한 tuple structs(튜플 구조체)를 지웒한다.  
tuple structs는 구조체에는 이름을 부여하지만, 필드에는 이름을 붙이지 않고 타입만 지정한다.  
이러한 tuple structs는 일반적인 구조체처럼 필드에 이름을 부여하는 것이 불필요하거나, tuple 자체에만 이름을 부여하여 다른 tuple과 구분하고자 할 때 사용한다.  

이러한 tuple structs는 `struct`키워드와 구조체의 이름, 그리고 튜플 안에서 사용할 타입을 차례대로 나열하여 정의한다.  
아래의 예제는 tuple structs를 정의하는 예제이다.  
```rust
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
이러한 tuple structs는 tuple과 같이 동작한다. 각 값에 대응하는 변수로 해체할 수 있고, `.`와 인덱스를 통해 개별 필드값에 접근할 수 있다.  