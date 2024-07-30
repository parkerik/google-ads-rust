use crate::google_ads::{schemas, service};
use axum::{
    extract::Json,
    routing::{put, Router},
};
use chrono::NaiveDate;

pub fn router() -> Router {
    Router::new()
        .route("/keyword-planner/forecast-metrics", put(get_forecast))
        .route("/keyword-planner/max-budget", put(get_max_budget))
}

async fn get_forecast(
    Json(params): Json<schemas::ForecastParams>,
) -> Json<schemas::CampaignForecastMetrics> {
    Json(
        service::get_forecast_metrics(
            params.criteria_ids,
            params.keywords,
            params.negatives,
            NaiveDate::parse_from_str(&params.start_date, "%Y-%m-%d")
                .expect("can parse start_date"),
            NaiveDate::parse_from_str(&params.end_date, "%Y-%m-%d").expect("can parse end_date"),
            params.budget,
        )
        .await
        .expect("can forecast")
        .campaign_forecast_metrics,
    )
}

async fn get_max_budget(Json(params): Json<schemas::MaxBudgetParams>) -> Json<schemas::MaxBudget> {
    let forecast = service::get_forecast_metrics(
        params.criteria_ids,
        params.keywords,
        params.negatives,
        NaiveDate::parse_from_str(&params.start_date, "%Y-%m-%d").expect("can parse start_date"),
        NaiveDate::parse_from_str(&params.end_date, "%Y-%m-%d").expect("can parse end_date"),
        schemas::MAX_BUDGET,
    )
    .await
    .expect("can forecast");

    let max_budget: f64 = forecast
        .campaign_forecast_metrics
        .cost_micros
        .parse()
        .unwrap();

    Json(schemas::MaxBudget {
        max_budget: (max_budget / schemas::BID_MICROS).round(),
    })
}
