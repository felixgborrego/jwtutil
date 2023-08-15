use chrono::{DateTime, Utc};
use clap::{App, Arg};
use colored_json::to_colored_json_auto;
use jwt::{Header, Token, Unverified};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DummySignature {}
fn main() {
    let matches = App::new("JWT Decoder")
        .version("0.1.0")
        .author("ChatGPT")
        .about("Decodes JWT tokens into JSON")
        .arg(
            Arg::with_name("TOKEN")
                .help("JWT token to decode")
                .required(true)
                .index(1),
        )
        .get_matches();

    let token = matches.value_of("TOKEN").unwrap();
    display_jwt_as_json(token);
}

fn display_jwt_as_json(token: &str) {
    match decode_token_without_verification::<Value>(token) {
        Ok(data) => {
            let json_value = data.claims();
            let pretty_json = to_colored_json_auto(&json_value).unwrap();
            println!("{}", pretty_json);
            // let json = serde_json::to_string_pretty(data.claims()).unwrap();
            // println!("{}", json);
            // Extract "exp" and "iat" fields
            if let Some(exp) = json_value["exp"].as_i64() {
                let date =
                    DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(exp, 0), Utc);
                let result = relative_time_string(date);
                println!(" * exp[expiration] time: {}", result);
            }

            if let Some(iat) = json_value["iat"].as_i64() {
                let date =
                    DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(iat, 0), Utc);
                let result = relative_time_string(date);
                println!(" * iat time: {}", result);
            }
        }
        Err(e) => {
            eprintln!("Error decoding JWT: {}", e);
        }
    }
}

fn decode_token_without_verification<'a, T: DeserializeOwned>(
    token: &'a str,
) -> Result<Token<Header, T, Unverified<'a>>, jwt::Error> {
    Token::<Header, T, Unverified<'a>>::parse_unverified(token)
}
fn relative_time_string(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(date);

    match duration.num_minutes().cmp(&0) {
        Ordering::Less => {
            let minutes = duration.num_minutes().abs();
            if minutes < 60 {
                format!("{} minutes from now", minutes)
            } else if minutes < 60 * 24 {
                format!("{} hours from now", minutes / 60)
            } else {
                format!("{} days from now", minutes / (60 * 24))
            }
        }
        Ordering::Equal => "just now".to_string(),
        Ordering::Greater => {
            if duration.num_minutes() < 60 {
                format!("{} minutes ago", duration.num_minutes())
            } else if duration.num_minutes() < 60 * 24 {
                format!("{} hours ago", duration.num_minutes() / 60)
            } else {
                format!("{} days ago", duration.num_minutes() / (60 * 24))
            }
        }
    }
}
