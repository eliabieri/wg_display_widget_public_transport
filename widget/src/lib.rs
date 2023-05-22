use serde_json::Error;
use serde_json::Value;

use schemars::{schema_for, JsonSchema};
use serde::Deserialize;

wit_bindgen::generate!({
    path: "../wg_display_widget_wit/wit",
    world: "widget"
});

#[derive(JsonSchema, Deserialize)]
struct WidgetConfig {
    city: String,
}

const WIDGET_NAME: &str = "Rust Widget Template";

struct MyWidget;

impl Widget for MyWidget {
    fn get_name() -> wit_bindgen::rt::string::String {
        WIDGET_NAME.into()
    }

    fn run(config: WidgetContext) -> WidgetResult {
        // Widgets can log to the console
        logging::log(logging::Level::Info, WIDGET_NAME, "Widget run started");

        // Widgets can handle the case where no config is provided
        if "{}" == config.config {
            return WidgetResult {
                data: "No config provided".into(),
            };
        }

        // Widgets can parse their config with ease using serde
        let config: WidgetConfig =
            serde_json::from_str(&config.config).expect("Failed to parse config");

        // Widgets can make network requests
        let response = http::request(
            http::Method::Get,
            format!(
                "https://aareguru.existenz.ch/v2018/today?city={}",
                config.city
            )
            .as_str(),
            None,
        );
        let Ok(response) = response else {
            return WidgetResult {
                data: "Failed to make network request".into(),
            };
        };

        if 200 != response.status {
            return WidgetResult {
                data: format!("Response status != 200: {}", response.status),
            };
        }

        let data: Result<Value, Error> = serde_json::from_slice(response.bytes.as_slice());
        let result = match data {
            Ok(data) => format!("Aare Temperature in {}: {} C", config.city, data["aare"]),
            Err(_) => "Response from AareGuru could not be parsed".into(),
        };

        WidgetResult { data: result }
    }

    fn get_config_schema() -> wit_bindgen::rt::string::String {
        let schema = schema_for!(WidgetConfig);
        serde_json::to_string_pretty(&schema).unwrap()
    }

    fn get_version() -> wit_bindgen::rt::string::String {
        "1.0.0".into()
    }
}

export_widget!(MyWidget);
