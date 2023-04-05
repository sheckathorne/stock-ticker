use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
use tokio::time::{sleep, Duration};
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct CompanyQuote {
    symbol: String,
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: u32,
}

struct QuoteHistory {
    symbol: String,
    price: f64,
    timestamp: u32
}

impl CompanyQuote {
    async fn get(symbol: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }
}

impl QuoteHistory {
    fn new(quote: CompanyQuote, quote_history: &mut Vec<QuoteHistory>) {
        let qh: QuoteHistory = QuoteHistory {
            symbol: quote.symbol.to_string(),
            price: quote.c,
            timestamp: quote.t,
        };

        quote_history.push(qh);
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API key must be set in .env").to_string();
    let args: Vec<String> = env::args().collect();
    let symbol: String = args.get(1).unwrap_or(&"AMC".to_string()).to_string();
    let mut quote_history: Vec<QuoteHistory> = vec![];

    loop {
        let res = CompanyQuote::get(&symbol, &api_key).await?;
        QuoteHistory::new(res, &mut quote_history);
        println!("{}'s current stock price: {} at time {}", symbol, res.c, res.t);
        sleep(Duration::from_millis(10_000)).await;
    }
}
