use std::collections::HashMap;

pub fn run(){
    // score_example();
    // ownership_example();
    // hashmap_covering_example();
    // hashmap_covering_example2();
    hashmap_update_example();
}

fn score_example(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn ownership_example(){
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("소유권 체크하기 : {}", field_name);
}

fn hashmap_covering_example(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);    // 이 값이 기존 값을 덮어씀

    println!("{:?}", scores);
}

fn hashmap_covering_example2(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn hashmap_update_example(){
    let text = "안녕, 내이름은 피용히 탐정이다";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}