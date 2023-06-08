use std::str::FromStr;
use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0
    {
        if m < n
        {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    n
}

// 실행 방법 : cargo run n m
// 문서 확인 방법 : rustup doc --std
fn main() {
    // mut로 선언하지 않으면, 값을 못 넣음;
    let mut numbers = Vec::new();

    // 1번 째 인수가 프로그램 이름이라 스킵
    for arg in env::args().skip(1)
    {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0
    {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // & : 레퍼런스 넘김
    for m in &numbers[1..]
    {
        // * : 메모리 역참조
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
    // rust에서는 expect나 std::process:exit 등의 명시적 함수 호출에 대해서만 
    // 오류 상태 코드를 가지고 종료하게 할 수 있음
}