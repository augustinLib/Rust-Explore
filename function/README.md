# Function
Rust에서는 함수와 변수명을 선언할 때 기본적으로 snake_case를 사용한다.  
함수 선언의 경우, `fn`키워드와 함수의 이름, 소괄호로 구성된다.  
이어지는 중괄호의 경우, 컴파일러에게 함수 본문의 시작과 끝을 알려준다.  

## Statements and Expressions
함수 본문은 statement(구문)와 expression(표현식)으로 구성된다.  

### Statements(구문)
statement(구문)는 **어떤 작업을 수행하지만 값을 반환하지 않는다.**  
예를 들어, `let` 키워드를 사용하여 변수를 선언하는 것은 statement(구문)이다.
```rust
fn main() {
    let x = 5;
}
```
함수 선언 역시 statement(구문)이다. 따라서 위의 예제는 statement 안에 statement가 존재한다.  
앞서 말했듯이, statement는 값을 반환하지 않기 때문에 다른 변수에 대입할 수 없다.  
이러한 특성은, 대입문이 대입된 값을 반환하는 C나 Ruby와 같은 언어와는 다르기에, 아래와 같은 코드는 컴파일 에러를 발생시킨다.
```rust
fn main() {
    let x = (let y = 6);
}
```

### Expressions(표현식)
Expression(표현식)은 어떤 값으로 평가된다. `5+6`과 같은 구문은 expression(표현식)이며, 11이라는 값으로 평가된다.  
이러한 expression(표현식)은 statement(구문)의 일부가 될 수 있다.  


```rust
fn main() {
    let x = 5;

    // {}는 범위를 표현하는 코드 블록이며, 이 역시 expression(표현식)이다.
    let y = {
        let x = 3;

        // expression(표현식)은 마지막에 ;를 붙이지 않는다.
        // ;를 붙이면 statement(구문)이 되어 값을 반환하지 않는다
        x + 1
    };

    println!("y: {}", y);
}
```

## Functions with Return Values
Function에 반환값이 존재할 때, `->`를 사용하여 반환값의 타입을 명시한다.  
Rust에서는 함수의 반환값을 명시적으로 선언하지 않아도, 컴파일러가 함수 본문의 마지막 표현식을 반환값으로 사용한다.  
> 함수 본문의 실행이 완전히 끝나지 않아도 `return`키워드로 반환값을 명시할 수 있다.  
```rust
fn main() -> i32 {
    // expression(표현식)은 마지막에 ;를 붙이지 않는다.
    // ;를 붙이면 statement(구문)이 되어 값을 반환하지 않는다
    10
}
```
```rust
fn main(x: i32) -> i32 {
    // expression(표현식)은 마지막에 ;를 붙이지 않는다.
    // ;를 붙이면 statement(구문)이 되어 값을 반환하지 않는다
    x + 1
}
```
