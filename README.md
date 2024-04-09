# Below is the example on how to use the lib:
```
use blipe_sdk;

#[tokio::main]
async fn main() {
    let prod = blipe_sdk::Producer::new(String::from("http://0.0.0.0:3000"));
    let response = prod.produce(20).await;
    let consumer = blipe_sdk::Consumer::new(String::from("http://0.0.0.0:3000"));
    let job = consumer.consume().await;
    println!("{:?}", job);
}
```
