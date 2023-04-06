use exitfailure::ExitFailure;
use reqwest::{Url, Client};
use serde_derive::{Deserialize, Serialize};
use std::env;
use tokio::time::{sleep, Duration};
use dotenv::dotenv;

struct App {
    api_key: String,
    client: Client,
    quote_history: Vec<CompanyQuote>
}


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
    async fn get(symbol: &String, config: &mut App) -> Result<(), ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, config.api_key
        );

        let url = Url::parse(&url)?;
        let res = config.client.get(url).send().await?.json::<CompanyQuote>().await?;
        config.quote_history.push(res);

        Ok(())
    }
}

fn init_config() -> App {
    dotenv().ok();
    App {
        api_key: std::env::var("API_KEY").expect("API key must be set in .env").to_string(),
        client: reqwest::Client::new(),
        quote_history: vec![]
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let mut config = init_config();
    let args: Vec<String> = env::args().collect();
    let symbol: String = args.get(1).unwrap_or(&"AAPL".to_string()).to_string();


    loop {
        CompanyQuote::get(&symbol, &mut config).await?;
        let latest_quote = config.quote_history.last().unwrap();
        println!("{}'s current stock price: {} at time {}", symbol, latest_quote.c, latest_quote.t);
        sleep(Duration::from_millis(5_000)).await;
    }
}
