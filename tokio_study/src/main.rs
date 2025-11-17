mod async_ex;
mod mini_redis_server_ex_1;
mod spawning;

#[tokio::main]
async fn main() {
    // mini_redis_server_ex_1::run().unwrap();
    // async_ex::run().await; //future만 반환하는 lazy execution이라 .await 필수
    spawning::run().await;
}