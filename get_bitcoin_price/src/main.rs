extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_market_cap=true&include_24hr_change=true")
        .await?    
        .text()
        .await?;
    // If you wanted to print the status or headers
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    println!("\n{}\n", res);

    Ok(())
}
