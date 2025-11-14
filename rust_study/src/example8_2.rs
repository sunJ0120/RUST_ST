pub fn run(){
    string_basic_ex();
    string_add_ex();
    string_index_ex();
    string_slice_ex();
}

fn string_basic_ex(){
    let mut s = String::new();

    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
}

fn string_add_ex(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1이 더 이상 유효하지 않다. 소유권을 가져왔기 때문에
    println!("{}", s3);
    println!("s2는 유효하다. {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}

fn string_index_ex(){
    let s1 = String::from("hello");
    // let h = s1[0]; // 오류 발생: Rust에서는 문자열을 인덱스로 접근할 수 없음

    let hello = String::from("Hola");
    let hello_2 = String::from("Здравствуйте");

    println!("{} 문자 개수: {}", hello, hello.len());
    println!("{} 문자 개수: {}", hello_2, hello_2.len());
}

fn string_slice_ex(){
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("슬라이스된 문자열 예상은 Зд.. 결과는? {}", s);

    for c in hello.chars() {
        println!("{c}");
    }

    for b in "Зд".bytes(){
        println!("{b}");
    }
}
