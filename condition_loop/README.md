# Condition_Loop

Rust의 조건문 및 반복문

## For
Rust에서의 for문은 기본적으로 아래와 같은 형태를 지닌다
```rust
fn main() {
    let container: [i32; 5] = [1, 2, 3, 4, 5];
    for item in container {
        println!("{}", item)
        // do something
    }
}
```
다만, 위와 같은 형태로 for문을 사용할 때 아래와 같은 사항을 주의해야 한다.  
> for문이 끝난 이후에, `container` 변수는 계속 지역 범위 내에는 존재하지만, 해당 `container`에 접근할 수 없다.  
> 추후 `container`에 접근하고 싶다면, 참조를 사용해서 for문을 실행시켜야 한다.

참조를 사용하여 for문을 실행시킨다는 것은 아래와 같은 형태를 가진다.
```rust
fn main() {
    let container: [i32; 5] = [1, 2, 3, 4, 5];
    // &를 사용하여 참조를 생성
    for item in &container {
        // do something
        println!("{}", item)
    }
}
```

또한, for문을 사용할 때, `container`의 값을 변경하고 싶다면, `mut`키워드를 사용하여 가변 참조로 사용해야 한다.  
```rust
fn main() {
    let mut container: [i32; 5] = [1, 2, 3, 4, 5];
    for item in &mut container {
        // do something
        println!("{}", item)
    }
}
```

또한 위와 같은 형식들은 메서드로도 표현 가능하며, 이를 정리하면 아래와 같다.
| 단축 형태 | 동등한 형태 | 접근 |
|:---:|:---:|:---:|
| `for item in collection` | `for item in IntoIterator::into_iter(collection)` | 소유권(ownership) |
| `for item in &collection` | `for item in collection.iter()` | 읽기 전용 |
| `for item in &mut collection` | `for item in collection.iter_mut()` | 읽고 쓰기 |


### 익명 반복문
for문 내에서 지역 변수를 사용하지 않는 경우 관례적으로 `_`를 사용하여 변수를 표현한다.(python과 동일)
```rust
fn main() {
    // 0..10 : 배제 범위(exclusive range), 값을 포함하지 않는 범위
    // 0..=10 : 포함 범위(inclusive range), 값을 포함하는 범위
    for _ in 0..10 {
        // do something
    }
}
```

### 인덱스 변수 관리를 피하는 법
통상적으로, 아래와 같이 for문으로 인해 증가하는 인덱스 변수를 사용하여 컨테이너의 값을 접근하는 경우가 있다.
```rust
fn main() {
    let collection = [1, 2, 3, 4, 5];
    // array.len() : 배열의 길이
    // 0..collection.len() : 배제 범위(exclusive range), 값을 포함하지 않는 범위
    for i in 0..collection.len() {
        println!("{}", collection[i]);
    }
}
```
위 예제는 Rust 문법에 맞으며, `for item in collection`구문으로 `collection`의 값을 직접 처리할 수 없을 때 필수적으로 사용된다.  
그러나 이는 아래와 같은 이유들로 Rust에서 일반적으로 권장되는 방식은 아니다.
- `collection[index]`구문을 사용하여 값을 인덱싱할 때 경계 확인으로 인한 런타임 비용이 발생한다. 즉, Rust는 `index`가 현재 `collection`에 유효한지 확인한다. 이러한 검사는 `collection`을 통해 직접 반복할 때는 필요하지 않다.
- 계속해서 주기적으로 `collection`에 접근하는 경우에는 `collection`이 변경될 가능성도 존재한다. Rust에서는 `collection`에 for 반복문을 직접 사용하면 `collection`이 프로그램의 다른 부분에 의해 변경되지 않은 상태로 유지되도록 보장할 수 있다.

## Continue
Python, C++과 동일하게 continue를 사용하여 반복문의 다음 반복으로 넘어갈 수 있다.
```rust
fn main() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}
```

## While
Rust에서의 while문은 기본적으로 아래와 같은 형태를 지닌다.
```rust
fn main() {
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
}
```