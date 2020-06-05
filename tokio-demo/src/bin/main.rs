extern crate tokio_demo;
use tokio_demo::echo;

#[tokio::main]
async fn main(){
    echo::echo().await;
}
