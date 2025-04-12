# 🪙 **Financial Data Fetcher in Rust** 📊

## 📌 **Overview**
This project is a Rust application that periodically fetches and records the pricing data of **Bitcoin**, **Ethereum**, and the **S&P 500 index**. The application:
- Defines three structs: `Bitcoin`, `Ethereum`, and `SP500`.
- Implements a `Pricing` trait to standardize the fetching and saving of price data.
- Uses the `ureq` library to make HTTP requests and `serde` for JSON parsing.
- Fetches data every 10 seconds and saves it to separate files for each asset.

## 🌟 **Features**
- Fetches real-time data for **Bitcoin**, **Ethereum**, and **S&P 500** from public APIs. 📡
- Saves the fetched pricing data to **CSV files** with timestamps. 📅💾
- Periodically fetches and saves the data every **10 seconds**. ⏰
- Written in **Rust** with structured error handling. 🦀

## ⚙️ **Requirements**
Before running the program, ensure you have the following:

1. **Rust**: Install Rust by following the official guide: [Get Started with Rust](https://www.rust-lang.org/learn/get-started). 🦀
   
2. **Dependencies**:
   - `ureq`: For making HTTP requests 🌐
   - `serde`: For deserializing JSON responses 📜
   - `chrono`: For formatting timestamps 🕰️

### To install the required dependencies, run:

```bash
cargo build
```

## 🚀 **How to Run the Program**
1. **Clone the repository** to your local machine:

2. **Build the project** (if you haven't already):

   ```bash
   cargo build
   ```

3. **Run the program** using:

   ```bash
   cargo run
   ```

The program will start fetching the prices for **Bitcoin**, **Ethereum**, and the **S&P 500** index every 10 seconds and save the results to separate **CSV files**. It will run indefinitely until manually stopped (e.g., using `Ctrl+C`).

### Example Output

```bash
Financial Data Fetcher Started
Fetching data every 10 seconds... Press Ctrl+C to stop

--- Fetching data at 2025-04-11 12:34:56 ---
Fetched Bitcoin price: $54500.00
Saved Bitcoin price to bitcoin_prices.csv
Fetched Ethereum price: $3800.50
Saved Ethereum price to ethereum_prices.csv
Fetched S&P 500 (SPY) price: $420.20
Saved S&P 500 price to sp500_prices.csv
Waiting 10 seconds for next update...
```

## 📂 **File Format**
The pricing data is saved in **CSV format** with the following columns:
- `timestamp`: The **Unix timestamp** of when the price was fetched.
- `price`: The **price** of the asset (in USD).

Example of the file content:
```csv
timestamp,price
1618410000,54500.00
1618410600,55000.00
```

Files are saved in the same directory where the program is run:
- `bitcoin_prices.csv` 📉
- `ethereum_prices.csv` 📈
- `sp500_prices.csv` 🏦

## 🛠️ **Error Handling**
The program handles errors related to:
- Network issues while fetching data from the APIs 🌍🔌.
- File issues while saving data to the CSV files 📝.

If any errors occur during data fetching or saving, they will be printed to the console. ⚠️

## 🛑 **Termination**
To stop the program, press `Ctrl+C` in the terminal. The program will exit gracefully. ✋

