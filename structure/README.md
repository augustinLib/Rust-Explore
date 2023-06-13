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

## Unit-like Structs(유사 유닛 구조체)
Rust에서는 구조체를 선언할 때, 필드를 하나도 가지지 않는 구조체를 선언할 수 있다.  
이러한 구조체를 **Unit-like Structs(유사 유닛 구조체)**라고 한다. 왜냐하면, 유닛 타임, 즉 `()`과 유사하게 동작하기 때문이다.  
이러한 Unit-like Structs는 특정 타입의 trait를 구현해야 하지만, 타입에 저장할 데이터가 없을 때 유용하게 사용된다.  

## 구조체를 사용하는 예제 프로그램
위에서 살펴본 구조체를 사용하는 예제 프로그램을 살펴보자.  
아래 예제는 사각형의 면적 크기를 구하는 rust 프로그램이다.  
```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
위 예제는 사각형의 너비와 높이를 각각 `width`와 `height`이라는 변수에 저장하고, `area`함수를 호출하여 사각형의 면적을 구하는 예제이다.  

다만, 위 예제의 경우 면적을 구하기 위해 `width`와 `height` 두 개의 parameter를 사용하였다. 두 parameter는 서로 연관이 있지만 코드상에서는 이 관계를 표현하지 않는다.  
따라서, 위 예제는 아래와 같이 tuple로 refactoring할 수 있다.  
```rust
fn main() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rect1));
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
```
위 예제는 tuple덕분에 `area` 함수에 하나의 parameter만 전달하면 되는 장점이 있다.  
그러나, tuple은 요소에 이름을 부여하진 않기에, 여전히 명확하지 않다는 한계가 있다.  
따라서, 위 예제는 아래와 같이 구조체를 사용하여 refactoring할 수 있다.  
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

// &Rectangle: Rectangle 인스턴스를 참조하는 불변 참조자
// 참조자를 사용하여 Rectangle 인스턴스를 전달하면, 인스턴스의 소유권을 넘기지 않고 Borrowing
// 이를 통해 main함수는 여전히 rect1에 대한 소유권을 가지며, 계속 사용할 수 있음
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

## Trait를 상속하여 유용한 기능 추가하기
위에서 살펴본 rectangle 예제에서, 인스턴스의 필드가 가진 값들을 출력하는 기능을 추가하면 디버깅 측면에서 유용할 것이다.  
이를 위해, 아래와 같이 구현을 진행하면 어떤 결과가 나오게 될까?
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1: {}", rect1); // error!
}
```
위와 같은 코드는 아래와 같은 에러를 발생시킨다. 
> error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

`println!` 매크로는 다양한 형식으로 문자열을 출력할 수 있으며, `{}`는 `println!` 매크로가 해당 값의 `Display` 형식을 출력하도록 한다.  
`Display`형식은 이제까지 다뤘던 기본 타입들에는 구현이 되어있지만, 사용자 정의 타입에는 구현되어 있지 않다. (python의 `__repr__`과 `__str__`과 비슷한 개념)  
이러한 기본 타입들은 스스로를 표현하는 방법이 제한되어 있는데(`i32`타입의 값 1은 숫자 1로만 표현됨), 구조체의 경우에는 자신을 출력할 방법이 하나 이상이다. (값들을 쉼표로 구분해야 할지, 모든 필드를 다 보여줘야할지 등등)  
따라서, Rust에서는 이러한 불명확성때문에 구조체가 `Display`를 구현하지 않는다.  

이를 해결하기 위해, `Debug`라는 출력 형식을 사용하여 결과를 출력할 수 있다. `Debug` trait는 `println!` 매크로에서 `{:?}`를 사용하여 출력할 수 있으며, 개발자들에게 유용한 형식으로 구조체의 값을 출력해준다.  
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    //{:?}를 사용하여 Debug 출력 형식 사용
    println!("rect1 is {:?}", rect1); // error!
}
```

그러나 위에 예제는 여전히 아래와 같은 에러를 발생시킨다.  
> error[E0277]: `Rectangle` doesn't implement `Debug`  

Rust는 `Debug` trait를 통해 디버깅 정보를 출력하는 기능을 제공하지만, 이를 사용하기 위해서는 구조체가 `Debug` trait를 구현해야 한다.  
이를 위해, `#[derive(Debug)]` annotation을 구조체 정의에 추가해줘야 한다.  
```rust
// #[derive(Debug)] annotation을 구조체 정의에 추가
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    //{:?}를 사용하여 Debug 출력 형식 사용
    println!("rect1 is {:?}", rect1); 
}

// rect1 is Rectangle { width: 30, height: 50 }
```
`rect1 is Rectangle { width: 30, height: 50 }`와 같이 정상적으로 구조체의 값을 출력할 수 있게 되었다.  
만약 구조체에 포함된 필드의 수가 많다면, `{:?}` 대신 `{:#?}`를 사용하여 출력할 수 있다.  
```rust
// #[derive(Debug)] annotation을 구조체 정의에 추가
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    //{:?}를 사용하여 Debug 출력 형식 사용
    println!("rect1 is {:?}", rect1); 
}

// rect1 is Rectangle {
//     width: 30,
//     height: 50,
// }
```
그러면 한 줄로 출력되는 것이 아닌, 필드별로 한 줄씩 출력되는 것을 확인할 수 있다.  

## Method(메서드)
Rust에서의 메서드는 구조체의 context 내에서 정의되며, **첫 번째 parameter로 항상 메서드를 호출할 구조체의 인스턴스를 표현하는 `self`를 가진다.** (python과 동일)  
이는 아래와 같이 구현할 수 있다.  
```rust
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

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```
위 예제 기준으로, Rectangle 타입의 context 안에 메서드를 정의하기 위해 `impl` 블록을 사용하였다.  
이후 `impl` 블록 안에 `area`메서드를 정의하고, `self`를 통해 메서드를 호출할 구조체의 인스턴스를 표현하였다.  

## Associated Function(연관 함수)
`impl` 블록에서는 self parameter를 가지지 않는 함수를 정의할 수도 있다. 이를 Associated Function(연관 함수)이라고 한다.  
연관 함수들은 self paramter를 통해 인스턴스를 진적 전달받지 않기 때문에 메서드가 아니라 함수이다.  
연관 함수의 대표적인 예로, `String::from`이 있다.   

이러한 연관 함수는 구조체의 새로운 인스턴스를 반환하는 constructor(생성자)로서 사용된다.   
예를 들어, 하나의 매개변수를 전달받아 이 값을 너비와 높이로 지정하는 `Rectangle` 구조체의 연관 함수를 정의해보자.  
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Rectangle 구조체 타입 인스턴스 반환
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```
이러한 연관 함수는 구조체의 이름과 `::` 을 통해 호출할 수 있다. (`String::from`이 그 예이다.)
즉, 아래와 같이 호출 가능하다.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Rectangle 구조체 타입 인스턴스 반환
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);
}
```

## Multiple `impl` Blocks
각 구조체는 여러 개의`impl` 블록을 선언할 수 있다. 예를 들어, 