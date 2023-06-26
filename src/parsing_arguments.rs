use std::str::FromStr;
use clap::{Parser, Args};
use chrono::{NaiveDate, ParseResult, Utc};
use email_address::EmailAddress;

/// A Program to create bills automatically
#[derive(Parser, Debug)]
pub struct TerminalArguments {
    /// date when the bill shall be written
    #[arg(long, default_value = "TODAY", value_parser = parse_date)]
    pub bill_date: NaiveDate,

    /// sender name
    #[arg(long)]
    pub sender_name: String,

    /// sender street and house number
    #[arg(long)]
    pub sender_address: String,

    /// sender zip code and city
    #[arg(long)]
    pub sender_zip_code_and_city: String,

    /// sender bank
    #[arg(long)]
    pub sender_bank: String,

    /// sender BIC
    #[arg(long)]
    pub sender_bic: String,

    /// sender IBAN
    #[arg(long)]
    pub sender_iban: String,

    /// sender tax number
    #[arg(long)]
    pub sender_tax_number: Option<String>,

    /// sender email
    #[arg(long, value_parser=check_email_address)]
    pub sender_email_address: Option<String>,

    /// sender phone number
    #[arg(long)]
    pub sender_phone_number: Option<String>,


    /// recipient name
    #[arg(long)]
    pub recipient_name: String,

    /// recipient street and house number
    #[arg(long)]
    pub recipient_address: String,

    /// recipient zip code
    #[arg(long)]
    pub recipient_zip_code_and_city: String,

    /// customer id
    #[arg(long, required=true)]
    pub customer_id: String,

    /// bill id
    #[arg(long, required=true)]
    pub bill_id: String,

    /// payment date
    #[arg(long, value_parser = parse_date, required=true)]
    pub payment_date: NaiveDate,

    /// output file name (.pdf)
    pub output_filename: String,

    /// list of products
    #[group(required=true)]
    pub products: Vec<Product>,
}

#[derive(Args, Debug, Clone)]
pub struct Product {
    /// description
    pub description: String,

    /// price
    pub price: String,
}

/// parse date string to NaiveDate
/// format: yyyy-mm-dd (ex.: 2023-06-19)
/// if parameter is "TODAY" it parses the current date
fn parse_date(arg: &str) -> ParseResult<NaiveDate> {
    if arg == "TODAY" {
        Ok(Utc::now().naive_utc().date())
    } else {
        NaiveDate::parse_from_str(arg, "%Y-%m-%d")
    }
}

fn check_email_address(arg: &str) -> Result<String, &'static str> {
    if EmailAddress::is_valid(arg) {
        Ok(arg.to_owned())
    } else {
        Err("given email addressed can't get parsed")
    }
}