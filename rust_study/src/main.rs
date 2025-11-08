use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("내가 무슨 번호를 생각했게~");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("너가 생각한 번호를 알려줘봐");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("값을 읽는데 실패했어");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("유효한 숫자를 입력해줘!");
                continue;
            }
        };

        println!("너가 생각한 번호 : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너가 생각한 번호가 너무 작아!"),
            Ordering::Greater => println!("너가 생각한 번호가 너무 커!"),
            Ordering::Equal => {
                println!("너가 생각한 번호가 맞아! 축하해!");
                break;
            }
        }
    }
}
