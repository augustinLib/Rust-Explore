# Data type
Rust의 Data type은 크게 아래와 같이 두 종류로 나뉜다.
- Scalar type
- Compound type

Rust는 **정적 타입 언어**이다. 즉, 컴파일 시점에 모든 변수의 타입이 결정되어야 한다.  
한 프로그램 내에서 여러 타입을 사용할 경우에는 type annotation을 통해 타입을 명시해줘야 한다.  


## Scalar type
Scalar type은 하나의 값으로 표현되는 타입이다. Rust는 다음과 같은 Scalar type을 지원한다.
- Integer
- Floating-point number
- Boolean
- Character


## Number
Integer와 Floating-point number는 Rust에서 숫자를 나타내는 두 가지 타입이다.

### Rust에서의 숫자 타입
- Rust는 정수와 부동소수점 숫자를 지원한다.
- C, C++와 같이 다양한 숫자 타입을 가지며, 이를 명시적으로 선언해줘야한다.  
- Rust에서는 타입 간 변환이 항상 명시적으로 일어나며, 16비트 정수를 32비트 정수로 자동 변환하지 않는다.
- Rust의 수는 메서드를 가질 수 있다. 예를 들어, 24.5를 반올림할때, `round(24.5_f32)` 와 같은 형태가 아닌, `24.5_f32.round()`를 사용한다.  

```Rust
fn main() {
    // let 키워드를 사용하여 변수를 선언
    // Rust에서는 타입이 명시되지 않은 경우 컴파일러가 타입을 추론
    let twenty = 20;

    // 타입 애너테이션을 사용하여 타입을 명시적으로 지정할 수 있음
    let twenty_one: i32 = 21;

    // 타입 접미사를 통해 타입을 명시적으로 지정할 수 있음
    // 숫자 리터럴 내에서 _를 사용할 수 있으며, 가독성 향상 이외에 다른 의미는 없음 (컴파일러는 이를 무시함)
    let twenty_two = 22_i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;

    // 숫자는 메서드를 가짐
    println!("{}", one_million.pow(2));

    // 배열은 모두 같은 타입이어야 하며 대괄호로 묶어서 생성함
    let forty_twos = [
        // 명시적인 타입 애너테이션이 없는 부동 소수점 리터럴은 상황에 따라 32비트 혹은 64비트가 됨
        42.0,

        // 부동 소수점 리터럴도 타입 접미사가 붙을 수 있음
        42f32,

        // 부동 소수점 리터럴과 타입 접미사 사이에 추가적인 밑줄도 쓸 수 있음
        42.0_f32,
    ];
    // 배열은 인덱스를 통해 접근할 수 있음
    println!("{:02}", forty_twos[0]);
}

```

### 2진수, 8진수, 16진수
Rust는 2진수, 8진수, 16진수를 지원한다.  
2진수는 `0b`로 시작하며, 8진수는 `0o`로 시작하며, 16진수는 `0x`로 시작한다.  
```Rust
fn main() {
    // 2진수, 8진수, 16진수 리터럴을 사용할 수 있음
    // 0b : 2진수, 0o : 8진수, 0x : 16진수
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
```
### 숫자 타입 종류
| type | description |
|:---:|:---:|
| `i8`, `i16`, `i32`, `i64`, `i128` | 8비트에서 128비트 크기의 부호 있는 정수 |
| `u8`, `u16`, `u32`, `u64`, `u128` | 8비트에서 128비트 크기의 부호 없는 정수 |
|`f32`, `f64`|32비트, 64비트 부동 소수점 수|
|`isize`, `usize`|CPU의 네이티브 크기를 따르는 부호가 있는, 없는 정수. 예를 들어 64비트 CPU에서는 64비트|

- 부호 없는 정수는 양의 정수만 나타낼 수 있지만, 부호 있는 정수에 비해 2배 큰 수를 나타낼 수 있다. (C와 C++의 `unsigned`와 같음)
- 부동 소수점 수는 `f32`와 `f64`가 있으며, `f64`가 더 정밀한 값을 나타낼 수 있다. (C와 C++의 `float`와 `double`과 같음)

### 수의 비교
Rust에서의 비교 연산은 트레이트(trait)를 통해 제공됨.  
| operator | description | example | 
|:---:|:---:|:---:|
| < | 미만, ~보다 작다 | `1.0 < 2.0` |
| <= | 이하, 같거나 작다 | `1.0 <= 2.0` |
| > | 초과, ~보다 크다 | `2.0 > 1.0` |
| >= | 이상, 같거나 크다 | `2.0 >= 1.0` |
| == | 같다 | `1.0 == 1.0` |
| != | 같지 않다 | `2.0 != 1.0` |

이러한 비교 연산시 주의해야할 점이 있다.
- 서로 다른 데이터타입은 비교할 수 없다 (정수, 실수를 넘어 `i32`와 `u16`도 안됨)
- 따라서 형변환을 통해 타입을 맞춰준 뒤 비교해야 한다.
- 형변환은 `as` 키워드를 통해 가능하다. (보통 작은 타입을 큰 타입으로 "승격"시키는 것이 안전함)
```Rust
fn main() {
  let a: i32 = 10;
  let b: u32 = 20;

  // Rust에서는 타입이 다른 변수 간의 비교를 허용하지 않음
  // 따라서 형변환을 통해 타입을 맞춰줘야 함
  if a < (b as i32) {
     println!("Ten is less than twenty!")
    }
}  
```

혹은 아래와 같은 방식으로도 실행 가능하다
```Rust
// use키워드를 통해 std::convert::TryInto 트레이트를 지역 범위로 가져옴
use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u32 = 20;
    
    // b.try_into()는 Result 안에 i32값을 감싸 반환함
    // Result는 성공값 또는 오륫값을 포함할 수 있음
    // unwrap() 메서드는 성공값을 처리하며, b의 값을 i32로 반환
    // u32에서 i32로 변환에 실패할 경우 unwrap()이 호출되고 프로그램이 중단됨
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than twenty!")
    }
}
```

### 유리수, 복소수, 기타 숫자 타입
Rust의 표준 라이브러리는 비교적 작으며, 다른 언어에서 자주 사용하는 몇 가지의 숫자 타입은 빠져 있다. 이는 아래와 같다.  
- 유리수 및 복소수를 다루는 데 사용하는 많은 수학적 객체
- 매우 크거나 매우 작은 수를 표현할 수 있는 임의의 크기를 가지는 정수와 임의의 정밀도롤 가지는 부동 소수점 수
- 화폐 단위에 쓰이는 고정 소수점 수

이러한 숫자 타입들을 사용하기 위해서는 `num`크레이트를 사용해야 한다.  
> 크레이트(crate)란 Rust의 패키지 시스템의 단위이다.  
> `https://crates.io`에서 공유되며, cargo를 통해 관리된다.  

`num` 크레이트를 사용하기 위해서는 `Cargo.toml`에 아래와 같이 추가해야 한다.
```toml
[dependencies]
num = "0.4.0"
```

혹은 `cargo add` 명령을 활성화하는 `cargo-edit`크레이트를 설치한 뒤 `cargo add`로 추가해줄수 있다.  
**`cargo-edit`크레이트 설치**
```bash
cargo install cargo-edit
```
**`cargo add`로 크레이트 추가**
```bash
cargo add num
```
## Boolean
Rust의 boolean 타입은 `bool`이며, `true`와 `false` 두 가지 값만 가질 수 있다.  
추가적으로, boolean 타입은 1바이트 크기이다.  
```Rust
fn main() {
    let t = true;
    let f: bool = false; // 명시적 타입 지정

    println!("t = {}, f = {}", t, f);
}
```

## Character
Rust의 문자 타입은 `char`이며, 4바이트 크기이다.  
(Rust는 유니코드를 사용하며, 유니코드는 4바이트 크기이다.)  
또한 Rust에서는 문자열을 표현하기 위해 큰따옴표(`"`)를 사용하며, 문자를 표현하기 위해 작은따옴표(`'`)를 사용한다.  
(C++과 동일)
```Rust
fn main() {
    let c = 'c';
    println!("c = {}", c)
}
```

## Compound Types
Compound Type은 하나의 타입으로 여러 개의 값을 그룹화한 타입이다.  
Rust에서의 Compound Type은 아래와 같다.
- Tuple
- Array


## Tuple
Tuple은 **서로 다른 타입의 여러 값**을 하나의 그룹으로 묶어서 표현하는 타입이다.  
Tuple은 **고정된 길이를 가지며, 한 번 생성되면 길이를 변경할 수 없다.**

Tuple은 아래와 같이 소괄호로 값의 목록을 쉼표로 구분하여 선언한다. (python과 동일)
```Rust
fn main() {
    // tuple의 선언 및 값 할당
    // 소괄호로 값을 묶어서 선언하며, 각각의 값은 콤마로 구분함
    // annotation의 경우에도 동일하게 적용됨
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
}
```

또한, python과 비슷하게 tuple의 개별 값을 아래와 같이 해체할 수 있다
```Rust
fn main() {
    let tup = (500, 6.4, 1);
    // tuple의 값을 해체하여 변수에 할당
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

추가적으로, 아래와 같이 `.` 다음에 인덱스를 지정하여 tuple의 각 요소를 직접 참조할 수 있다.
```Rust
fn main() {
    let tup = (500, 6.4, 1);
    // tuple의 값을 참조하여 변수에 할당
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

## Array
Tuple과는 다르게, Array는 **동일한 타입의 여러 값**을 하나의 그룹으로 묶어서 표현하는 타입이다.  
Rust에서의 Array는 고정 길이를 가진다.  
Array는 아래와 같이 대괄호로 값의 목록을 쉼표로 구분하여 선언한다. 
```Rust
fn main() {
    // annotation은 [타입, 길이] 와 같은 형식으로 작성함
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
}
```
> C++과 동일하게, array는 고정 길이를 가지고, vector가 가변 길이를 가진다.  

`.` 다음에 인덱스를 지정하여 각 요소에 접근했던 tuple과는 다르게, array는 `[]` 다음에 인덱스를 지정하여 각 요소에 접근한다.
```Rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 배열의 요소는 인덱스를 통해 접근할 수 있음
    println!("a[0] = {}", a[0]);
}
```

