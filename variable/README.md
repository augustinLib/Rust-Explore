# Variable
Rust에서의 변수 선언은 `let` 키워드를 사용하며, 이렇게 선언된 변수는 기본적으로 immutable하다. 이러한 immutable variable에 한번 값을 할당하게 되면 값을 변경할 수 없다.  
mutable variable을 사용하기 위해서는 `mut` 키워드를 사용해야 한다.  
```rust
fn main() {
    // mutable한 변수 생성
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    // mutable한 변수이기에 값 변경 가능
    x = 6;
    println!("The value of x is: {}", x);
}
```

## Rust에서의 variable과 constant의 차이
변수의 값을 변경할 수 없다는 점에서는, 타 언어의 상수 (`const`)와 비슷하다.  
Immutable variable과 constant는 한번 할당한 값을 변경할 수 없다는 점이 같지만, 아래와 같은 차이점이 존재한다.  
- Constant에는 `mut`키워드를 사용할 수 없다. Constant는 기본 선언으로 인해 immutable한게 아니라 항상 immutable하다.
- Constant는 `let`키워드가 아닌 `const`키워드로 선언하며, **할당할 값의 타입을 반드시 선언해야 한다**
- Constant는 전역 범위를 비롯하여 원하는 어떤 범위 내에서도 선언할 수 있다.  
- Constant는 반드시 상수 표현식을 사용하여 값을 할당해야 하며, **함수 호출 결과나 런타임에 계산되는 값은 할당할 수 없다.**  


```rust
fn main() {
    // 상수 선언 및 값 할당
    // const 키워드를 사용하여 타입을 명시하여 선언하였으며, 상수 표현식을 사용하여 값 할당
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
```

## Shadowing (변수 가리기)
이미 선언한 변수의 이름과 같은 이름의 변수를 선언할 수 있다. 이렇게 선언된 변수는 기존의 변수를 가리게 되며, 이를 shadowing이라고 한다.  
> 이때, `mut`키워드를 사용한 값 재할당과는 다르다.  
> `let`키워드를 사용하여 변수를 새로 선언하지 않고 값만 할당하면 컴파일 에러가 발생한다.  
> 또한 새롭게 변경된 변수도 여전히 immutable하다.  
> <br>
> 추가적으로, `mut`키워드를 사용했을때와는 다르게 새로운 변수를 선언하는 것이기에, **이름은 같지만 타입이 다른 변수로 사용할 수 있다.**  
> **(`mut`키워드를 사용하여 immutable한 변수를 만들었을 경우에는 타입 변경은 불가능함)**

```rust
fn main() {
    // 변수 x 초기 선언 및 할당
    let x = 5;

    // 변수 x 재선언 및 재할당
    let x = x + 1;

    // 변수 x 재선언 및 재할당
    let x = x * 2;
    
    println!("The value of x is: {}", x);
}
```