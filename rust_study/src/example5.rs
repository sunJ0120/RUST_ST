pub fn run(){
    let user1 = User {
        active: true,
        username: String::from("sunJ0120"),
        email: String::from("sspure1214@gmail.com"),
        sign_in_count: 1,
    };

    println!("username : {}", user1.username);

    let mut user2 = User {
        active: user1.active,
        username: String::from("sunClone0120"),
        email: String::from("sspureClone1214@gmail.com"),
        sign_in_count: 2,
    };
    user2.email = String::from("sspure123@gmail.com");
    println!("바뀐 user2 email : {}", user2.email);

    // 함수로 User 인스턴스 생성하기
    let user3_email = String::from("sspure123@naver.com");
    let user3_username = String::from("inskrrgram");

    let user3 = build_user(user3_email, user3_username);
    println!("user3 username : {}", user3.username);

    // user3의 일부 속값을 복사하여 user4 생성하기
    let user4 = User {
        username: String::from("inskrrgram2"),
        ..user3
    };
    println!("user4 sign_in_count : {}", user4.sign_in_count);
    println!("user3 username : {}", user3.username);
    println!("user3 sign_in_count : {}", user3.sign_in_count);
    // println!("user3 email : {}", user3.email);

    // 튜플 구조체 사용하기
    let song = MyFavoriteSongs(
        String::from("Ditto"),
        String::from("뚫고 지나가요")
    );

    let instrument = MyFavoriteInstrument(
        String::from("비올라"),
        String::from("베이스")
    );

    println!("내가 좋아하는 노래 1 : {}", song.0);
    println!("내가 좋아하는 악기 1 : {}", instrument.0);

    // print_my_favorite_instrument(song);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn print_my_favorite_instrument(instrument: MyFavoriteInstrument) {
    println!("내가 좋아하는 악기: {} {}", instrument.0, instrument.1);
}

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct MyFavoriteSongs(String, String);
struct MyFavoriteInstrument(String, String);