/// This module provides weather data structures and mock data for the weather forecast API.
use serde::Serialize;

/// A struct representing a single day's weather forecast.
#[derive(Serialize)]
pub struct WeatherForecast {
    /// The place name for the weather forecast.
    pub place: String,
    /// The date of the weather forecast in the format "YYYY-MM-DD".
    pub date: String,
    /// A description of the weather for the forecasted date.
    pub weather: String,
}

/// Returns a vector of mock weather forecasts.
///
/// # Example
///
/// ```
/// use weather_mock::weather_data::{get_mock_weather_data, WeatherForecast};
///
/// let mock_data = get_forecast();
/// assert_eq!(mock_data.len(), 6);
/// ```
pub fn get_forecast() -> Vec<WeatherForecast> {
    vec![
        WeatherForecast {
            place: "Santa Cruz".to_string(),
            date: "2020-10-22".to_string(),
            weather: "Heavy Cloud".to_string(),
        },
        WeatherForecast {
            place: "Santa Cruz".to_string(),
            date: "2020-10-23".to_string(),
            weather: "Heavy Cloud".to_string(),
        },
        WeatherForecast {
            place: "Santa Cruz".to_string(),
            date: "2020-10-24".to_string(),
            weather: "Heavy Cloud".to_string(),
        },
        WeatherForecast {
            place: "Santa Cruz".to_string(),
            date: "2020-10-25".to_string(),
            weather: "Light Cloud".to_string(),
        },
        WeatherForecast {
            place: "Santa Cruz".to_string(),
            date: "2020-10-26".to_string(),
            weather: "Heavy Cloud".to_string(),
        },
        WeatherForecast {
            place: "Santa Cruz".to_string(),
            date: "2020-10-27".to_string(),
            weather: "Light Cloud".to_string(),
        },
    ]
}
