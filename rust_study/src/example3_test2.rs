use std::io;

// n번째 피보나치 수 생성하기
pub fn run() {
    println!("n번째 피보나치 수 생성기 예제");
    println!("몇 번째 피보나치 수를 원하시나요?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("값을 읽는데 실패했습니다.");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("유효한 숫자를 입력해주세요!");
            return;
        }
    };

    let result = fibonacci(n);
    println!("피보나치 수열의 {n}번째 수는 {result}입니다.");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}
