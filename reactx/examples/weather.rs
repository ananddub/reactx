//! Fetch live weather data with `use_fetch` and render it inside `Suspense`,
//! written with HTML-like `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example weather`
//! Needs network access; see <https://open-meteo.com>.

#![allow(non_snake_case)]

use reactx::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct Forecast {
    current_weather: CurrentWeather,
}

#[derive(Clone, PartialEq, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    weathercode: u32,
}

fn describe(code: u32) -> &'static str {
    match code {
        0 => "clear sky",
        1..=3 => "partly cloudy",
        45 | 48 => "fog",
        51..=57 => "drizzle",
        61..=67 => "rain",
        71..=77 => "snow",
        80..=82 => "showers",
        85 | 86 => "snow showers",
        95..=99 => "thunderstorm",
        _ => "unknown",
    }
}

fn forecast_url(latitude: f64, longitude: f64) -> String {
    format!(
        "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current_weather=true"
    )
}

#[component]
fn WeatherView(latitude: f64, longitude: f64) -> Element {
    let forecast = use_fetch::<Forecast>(forecast_url(latitude, longitude));
    let data = forecast.suspend()?;

    let content = match &*data.read() {
        Ok(forecast) => {
            let current = &forecast.current_weather;
            let headline = format!(
                "{:.1} °C — {}",
                current.temperature,
                describe(current.weathercode)
            );
            let wind = format!("Wind {:.1} km/h", current.windspeed);
            jsx! {
                <span class="text-blue-300 bold">{headline}</span>
                <span class="dim">{wind}</span>
            }
        }
        Err(error) => {
            let message = format!("Weather unavailable: {error}");
            jsx! { <span class="text-rose-400">{message}</span> }
        }
    };
    content
}

#[component]
fn App() -> Element {
    let cities = [
        ("Berlin", 52.52, 13.41),
        ("Tokyo", 35.68, 139.69),
        ("New York", 40.71, -74.01),
    ];
    let mut selected = use_signal(|| 0usize);
    let (name, latitude, longitude) = cities[selected().min(cities.len() - 1)];

    jsx! {
        <ThemeProvider>
            <Container class="flex flex-col border border-blue-300 rounded p-1 gap-1">
                <span class="bold">Weather</span>
                <div class="flex flex-row gap-1">
                    {cities.iter().enumerate().map(|(index, city)| jsx! {
                        <Button
                            key="{city.0}"
                            variant={if index == selected() { "primary" } else { "ghost" }}
                            on_click=move |_| selected.set(index)
                        >{city.0}</Button>
                    })}
                </div>
                <Suspense>
                    <WeatherView key="{name}" latitude={latitude} longitude={longitude} />
                </Suspense>
                <span class="dim">{"Data from open-meteo.com. Ctrl+C to quit."}</span>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
