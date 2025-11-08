pub fn run() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Characters: {c}, {z}, {heart_eyed_cat}");

    // 복합 타입
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of six_point_four is: {six_point_four}");

    // 배열
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The first element is: {first}");

    // 함수
    another_function(5);

    // 구문과 표현식
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // 반환 값을 갖는 함수
    let z = five();
    println!("The value of z is: {z}");

    // 제어 흐름문_if
    if_function();

    // 제어 흐름문_loop
    loop_function();
    loop_function2();

    // 제어 흐름문_while
    while_function();

    // 제어 흐름문_for
    for_function();
}

fn another_function(x: i32) {
    println!("Another function! x value is: {x}");
}

fn five() -> i32 {
    5
}

fn if_function() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn loop_function() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
}

fn loop_function2() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("The result is: {count}");
}

fn while_function() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_function() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}