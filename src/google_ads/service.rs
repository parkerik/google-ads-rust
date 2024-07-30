use crate::google_ads::schemas;

use anyhow::Result;
// use cached::proc_macro::cached;
use chrono::{Datelike, NaiveDate};
use dotenvy_macro::dotenv;
use reqwest;
use std::collections::HashMap;
// use async_recursion::async_recursion;
// use tokio_retry::strategy::FixedInterval;
// use tokio_retry::Retry;

// #[cached(time = 3600, result = true)]
async fn get_access_token() -> Result<String> {
    let mut body = HashMap::new();
    body.insert("grant_type", "refresh_token");
    body.insert("client_id", dotenv!("GOOGLE_ADS_CLIENT_ID"));
    body.insert("client_secret", dotenv!("GOOGLE_ADS_CLIENT_SECRET"));
    body.insert("refresh_token", dotenv!("GOOGLE_ADS_REFRESH_TOKEN"));

    let client = reqwest::Client::new();
    let response = client
        .post("https://www.googleapis.com/oauth2/v3/token")
        .json(&body)
        .send()
        .await?;

    println!("{:?}", response);

    Ok(response.json::<schemas::AuthToken>().await?.access_token)
}

async fn build_forecast_request(
    criteria_ids: Vec<u32>,
    keywords: Vec<String>,
    negatives: Vec<String>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    budget: f64,
) -> Result<schemas::ForecastBody> {
    let mut geo_locations = vec![];
    for criteria_id in criteria_ids.into_iter() {
        geo_locations.push(schemas::GeoModifiers {
            geo_target_constant: format!("geoTargetConstants/{}", criteria_id).to_string(),
        })
    }
    let mut bid_keywords = vec![];
    for kw in keywords.into_iter() {
        bid_keywords.push(schemas::BiddableKeywords {
            keyword: schemas::Keyword {
                match_type: "BROAD".to_string(),
                text: kw.to_string(),
            },
        })
    }
    let mut negative_keywords = vec![];
    for kw in negatives.into_iter() {
        negative_keywords.push(schemas::Keyword {
            match_type: "BROAD".to_string(),
            text: kw.to_string(),
        })
    }

    let period = (end_date.day() - start_date.day() + 1) as f64;
    let daily_budget = (budget / period * 100.0).round() as u64 * 10_000;

    Ok(schemas::ForecastBody {
        forecast_period: schemas::DateRange {
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
        },
        campaign: schemas::Campaign {
            language_constants: vec!["languageConstants/1000".to_string()],
            keyword_plan_network: "GOOGLE_SEARCH".to_string(),
            bidding_strategy: schemas::BiddingStrategy {
                maximize_conversions_bidding_strategy: schemas::DailyTargetSpendMicros {
                    daily_target_spend_micros: daily_budget,
                },
            },
            geo_modifiers: geo_locations,
            ad_groups: vec![schemas::AdGroups {
                biddable_keywords: bid_keywords,
            }],
            negative_keywords: negative_keywords,
        },
    })
}

// #[async_recursion]
pub async fn get_forecast_metrics(
    criteria_ids: Vec<u32>,
    keywords: Vec<String>,
    negatives: Vec<String>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    budget: f64,
) -> Result<schemas::ForecastMetrics> {
    println!("where is this failing?");
    let token = get_access_token().await?;
    println!("{}", token);
    println!("wheres the token?");

    let forecast_body = build_forecast_request(
        criteria_ids,
        keywords,
        negatives,
        start_date,
        end_date,
        budget,
    )
    .await;

    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://googleads.googleapis.com/{}/customers/{}:generateKeywordForecastMetrics",
            schemas::GOOGLE_ADS_VERSION,
            dotenv!("GOOGLE_ADS_CUSTOMER_ID")
        ))
        .header("Content-Type", "application/json")
        .header("developer-token", dotenv!("GOOGLE_ADS_DEVELOPER_TOKEN"))
        .header("login-customer-id", dotenv!("GOOGLE_ADS_LOGIN_CUSTOMER_ID"))
        .header("Authorization", format!("{}{}", "Bearer ", token))
        .json(&forecast_body?)
        .send()
        .await?
        .text()
        .await?;

    let forecast: schemas::ForecastMetrics = match serde_json::from_str(&response) {
        Ok(forecast) => forecast,
        Err(_) => {
            panic!("google ads error");
            // println!("retrying...");
            // let retry_strategy = FixedInterval::from_millis(0);
            // Retry::spawn(retry_strategy, || get_forecast_metrics(state, plan, group)).await?
        }
    };
    println!("{}", serde_json::to_string_pretty(&forecast).unwrap());
    Ok(forecast)
}
