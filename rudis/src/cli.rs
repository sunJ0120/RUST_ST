use std::io::{self, Write};
use rudis::Store;
use rudis::Command;

fn main(){
    let store = Store::new();

    println!("🦀 Rudis CLI v0.1.0");
    println!("종료를 원하시면 'EXIT'를 눌러주세요.");
    println!();

    loop{
        // 기본 프롬프터 출력
        print!("rudis> ");
        io::stdout().flush().unwrap();

        // 입력 받기
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("입력 하신 것을 읽는데 실패하였습니다. : {}", e);
                continue;
            }
        }

        let input = input.as_str().trim();    // 빈칸 무시
        if input.is_empty() {
            continue;
        }

        // Command 파싱
        let command = match Command::parse(input){
            Ok(cmd) => {cmd},
            Err(e) => {
                println!("파싱에 실패하였습니다. : {}", e);
                continue;
            }
        };

        // Exit
        if matches!(command, Command::Exit) {
            println!("{}", command.execute(&store));
            break;
        }

        // Command 실행
        let result = command.execute(&store);
        println!("{}", result);
    }
}