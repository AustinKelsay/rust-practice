extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("http://httpbin.org/get").send().await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Move and borrow value of `res`
    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}
