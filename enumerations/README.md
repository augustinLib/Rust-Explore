# Enumeration
어떤 상황에서 structure(구조체) 대신 enumerate(열거자)를 사용해야 할까?  
IP주소를 다루는 상황을 생각해보자. 현재의 IP주소는 버전 4와 버전 6 두 가지가 표준으로 자리잡고 있다.  
따라서, 프로그램이 다루개 될 수 있는 IP주소의 종류는 두 가지가 되며, 각각의 값에 이름을 부여하여 열거할 수 있다.  

그러나, IP주소는 두 형식을 모두 지원할 수는 없다. 이러한 특징은 enumeration을 사용하기에 적합하다.  
왜냐하면, **enumeration에 나열한 값은 반드시 하나만 사용할 수 있기 때문**이다.  
IP주소의 형식이 버전 4이든 버전 6이든, 결국은 IP주소이기에 enumeration을 사용하여 IP주소를 같은 값으로 처리할 수 있게 된다.  
이는 아래와 같이 표현 가능하다.
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
이렇게 `enum`키워드로 정의한 IpAddrKind 열거자는 이제 코드에서 사용할 수 있는 하나의 타입으로 간주된다.  

## Enumerate Values
IpAddrKind 열거자의 각 값을 표현하는 인스턴스는 다음과 같이 생성한다.  
```rust
// enum에 debug 트레잇을 구현하여, println! 매크로에서 사용할 수 있도록 한다.
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

let four= IpAddrKind::V4;
let six = IpAddrKind::V6;
```
위 예제와 같이, 열거자의 각 값은 식별자를 이용하여 구분하며, 식별자와 값을 구분하기 위해 두 개의 콜론(`::`)을 사용한다.  
또한, 아래와 같이 열거자를 매개변수로 갖는 함수를 정의할 수 있다.  
```rust
enum IpAddrKind {
    V4,
    V6,
}

fn enums_parameter(ip_type: IpAddrKind) {
    println!("ip_type: {:?}", ip_type);
}

fn main() {
    enums_parameter(IpAddrKind::V4);
    // ip_type: V4 출력됨
}
```

이어서, 열거자와 구조체를 아래와 같이 조합하여 사용할 수 있다.  
```rust
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}, loopback: {:?}", home, loopback);
}
```

위 예제처럼 구조체와 열거자를 조합하여 사용함으로써, IP 타입과 그 값 모두를 하나로 묶어서(구조체) 표현할 수 있다.  

그러나, 위와 같은 방법 이외에도 열거자를 구조체에 넣지 않고, 열거자만으로도 IP 타입과 그 값 모두를 하나로 묶어서 표현할 수 있다.  
아래와 같이, V4와 V6의 값을 정의하면서, 연관된 값의 타입을 `String`타입으로 명시할 수 있다.  
```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
}
```
위 예제를 보면, 열거자의 값에 데이터를 직접 지정하는 것을 볼 수 있다. 위**와 같이 열거자를 정의하면서 타입을 명시해주고, 열거자의 값에 데이터를 직접 지정하는 방법을 통해, 별도의 구조체를 정의하지 않고도** IP 타입과 그 값 모두를 하나로 묶어서 표현할 수 있다.  

또한, 위와 같은 방법은 **열거자에 나열된 각각의 값들이 서로 다른 수와 타입의 데이터를 가질 수 있다**는 장점을 가진다.  
아래의 예제는 V4 형식은 `u8`값 4개, V6 형식은 `String`값 1개를 가지고 있다.  
```rust
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // u8, u8, u8, u8
    let home = IpAddr::V4(127, 0, 0, 1);

    // String
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
}
```
추가적으로, 열거자의 값에는 **문자열, 숫자, 구조체 등 어떤 종류의 데이터도 저장할 수 있다.** 심지어 다른 열거자를 저장해도 무방하다.  

## Enumerate Values with Different Types
아래의 예제를 통해, 다른 수와 다른 타입의 데이터를 가지는 열거자에 대해 자세히 알아보자.  
```rust
#[derive(Debug)]
enum Message {
    // Quit값은 연관 데이터를 갖지 않는다.
    Quit,

    // Move값은 anonymous struct(익명 구조체)를 갖는다.
    Move { x: i32, y: i32 },

    // Write값은 1개의 String type 값을 갖는다.
    Write(String),

    // ChangeColor값은 3개의 i32 type 값을 갖는다.
    ChangeColor(i32, i32, i32),
}
```
이와 같은 열거자를 정의하는 것은, 아래와 같이 각각의 항목에 대한 각기 다른 구조체 타입을 정의하는 것과 유사하다.  
```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```
위 두 예제가 다른 점은, 열거자를 사용한 예제는 `struct`키워드를 사용하지 않으며, 열거자의 개별 값들은 모두 `Message`타입이라는 것이다. (아래의 예제는 각 구조체의 타입임 Ex : `QuitMessage`, `MoveMessage` 등)

또한, 열거자 역시 **구조체처럼 `impl` 블록을 사용하여 메소드를 정의할 수 있다.**   
```rust
#[derive(Debug)]
enum Message {
    // Quit값은 연관 데이터를 갖지 않는다.
    Quit,

    // Move값은 anonymous struct(익명 구조체)를 갖는다.
    Move { x: i32, y: i32 },

    // Write값은 1개의 String type 값을 갖는다.
    Write(String),

    // ChangeColor값은 3개의 i32 type 값을 갖는다.
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call: {:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```
## Option 열거자를 Null값 대신 사용할 때의 장점
Rust의 표준 라이브러리가 제공하는 `option`열거자는 매우 다양한 곳에서 사용될 수 있다.  
`Option`열거자는 특정 값이 존재하거나 존재하지 않는, 범용적인 상황에 적합하도록 고안되었기 때문이다.  

**Rust는 다른 언어와 달리 null값이 없다.**  
Null값의 경우, Null값을 Null이 아닌 값처럼 사용할 경우 에러가 발생한다는 큰 문제를 발생시킨다.  
Rust에서는 이러한 문제를 예방하기 위해, Null값이 없으며, 대신 특정 값의 존재 여부를 표현하는 `Option<T>`열거자를 사용한다.  
이는 아래와 같이 표준 라이브러리에 정의되어 있다.  
```rust
enum Option<T> {
    Some(T),
    None,
}
```