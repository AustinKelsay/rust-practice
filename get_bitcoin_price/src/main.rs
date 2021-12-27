extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_market_cap=true&include_24hr_change=true").send().await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Move and borrow value of `res`
    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}
