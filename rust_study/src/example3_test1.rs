use std::io;

// 화씨 온도와 섭씨 온도 간 변환하기
pub fn run(){
    println!("온도 변환기 예제");
    println!("1. 섭씨 -> 화씨");
    println!("2. 화씨 -> 섭씨");
    println!("변환 방식을 선택하세요.");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("값을 읽는데 실패했습니다.");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("유효한 숫자를 입력해주세요!");
            return;
        }
    };

    match choice {
        1 => celsius_fahrenheit_conversion(choice),
        2 => fahrenheit_celsius_conversion(choice),
        _ => {
            println!("유효한 선택이 아닙니다!");
            return;
        }
    }
}

fn celsius_fahrenheit_conversion(choice: u32) {
    println!("섭씨 온도를 입력하세요:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("값을 읽는데 실패했습니다.");

    // TODO : 변환
    let celsius: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("유효한 숫자를 입력해주세요!");
            return;
        }
    };
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("섭씨 {celsius}도는 화씨 {fahrenheit}도입니다.");
}

fn fahrenheit_celsius_conversion(choice: u32) {
    println!("화씨 온도를 입력하세요:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("값을 읽는데 실패했습니다.");

    let fahrenheit : f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("유효한 숫자를 입력해주세요!");
            return;
        }
    };
    let celsius =  (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("화씨 {fahrenheit}도는 섭씨{celsius}도 입니다.");
}