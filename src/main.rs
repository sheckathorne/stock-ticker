use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
use tokio::time::{sleep, Duration};
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct CompanyQuote {
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: u32,
}

impl CompanyQuote {
    async fn get(symbol: &String, api_key: &String, quote_history: &mut Vec<CompanyQuote>) -> Result<(), ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;
        quote_history.push(res);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API key must be set in .env").to_string();
    let args: Vec<String> = env::args().collect();
    let symbol: String = args.get(1).unwrap_or(&"AMC".to_string()).to_string();
    let mut quote_history: Vec<CompanyQuote> = vec![];

    loop {
        CompanyQuote::get(&symbol, &api_key, &mut quote_history).await?;
        let latest_quote = quote_history.last().unwrap();
        println!("{}'s current stock price: {} at time {}", symbol, latest_quote.c, latest_quote.t);
        sleep(Duration::from_millis(5_000)).await;
    }
}
