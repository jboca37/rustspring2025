use serde::Deserialize;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Define the Pricing trait
trait Pricing {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>>;
    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> &str;
}

// Struct for Bitcoin
struct Bitcoin {
    file_path: String,
}

// Struct for Ethereum
struct Ethereum {
    file_path: String,
}

// Struct for S&P 500
struct SP500 {
    file_path: String,
}

// Implement the Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>> {
        // Use CoinGecko API to fetch Bitcoin price
        let response = ureq::get(
            "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd",
        )
        .call()?
        .into_string()?;

        // Parse the JSON response
        #[derive(Deserialize)]
        struct CoinGeckoResponse {
            bitcoin: BitcoinPrice,
        }

        #[derive(Deserialize)]
        struct BitcoinPrice {
            usd: f64,
        }

        let price_data: CoinGeckoResponse = serde_json::from_str(&response)?;
        println!("Fetched Bitcoin price: ${:.2}", price_data.bitcoin.usd);

        Ok(price_data.bitcoin.usd)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut file = open_or_create_file(&self.file_path)?;

        writeln!(file, "{},{:.2}", timestamp, price)?;
        println!("Saved Bitcoin price to {}", self.file_path);

        Ok(())
    }

    fn name(&self) -> &str {
        "Bitcoin"
    }
}

// Implement the Pricing trait for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>> {
        // Using CoinGecko API to fetch Ethereum price
        let response = ureq::get(
            "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd",
        )
        .call()?
        .into_string()?;

        // Parse the JSON response
        #[derive(Deserialize)]
        struct CoinGeckoResponse {
            ethereum: EthereumPrice,
        }

        #[derive(Deserialize)]
        struct EthereumPrice {
            usd: f64,
        }

        let price_data: CoinGeckoResponse = serde_json::from_str(&response)?;
        println!("Fetched Ethereum price: ${:.2}", price_data.ethereum.usd);

        Ok(price_data.ethereum.usd)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut file = open_or_create_file(&self.file_path)?;

        writeln!(file, "{},{:.2}", timestamp, price)?;
        println!("Saved Ethereum price to {}", self.file_path);

        Ok(())
    }

    fn name(&self) -> &str {
        "Ethereum"
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>> {
        // Using Alpha Vantage API for S&P 500 data
        let response = ureq::get("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=SPY&apikey=Q1VTW6BNGWFHN1MI")
            .call()?
            .into_string()?;

        // Parse the JSON response
        #[derive(Deserialize)]
        struct AlphaVantageResponse {
            #[serde(rename = "Global Quote")]
            global_quote: GlobalQuote,
        }

        #[derive(Deserialize)]
        struct GlobalQuote {
            #[serde(rename = "05. price")]
            price: String,
        }

        let data: AlphaVantageResponse = serde_json::from_str(&response)?;
        let price = data.global_quote.price.parse::<f64>()?;
        println!("Fetched S&P 500 (SPY) price: ${:.2}", price);

        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut file = open_or_create_file(&self.file_path)?;

        writeln!(file, "{},{:.2}", timestamp, price)?;
        println!("Saved S&P 500 price to {}", self.file_path);

        Ok(())
    }

    fn name(&self) -> &str {
        "S&P 500"
    }
}

fn open_or_create_file(path: &str) -> Result<File, Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;

    Ok(file)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create instances of each asset
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin {
            file_path: "bitcoin_prices.csv".to_string(),
        }),
        Box::new(Ethereum {
            file_path: "ethereum_prices.csv".to_string(),
        }),
        Box::new(SP500 {
            file_path: "sp500_prices.csv".to_string(),
        }),
    ];

    println!("Financial Data Fetcher Started");
    println!("Fetching data every 10 seconds... Press Ctrl+C to stop");

    // Create or ensure header exists in each file
    for asset in &assets {
        let path = match asset.name() {
            "Bitcoin" => "bitcoin_prices.csv",
            "Ethereum" => "ethereum_prices.csv",
            "S&P 500" => "sp500_prices.csv",
            _ => continue,
        };

        // Check if file exists and is empty
        if !std::path::Path::new(path).exists() || std::fs::metadata(path)?.len() == 0 {
            let mut file = File::create(path)?;
            writeln!(file, "timestamp,price")?;
        }
    }

    // Main loop to fetch and save data
    loop {
        println!(
            "\n--- Fetching data at {} ---",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );

        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Error saving {} price: {}", asset.name(), e);
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching {} price: {}", asset.name(), e);
                }
            }
        }

        // Wait for 10 seconds before the next fetch
        println!("Waiting 10 seconds for next update...");
        thread::sleep(Duration::from_secs(10));
    }
}
