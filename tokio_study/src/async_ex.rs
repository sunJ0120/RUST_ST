use std::time::Instant;
use tokio::time::{sleep, Duration};

pub async fn run(){
    println!("Hello");
    let start = Instant::now();

    println!("[{:.2}초] start", start.elapsed().as_secs_f32());
    tokio::join!(say_name(), say_world());  // 동시 실행
    println!("[{:.2}초] done", start.elapsed().as_secs_f32());
}

async fn say_world(){
    sleep(Duration::from_secs(2)).await;  // 2초 대기
    println!("world");
}

async fn say_name(){
    sleep(Duration::from_secs(1)).await;  // 1초 대기
    println!("sunJ");
}
