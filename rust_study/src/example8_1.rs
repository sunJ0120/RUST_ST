pub fn run(){
    vector_basic_ex();
    vector_get_and_index_ex();
    vector_push_ex();
    vector_for_ex();
    vector_enum_ex();
    vector_lifetime_ex();
}

fn vector_basic_ex(){
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn vector_get_and_index_ex(){
    let v = vec![1,2,3,4,5];

    let third = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn vector_push_ex(){
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6);
    let second = &v[0];    // 재 할당 후 가져오는 것은 가능

    // println!("The first element is: {}", first);
    println!("The second element is: {}", second);
}

fn vector_for_ex(){
    let v = vec![100,32,57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100,32,57];
    for i in &mut v {
        println!("주솟값이 나오는지 확인해보자. {:p}", i);
        *i += 50;
        println!("실제값이 나오는지 확인해보자. {}", *i);
    }
}

fn vector_enum_ex(){
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int value: {}", value),
            SpreadsheetCell::Float(value) => println!("Float value: {}", value),
            SpreadsheetCell::Text(value) => println!("Text value: {}", value),
        }
    }
}

fn vector_lifetime_ex(){
    {
        let v = vec![1, 2, 3, 4, 5];
    }
    // println!("벡터가 스코프를 벗어나 메모리에서 해제되었음 : {:?}", &v[0]);
}