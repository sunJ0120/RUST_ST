pub fn run(){
    let a_str = String::from("안녕하세요");
    function_scope(a_str.clone());    // clone() 메서드를 사용하여 복사본을 전달

    println!("함수 호출 후에는 이동으로 인해 scope 벗어나서 사용 불가 : {}", a_str);
}

fn function_scope(a_str: String) -> String {
    println!("입력된 문자열: {}", a_str);
    a_str
}