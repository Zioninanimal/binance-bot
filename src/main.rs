// Imports

#[derive(Debug)] 
enum Error {
  Api(reqwest::Error),
  Net(String),
  Order(String)  
}

use reqwest::Client; 
use serde_json::json;

const BUY_THRESHOLD: f64 = 19_000.0;
const SELL_THRESHOLD: f64 = 21_000.0;

struct Bot {
  client: Client,
  api_key: String,
  api_secret: String,  
  holdings: f64  
}

impl Bot {

  fn new(api_key: String, api_secret: String) -> Self {
    let client = Client::new();   
    Bot {
      client,
      api_key,
      api_secret, 
      holdings: 0.0  
    }
  }

  async fn get_price(&self) -> f64 {
    // Make API request to get BTC price
    // Parse response to get latest price
  }

  async fn buy(&mut self, amount: f64) -> Result<(), reqwest::Error> {
    // Generate signature
    let params = json!({
       "symbol": "BTCUSDT",  
       "side": "BUY",
       "type": "MARKET",
       "quantity": amount,
       "timestamp": unix_timestamp()  
    });
    
    let signature = sign_request(params.to_string(), self.api_secret); 

    // Make POST request to create order
    let resp = self.client.post("https://api.binance.com/api/v3/order")
      .json(&params)
      .header("X-MBX-APIKEY", self.api_key)
      .send()
      .await?;

    // Handle any errors  
    if !resp.status().is_success() {
       // Parse error message
       return Err(reqwest::Error::Status(resp.status()));
    }

    // Update holdings if successful
    self.holdings += amount;

    Ok(())
  }

  // sell() method

  // sign_request() helper method  
}

#[tokio::main]
async fn main() {
  let bot = Bot::new("abc123".into(), "xyz789".into());
  
  loop {
    let price = bot.get_price().await;

    if price < 10000.0 {
      bot.buy(0.1).await; 
    } else if price > 11000.0 {
      bot.sell(0.05).await;
    }

    sleep(60000);
  }
}
