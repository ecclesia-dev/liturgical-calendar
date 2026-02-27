use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use calendar_core::{Calendar, LiturgicalDay};
use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Shared calendar cache (year -> Calendar)
pub struct AppState {
    cache: Mutex<HashMap<i32, Calendar>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn get_calendar(&self, year: i32) -> Calendar {
        let mut cache = self.cache.lock().unwrap();
        cache.entry(year).or_insert_with(|| Calendar::new(year));
        Calendar::new(year)
    }
}

#[derive(Deserialize)]
pub struct TodayQuery {
    tz: Option<String>,
}

#[derive(Serialize)]
pub struct SeasonResponse {
    date: NaiveDate,
    season: calendar_core::LiturgicalSeason,
    season_name: String,
    week: u8,
    color: calendar_core::LiturgicalColor,
}

fn bad_request(code: &str, msg: String) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": code, "message": msg})))
}

fn not_found(msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "not_found", "message": msg})))
}

fn resolve_today(tz_str: &str) -> Result<NaiveDate, (StatusCode, Json<serde_json::Value>)> {
    match tz_str.parse::<chrono_tz::Tz>() {
        Ok(tz) => Ok(Utc::now().with_timezone(&tz).date_naive()),
        Err(_) => Err(bad_request("invalid_timezone", format!("Unknown timezone: {}", tz_str))),
    }
}

fn season_display_name(s: calendar_core::LiturgicalSeason) -> &'static str {
    match s {
        calendar_core::LiturgicalSeason::Advent => "Advent",
        calendar_core::LiturgicalSeason::Christmas => "Christmastide",
        calendar_core::LiturgicalSeason::AfterEpiphany => "Time after Epiphany",
        calendar_core::LiturgicalSeason::Septuagesima => "Septuagesima",
        calendar_core::LiturgicalSeason::Lent => "Lent",
        calendar_core::LiturgicalSeason::Passiontide => "Passiontide",
        calendar_core::LiturgicalSeason::HolyWeek => "Holy Week",
        calendar_core::LiturgicalSeason::Easter => "Eastertide",
        calendar_core::LiturgicalSeason::Ascensiontide => "Ascensiontide",
        calendar_core::LiturgicalSeason::AfterPentecost => "Time after Pentecost",
    }
}

pub fn create_router() -> Router {
    let state = std::sync::Arc::new(AppState::new());

    Router::new()
        // GET /today
        .route("/today", get({
            let state = state.clone();
            move |query: Query<TodayQuery>| async move {
                let today = resolve_today(query.tz.as_deref().unwrap_or("UTC"))?;
                let cal = state.get_calendar(today.year());
                match cal.get(today) {
                    Some(day) => Ok(Json(serde_json::to_value(day).unwrap())),
                    None => Err(not_found("Date not in calendar")),
                }
            }
        }))
        // GET /date/{YYYY-MM-DD}
        .route("/date/{date}", get({
            let state = state.clone();
            move |Path(date_str): Path<String>| async move {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .map_err(|_| bad_request("invalid_date", "Expected YYYY-MM-DD format".into()))?;
                let cal = state.get_calendar(date.year());
                match cal.get(date) {
                    Some(day) => Ok(Json(serde_json::to_value(day).unwrap())),
                    None => Err(not_found("Date not in calendar")),
                }
            }
        }))
        // GET /month/{YYYY-MM}
        .route("/month/{year_month}", get({
            let state = state.clone();
            move |Path(ym): Path<String>| async move {
                let parts: Vec<&str> = ym.split('-').collect();
                if parts.len() != 2 {
                    return Err(bad_request("invalid_format", "Expected YYYY-MM format".into()));
                }
                let year: i32 = parts[0].parse().map_err(|_| bad_request("invalid_year", "Expected numeric year".into()))?;
                let month: u32 = parts[1].parse().map_err(|_| bad_request("invalid_month", "Expected numeric month".into()))?;
                if !(1..=12).contains(&month) {
                    return Err(bad_request("invalid_month", "Month must be 1-12".into()));
                }
                let cal = state.get_calendar(year);
                let mut days: Vec<&LiturgicalDay> = cal
                    .days()
                    .values()
                    .filter(|d| d.date.month() == month)
                    .collect();
                days.sort_by_key(|d| d.date);
                Ok(Json(serde_json::to_value(&days).unwrap()))
            }
        }))
        // GET /season — current liturgical season
        .route("/season", get({
            let state = state.clone();
            move |query: Query<TodayQuery>| async move {
                let today = resolve_today(query.tz.as_deref().unwrap_or("UTC"))?;
                let cal = state.get_calendar(today.year());
                match cal.get(today) {
                    Some(day) => {
                        let resp = SeasonResponse {
                            date: today,
                            season: day.season,
                            season_name: season_display_name(day.season).to_string(),
                            week: day.week,
                            color: day.color,
                        };
                        Ok(Json(serde_json::to_value(&resp).unwrap()))
                    }
                    None => Err(not_found("Date not in calendar")),
                }
            }
        }))
}

/// Start the API server on the given port.
pub async fn serve(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Liturgical Calendar API listening on port {}", port);
    axum::serve(listener, app).await?;
    Ok(())
}
