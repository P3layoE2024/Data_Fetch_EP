use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use ureq;

// Pricing trait for structs
trait Pricing {
    fn get_url() -> &'static str;
    fn extract_price(&self) -> f64;
    // Retrieves data from API
    fn fetch_price() -> std::io::Result<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        let response = ureq::get(Self::get_url())
            .call()
            .expect("HTTP request failed");
        response.into_json()
    }
    // Saves price to text file
    fn save_price(&self, filename: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open file");
        writeln!(file, "{}", self.extract_price()).expect("Failed to write to file");
    }
}

// Structs for CoinGecko response
#[derive(Deserialize, Debug)]
struct CoinGeckoPrice {
    bitcoin: Option<CryptoPrice>,
    ethereum: Option<CryptoPrice>,
}

#[derive(Deserialize, Debug)]
struct CryptoPrice {
    usd: f64,
}

// Structs for individual assets
#[derive(Deserialize, Debug)]
struct Bitcoin {
    #[serde(flatten)]
    data: CoinGeckoPrice,
}

#[derive(Deserialize, Debug)]
struct Ethereum {
    #[serde(flatten)]
    data: CoinGeckoPrice,
}

// Implement Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn get_url() -> &'static str {
        "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"
    }
    fn extract_price(&self) -> f64 {
        self.data.bitcoin.as_ref().map(|b| b.usd).unwrap_or(0.0)
    }
}

// Pricing trait for Ethereum struct
impl Pricing for Ethereum {
    fn get_url() -> &'static str {
        "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd"
    }
    fn extract_price(&self) -> f64 {
        self.data.ethereum.as_ref().map(|e| e.usd).unwrap_or(0.0)
    }
}

fn main() {
    loop {
        // Retireve Bitcoin from API
        if let Ok(bitcoin) = Bitcoin::fetch_price() {
            bitcoin.save_price("bitcoin.txt");
        }
        else{/*error*/}
        // REtireve Etheruem from API
        if let Ok(ethereum) = Ethereum::fetch_price() {
            ethereum.save_price("ethereum.txt");
        }
        else{/*error*/}
        // Wait for 10 agonizing seconds
        thread::sleep(Duration::from_secs(60));
    }
}
