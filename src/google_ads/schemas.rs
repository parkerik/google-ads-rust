use serde::{Deserialize, Serialize};

pub const GOOGLE_ADS_VERSION: &str = "v17";
pub const BID_MICROS: f64 = 1_000_000.;
pub const MAX_BUDGET: f64 = 1_000_000.;

#[derive(Deserialize)]
pub struct AuthToken {
    pub access_token: String,
}

#[derive(Serialize, Clone)]
pub struct MaxBudget {
    pub max_budget: f64,
}

#[derive(Deserialize, Clone)]
pub struct ForecastParams {
    pub criteria_ids: Vec<u32>,
    pub keywords: Vec<String>,
    pub negatives: Vec<String>,
    pub start_date: String,
    pub end_date: String,
    pub budget: f64,
}
#[derive(Deserialize, Clone)]
pub struct MaxBudgetParams {
    pub criteria_ids: Vec<u32>,
    pub keywords: Vec<String>,
    pub negatives: Vec<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignForecastMetrics {
    _impressions: f64,
    _click_through_rate: f64,
    _average_cpc_micros: String,
    _clicks: f64,
    pub cost_micros: String,
    pub conversions: f64,
    _conversion_rate: f64,
    _average_cpa_micros: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastMetrics {
    pub campaign_forecast_metrics: CampaignForecastMetrics,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BiddingStrategy {
    pub maximize_conversions_bidding_strategy: DailyTargetSpendMicros,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyTargetSpendMicros {
    pub daily_target_spend_micros: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoModifiers {
    pub geo_target_constant: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    pub match_type: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BiddableKeywords {
    pub keyword: Keyword,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdGroups {
    pub biddable_keywords: Vec<BiddableKeywords>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    pub language_constants: Vec<String>,
    pub keyword_plan_network: String,
    pub bidding_strategy: BiddingStrategy,
    pub geo_modifiers: Vec<GeoModifiers>,
    pub ad_groups: Vec<AdGroups>,
    pub negative_keywords: Vec<Keyword>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastBody {
    pub forecast_period: DateRange,
    pub campaign: Campaign,
}
