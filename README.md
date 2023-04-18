# Rustnance
[![Downloads](https://img.shields.io/crates/d/rustnance.svg?style=flat-square)](https://crates.io/crates/rustnance/)
[![Version](https://img.shields.io/crates/v/rustnance.svg?style=flat-square)](https://crates.io/crates/rustnance/)
[![License](https://img.shields.io/crates/l/rustnance.svg?style=flat-square)](https://crates.io/crates/rustnance/)


## Synopsis
A crate that provides functionality within the space of financial analysis

## Explanation
This crate is meant to provide calculations as well as metrics and should not be seen as any financial or investing advice. The intent is to firstly develop calculation for fundamental analysis but technical analysis related functionality may be added. Some of the existing functions you will find consist of: 

* Calculate intrinsic value
* Calculate compound interest

## Usage
Inside your Cargo.toml
```toml
[dependencies]
rustnance = "0.2.1" # Latest version
```
Example usage
```rust
use rustnance::value; // Bring module into scope
fn main() {
    let free_cash_flow: Vec<f32> = vec![1000.0, 2000.0, 3000.0, 4000.0, 5000.0]; // Historical free cash flow
    let expected_return: f32 = 0.15; // The return you expect to achieve from an investment
    let outstanding_shares: f32 = 1_000.0; // Amount of shares that excist for the specific company
    let margin_of_safety: f32 = 0.3; // A margin of safety to take into account for the uncertainty of any calculation
    
    let intrinsic_value: f32 = value::calculate_intrinsic_value(&free_cash_flow, &expected_return);
    let intrinsic_value_per_share: f32 = value::intrinsic_value_per_stock(&intrinsic_value, &outstanding_shares);
    let share_buy_price: f32 = value::margin_of_safety(&intrinsic_value_per_share, &margin_of_safety);
    println!("A reasonable price would be: {}", share_buy_price);
}
```

## Module structure
![Output of `cargo modules generate tree`](docs/module_structure.png)

## Find a bug?
If you found a bug, an issue, an improvement or a potential addition to this project, please submit an issue(if it hasn't already been raised) using the issue tab towards the top. I intend to fix issues as they arise as well as add requested functionality.
