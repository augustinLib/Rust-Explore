# Ownership
프로그램은 실행 중에 컴퓨터의 메모리를 지속적으로 관리해야한다.  
Python, Go와 같은 언어들은 가비지 콜렉터를 이용하여 사용되지 않는 메모리를 해제하는 방식으로 메모리를 관리하며, C++과 같은 언어들은 개발자가 직접 메모리를 관리해야한다.  
Rust는 이와 다르게, 소유권이라는 개념을 통해 컴파일 타임에 메모리를 관리한다.  


## Ownership Rules
Rust에서는 다음과 같은 규칙을 통해 소유권을 관리한다.
- Rust가 다루는 각각의 값은 owner(소유자)라고 부르는 변수를 가지고 있다.  
- 특정 시점에 값의 소유자는 단 하나만 존재할 수 있다.
- 소유자가 범위를 벗어나면 그 값은 제거된다.

## Variable Scope
Scope(범위)란, 프로그램 안에서 특정 아이템이 유효한 한도를 의미한다.  
```rust
fn variable_scope() {
    // 문자열 리터럴로 변수 s를 생성
    let s = "hello"; // 변수 s는 이 지점부터 유효함

} // 여기서 변수 s는 scope 밖으로 나가서 더이상 유효하지 않음
```

## String Type
String 타입은 Rust에서 제공하는 표준 라이브러리의 타입이다.
이 String 타입은 heap(힙)에 할당되기 때문에, 컴파일 시점에 알 수 없는 크기의 문자열을 저장할 수 있다.  
아래와 같이 `from`함수를 이용하면 문자열 리터럴을 이용해 String 인스턴스를 생성할 수 있다.  
```rust
fn main() {
    // String 타입은 힙에 할당되는 문자열 타입
    // String 타입은 가변적이다.
    // String::from() 함수를 사용하여 문자열 리터럴을 이용해 String 인스턴스 생성 가능
    let mut s = String::from("hello");

    // push_str() 함수는 리터럴을 String에 붙여주는 함수
    // python의 .join()과 비슷한 기능을 한다.
    s.push_str(", world!"); 

    println!("{}", s); // 이 코드는 "hello, world!"를 출력함
}
```

## Memory and Allocation
문자열 리터럴은 컴파일 시점에 문자열의 내용을 알고 있기 때문에 빠르고 효율적이라는 특징을 가짐.  
그러나, 이러한 장점은 문자열 리터럴이 immutable하다는 특징에서 비롯됨.  
만약 문자열 리터럴을 가변적으로 사용하고 싶다면, String 타입을 사용해야한다.  
Mutable한 특징을 가지는 **String 타입은 길이를 조절할 수 있는 텍스트이기에, 컴파일 시점에 알 수 없는 내용을 저장하기 위해 heap(힙) 메모리에 일정 부분 할당된다.**   
이를 위해 아래와 같은 두 가지 단계를 거친다.  
- 해당 메모리는 **런타임에 운영체제에 요청**해야 함
- String 타입의 사용이 완료된 이후에는 해당 **메모리를 운영체제에 반환**해야 함

첫 번째 단계의 경우, `String::from()`함수를 통해 필요한 메모리를 요청한다.  
그러나, 두 번째 단계의 경우, GC가 있는 언어는 자동으로 메모리를 반환하지만, GC가 없는 언어는 개발자가 직접 메모리를 반환해야한다. 이는 매우 큰 어려움을 초래한다.  

Rust에서는 아래와 같이 메모리의 할당과 해제를 다른 방식으로 수행한다.  
> **Rust에서는 변수에 할당된 메모리는 변수를 소유한 scope가 종료되는 시점에 자동으로 해제된다.**

변수가 scope를 벗어나게 되면, Rust는 `drop`이라는 이름의 함수를 호출하여 해당 변수의 메모리를 해제한다.  

<br>

## 변수와 데이터가 상호작용하는 방식 : Move(이동)
Rust에서는 여러 개의 변수를 이용하여 같은 데이터를 각기 다른 방법으로 처리할 수 있다.
아래는 정수를 사용하는 예제이다.  
```rust
fn main() {
    let x = 5;
    let y = x;
}
```
위 에제는 값 5를 변수 `x`에 대입한 이후, 변수 `y`에 `x`의 복사본을 대입한다.  
이 경우 `x`와 `y`에 같은 값 5를 할당하게 되었고, 정수는 이미 알려진 고정 크기의 단순한 값이기에 5라는 값 두 개가 스택에 저장된다.  

그러나, String 타입의 경우, 이와 다르게 동작한다.  
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```
앞선 예제처럼, `s1`에 String 타입의 문자열을 대입한 이후, `s2`에 `s1`의 복사본을 대입할 것으로 예상할 수 있다.  
그러나, 실제로는 이와 다르게 동작한다.  
String 타입은 아래의 그림처럼 문자열 콘텐츠를 저장하고 있는 메모리에 대한 포인터(ptr), 길이(len), 용량 정보(capacity)의 3가지로 구성된다.  
이 3가지 데이터는 stack(스택)에 저장되고, 문자열 콘텐츠는 heap(힙)에 저장된다.

<div align="center">
<img src="https://doc.rust-lang.org/book/img/trpl04-01.svg" width=50%>
</div>

다시 예제로 돌아와서, 변수 `s1`을 `s2`에 대입하면, String 타입의 데이터가 복사된다.  
**즉, 포인터가 가리키는 heap(힙)에 저장된 실제 문자열 데이터가 아닌, 문자열에 대한 포인터, 길이, 용량 정보가 스택에 복사된다.**  
이는 아래의 그림과 같다.
<div align="center">
<img src="https://doc.rust-lang.org/book/img/trpl04-02.svg" width=50%>
</div>

만약 Rust가 아래와 같이 heap 메모리를 복사하게 된다면, heap 메모리의 크기가 매우 큰 경우, heap 메모리를 복사하는데 많은 시간이 소요될 것이다.
<div align="center">
<img src="https://doc.rust-lang.org/book/img/trpl04-03.svg" width=50%>
</div>

앞서 언급한 것처럼, Rust는 변수가 scope를 벗어나면 자동으로 `drop`함수를 호출하여 해당 변수가 사용하던 heap 메모리를 해제한다.  
이 때, **두 개의 변수가 같은 heap 메모리를 가리키고 있다면, 두 변수가 모두 heap 메모리를 해제**하려고 할 것이다.  
이러한 문제는 **double free error(이중 해제 에러)** 라고 하며, 메모리 안전성 버그 중 하나이다.  
메모리를 두 번 해제하는 것은 메모리의 corruption(불순화)을 초래하고, 이는 보안 취약점으로 이어질 수 있다.  
<br>

Rust는 이러한 문제를 방지하기 위해, 할당된 메모리를 복사하는 대신, **이동(move)** 이라는 개념을 사용한다.  
즉, 첫 번째 변수인 `s1`을 무효화하고, 두 번째 변수인 `s2`만 유효하게 만든다.  
(이 경우. 변수 `s1`이 변수 `s2`으로 move(이동)하였다고 한다.)  

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // 변수 s1이 s2로 이동하여 올바르지 못한 참조가 됨
    println!("{}, world!", s1); // error!
    println!("{}, world!", s2);
}
```

이러한 이동(move)의 개념은 아래의 그림과 같다.

<div align="center">
<img src="https://doc.rust-lang.org/book/img/trpl04-04.svg" width=50%>
</div>

그림을 보게 되면, `s1`이 `s2`로 이동하였기 때문에, `s1`은 더 이상 유효하지 않다.(검은색으로 음영처리함으로써 표현)  
따라서, 변수 `s2`만 유효한 상태이기에, 두 변수가 범위를 벗어나게 되어도, 변수 `s2`만 heap 메모리를 해제하게 된다.  

> 다른 언어에서는 실제 데이터를 복사하지 않고 포인터와 길이, 용량만을 복사하는 shallow copy(얕은 복사), 실제 데이터까지 복사하는 deep copy(깊은 복사)를 사용하는 경우와 다르게, Rust는 move(이동)을 사용하여 이러한 문제를 방지하며, 절대 자동으로 deep copy(깊은 복사)를 하지 않는다.

<br>

## 변수와 데이터가 상호작용하는 방식 : Clone(복제)
위에서 살펴본것과 다르게, 힙 메모리에 있는 데이터를 복제하고 싶다면, `clone` 메소드를 사용하면 된다.  
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1의 값을 복사하여 s2에 할당함

    println!("s1 = {}, s2 = {}", s1, s2);
}
```
이는 아래와 같은 작업을 수행한다.  

<div align="center">
<img src="https://doc.rust-lang.org/book/img/trpl04-03.svg" width=50%>
</div>

<br>

## 스택 전용 데이터 : Copy(복사)
아래의 정수형 데이터를 사용하는 예제를 살펴보자
```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```
위 예제에서, `clone`메소드를 사용하지 않았음에도 변수 `x`는 여전히 유효하며, 변수 `y`로 이동하지 않았다.  
그 이유는, **정수형 데이터는 컴파일 시점에 크기가 정해지고, 스택에 저장되기 때문에 실제 값을 복사해도 전혀 부담되지 않기 때문이다.**  
이 경우에는 깊은 복사와 얇은 복사의 차이가 전혀 없기에 `clone`메소드를 호출한다고 해도 차이점이 발생하지 않는다.  
<br>

Rust는 스택에 저장되는 정수형과 같은 데이터 타입에 적용할 수 있는 `Copy` trait를 제공한다.
만약 **특정 타입에 `Copy` trait가 구현되어 있다면, 이전 변수를 새 변수에 할당하여도 무효화되지 않는다.**  
다만, 특정 타입, 혹은 해당 타입의 일부에 `Drop` trait가 적용되어 있으면 `Copy` trait를 적용할 수 없다.  

통상적으로, 단순한 스칼라값에는 `Copy` trait가 적용되어 있으며, 할당이 필요하거나 특정 형태의 자원에도 `Copy` trait가 적용되어 있다. 대표적으로 아래와 같은 타입들이 있다.
- `u32`와 같은 정수형 타입
- true와 false와 같은 불리언 타입
- 문자 타입, `char`
- `f64`와 같은 부동소수점 타입
- `Copy` trait가 적용된 타입을 포함하는 tuple. 예를 들어, `(i32, i32)`는 `Copy` trait가 적용되지만, `(i32, String)`은 적용되지 않는다.


## Ownership(소유권)과 함수
값을 함수에 전달한다는 의미는 값을 변수에 대입하는 것과 유사하다.  
변수를 함수에 전달하게 되면 대입과 마찬가지로 값의 move(이동)나 copy(복사)가 발생한다.  
```rust
fn main() {
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
```

## Return Values and Scope(반환값과 범위)
return값의 경우에도 소유권을 이전한다.  
```rust
fn main() {
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
```
값을 다른 변수에 할당하면 move가 발생하고, ownership이 옮겨진다.  
Heap 메모리에 저장된 변수의 데이터는, ownership이 옮겨지지 않았다면 scope를 벗어날 때 `drop` 함수가 호출되어 메모리가 해제된다.

이러한 특징은 **함수에 전달했던 변수를 나중에 다시 사용하기 위해 매번 그 변수값을 함수의 실제 return값과 함께 다시 반환**하도록 해야 한다. 아래의 예제는 tuple을 이용하여 여러 값(원본과 수정된 값)을 반환하는 예제이다.
```rust
fn main() {
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
```

이러한 방식은 매우 복잡하기에, reference(참조)를 이용하여 보다 간단히 해결할 수 있다.

<br>

## References and Borrowing(참조와 대여)
기존 방식처럼 함수로 하여금 원본 값도 반환하게 하여 ownership을 다시 되돌려주는 것은 매우 비효율적이다.  
이러한 방법 대신, 함수의 parameter로 전달된 객체의 reference를 이용하도록 할 수 있다.
```rust
fn main() {
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
```
위 예제의 동작 방식은 아래와 같다.

<div align="center">
<img src="https://doc.rust-lang.org/book/img/trpl04-05.svg" width=50%>
</div>

`&s1`를 사용하면 **변수 `s1`의 값은 읽을 수 있지만, ownership은 가져오지 않는 reference를 생성**한다.  
Reference는 ownership을 갖지 않기 때문에 reference가 가리키는 값(변수 `s1`)은 reference가 scope를 벗어나더라도 `drop`함수가 호출되지 않는다

또한, 이처럼 **함수의 parameter로 reference를 전달하는 것을 borrowing(대여)** 라고 한다.  
추가적으로, 변수가 기본적으로 immutable인것처럼, reference 역시 기본적으로 immutable하다. 따라서 참조하고 있는 값을 변경할 수 없다.  
그렇다면 참고하고 있는 값을 변경하고 싶다면 어떻게 해야할까?  

## Mutable References(가변 참조자)
만약 참조하고 있는 값을 변경하고 싶다면, `&mut`을 사용하여 mutable reference를 생성해야 한다.
```rust
fn main() {
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
```

그러나 이러한 가변 참조에도 제약이 있다.  
바로 **특정 scope 내에서의 특정 변수에 대한 가변 참조자는 오직 하나만 존재**할 수 있다는 것이다.  
아래 예제를 보자.
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s; // 가변 참조자 생성
    let r2 = &mut s; // 가변 참조자 생성

    println!("{}, {}", r1, r2); // error!
}
```
위 예제는 실행되지 않는다. main함수의 scope 내에서 s에 대한 가변 참조자가 두 개 존재하기 때문이다.  

이러한 제약 덕분에 Rust는 data races(데이터 경합)을 컴파일 시점에 방지할 수 있다.
Data races(데이터 경합)는 예측할 수 없는 결과를 유발하며, 런타임에 원인을 파악하고 수정하기 매우 어렵다.  

다만, 하나의 변수에 대해 여러 가변 참조자를 활용하고 싶으면, 아래와 같이 중괄호를 이용하면 된다.
```rust
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s; // 가변 참조자를 생성함
    } // scope 밖으로 벗어나면서 가변 참조자가 소멸됨
    
    let r2 = &mut s; // 두 번째 가변 참조자를 생성함
    // 앞선 예제와는 다르게, 다른 scope에서 가변 참조자를 생성하였기 때문에 에러가 발생하지 않음
}
```

만약 scope 내에서 이미 불변 참조를 사용하고 있다면, 가변 참조자를 생성할 수 없다.  
아래의 예제는 에러를 발생시킨다.  
```rust
fn main() {
    let mut s = String::from("hello");

    // 불변 참조자는 읽기 전용이기에 scope 내에서 여러 개를 생성해도 문제가 없음
    let r1 = &s; // 불변 참조자 생성
    let r2 = &s; // 불변 참조자 생성

    // 이미 불변 참조자가 존재하기 때문에 가변 참조자를 생성할 수 없음
    let r3 = &mut s; // 가변 참조자 생성

    println!("{}, {}, and {}", r1, r2, r3); // error!
}
```

## Dangling References(죽은 참조)
포인터를 사용하는 언어의 경우, dangling pointer(죽은 포인터)로 인해 에러가 발생할 수 있다.  
Dangling pointer(죽은 포인터)란, 메모리가 해제되었거나, 다른 변수가 할당되었음에도 불구하고 여전히 해당 메모리를 참조하고 있는 포인터를 의미한다.  

Rust는 dangling pointer(죽은 포인터)를 컴파일 시점에 방지할 수 있다.  
즉, 어떤 데이터에 대한 reference를 생성할 때, 컴파일러가 해당 데이터에 대한 참조를 실행하기에 앞서 해당 데이터가 범위를 벗어나지 않았는지를 확인한다.  
아래 예제는 죽은 참조를 생성하는 예제이다.
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // String에 대한 참조자를 반환함
    let s = String::from("hello"); // s는 새로운 String을 생성함

    &s // s의 참조자를 반환함
} // s는 scope를 벗어나고, drop 함수가 호출됨. 따라서 s는 메모리에서 해제됨
// 따라서, s의 참조자를 반환하는 것은 불가능함
```
변수 `s`가 `dangle`함수 내에서 생성되었기 때문에, `dangle`함수가 종료되면서 `s`는 메모리에서 해제된다.  
그러나 이 함수는 해제된 메모리(변수 `s`)를 참조하고 있는 참조자를 반환하고 있다.  
이는 메모리 참조 에러를 발생시키는데, Rust는 이러한 상황을 컴파일 시점에 방지한다.

이러한 문제를 해결하려면, 아래와 같이 참조자가 아닌, String 타입을 반환해야 한다.  
```rust
fn main() {
    let reference_to_nothing = no_dangle();
}

fn no_dangle() -> String { // String 타입을 반환함
    let s = String::from("hello");

    s // s를 반환함
}
```

## 참조에 대한 규칙
지금까지 알아본 참조자에 대한 규칙을 정리하면 아래와 같다.
- 어느 한 scope 내에서, 하나의 가변 참조 혹은 여러 개의 불변 참조를 생성할 수 있지만, 가변 참조와 불변 참조 모두를 동시에 생성할 수 없다.
- 참조자는 항상 유효해야 한다.

## Slice 타입
Slice는 참조자와 같이 ownership을 갖지 않는 데이터 타입이다.  
Slice를 사용하면 컬렉션 전체가 아닌, 컬렉션 내의 일부 연속된 요소에 대한 참조자를 생성할 수 있다.  
