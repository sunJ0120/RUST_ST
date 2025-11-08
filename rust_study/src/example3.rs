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
}
