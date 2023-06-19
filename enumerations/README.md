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

