pub fn run(){
    let a_str = String::from("안녕하세요");
    function_scope(a_str.clone());    // clone() 메서드를 사용하여 복사본을 전달

    println!("함수 호출 후에는 이동으로 인해 scope 벗어나서 사용 불가 : {}", a_str);

    let mut b_str = String::from("반갑습니다");

    {
        let b2 = &mut b_str;
        rental_scope(b2);  // scope가 달라서 사용 가능
    }

    let b1 = &mut b_str;

    rental_scope(b1);  // 참조자를 전달
    println!("참조자로 전달했기 때문에 여전히 사용 가능: {}", b_str);

    let mut c_str = String::from("반갑습니다");
    let c1 = &c_str;   // 문제없음
    let c2 = &c_str;   // 문제없음
    // let c3 = &mut c_str;   // 문제!!!!

    // println!("{}, {}, and {}", c1, c2, c3);

    // 참조자 위치
    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);

    let r3 = &mut s; // 위에서 사용이 끝났기 때문에, 문제없음
    println!("{}", r3);

    // 댕글링 참조
    // let dangling_ref = dangle();
    let dangling_ref = no_dangle();
    println!("댕글링 참조 방지: {}", dangling_ref);

    // 공백으로 구분해서 첫 번째 단어를 반환하는 함수
    let my_string = String::from("hello world !");
    let word = first_word(&my_string);
    // println!("첫 번째 단어: {}", word);

    println!("첫 번째 단어: {}", word);
}

fn function_scope(a_str: String) -> String {
    println!("입력된 문자열: {}", a_str);
    a_str
}

fn rental_scope(b: &mut String) {
    b.push_str("😍😍");
    println!("대여한 문자열: {}", b);
}

// fn dangle () -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle () -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();    // String을 하나하나 쪼개서 해당 요소가 공백 값인지 확인해야 하므로, as_bytes 메서드를 이용해 바이트 배열로 변환

    for (i, &item) in bytes.iter().enumerate() {    // enumerate()로 감싸기, 인덱스와 참조자
        if item == b' ' {    // 참조자가 가리키는 값이 공백(b' ')인지 확인
            return &s[0..i];    // 공백 전까지 프린트
        }
    }

    &s[..]
}