use std::io;
use rand::Rng;

fn main() {
    println!("내가 무슨 번호를 생각했게~");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("(비밀 번호: {secret_number})");

    println!("너가 생각한 번호를 알려줘봐");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("값을 읽는데 실패했어");

    println!("너가 생각한 번호 : {guess}");
}
