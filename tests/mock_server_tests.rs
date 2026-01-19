use brightsky::types::*;
use brightsky::*;
use chrono::NaiveDate;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

#[tokio::test]
async fn test_current_weather_api_success() {
    let mock_server = MockServer::start().await;

    let mock_response = r#"{
        "weather": {
            "timestamp": "2023-08-07T12:00:00+00:00",
            "source_id": 1234,
            "cloud_cover": 75.0,
            "condition": "rain",
            "dew_point": 18.5,
            "icon": "rain",
            "pressure_msl": 1008.2,
            "relative_humidity": 85,
            "temperature": 22.3,
            "visibility": 8000,
            "precipitation_10": 0.2,
            "precipitation_30": 0.8,
            "precipitation_60": 1.5,
            "wind_speed_10": 15.5,
            "wind_direction_10": 230,
            "wind_gust_speed_10": 25.0
        },
        "sources": [
            {
                "id": 1234,
                "dwd_station_id": "01766",
                "wmo_station_id": "10315",
                "station_name": "Münster/Osnabrück",
                "observation_type": "synop",
                "first_record": "2010-01-01T00:00:00+00:00",
                "last_record": "2023-08-07T12:00:00+00:00",
                "lat": 52.1347,
                "lon": 7.6969,
                "height": 48.0,
                "distance": 5420.3
            }
        ]
    }"#;

    Mock::given(method("GET"))
        .and(path("/current_weather"))
        .and(query_param("lat", "52.52"))
        .and(query_param("lon", "13.4"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_response))
        .mount(&mock_server)
        .await;

    // Create a custom client with mock server URL
    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    // We need to create a custom URL for testing
    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();
    let response_text = response.text().await.unwrap();
    let weather_response: CurrentWeatherResponse = serde_json::from_str(&response_text).unwrap();

    assert_eq!(weather_response.weather.temperature, Some(22.3));
    assert_eq!(
        weather_response.weather.condition,
        Some(WeatherCondition::Rain)
    );
    assert_eq!(weather_response.weather.icon, Some(WeatherIcon::Rain));
    assert_eq!(weather_response.weather.precipitation_60, Some(1.5));
    assert_eq!(weather_response.sources.len(), 1);
    assert_eq!(
        weather_response.sources[0].station_name,
        "Münster/Osnabrück"
    );
}

#[tokio::test]
async fn test_weather_api_success() {
    let mock_server = MockServer::start().await;

    let mock_response = r#"{
        "weather": [
            {
                "timestamp": "2023-08-07T00:00:00+00:00",
                "source_id": 5678,
                "cloud_cover": 45.0,
                "condition": "dry",
                "dew_point": 12.1,
                "icon": "clear-day",
                "pressure_msl": 1015.8,
                "relative_humidity": 55,
                "temperature": 25.7,
                "visibility": 15000,
                "precipitation": 0.0,
                "solar": 0.8,
                "sunshine": 45.0,
                "wind_speed": 12.3,
                "wind_direction": 180,
                "wind_gust_speed": 18.5,
                "precipitation_probability": 5
            },
            {
                "timestamp": "2023-08-07T01:00:00+00:00",
                "source_id": 5678,
                "cloud_cover": 50.0,
                "condition": "dry",
                "dew_point": 13.2,
                "icon": "clear-day",
                "pressure_msl": 1015.5,
                "relative_humidity": 58,
                "temperature": 26.1,
                "visibility": 12000,
                "precipitation": 0.0,
                "solar": 0.9,
                "sunshine": 50.0,
                "wind_speed": 14.1,
                "wind_direction": 185,
                "wind_gust_speed": 20.2,
                "precipitation_probability": 10
            }
        ],
        "sources": [
            {
                "id": 5678,
                "dwd_station_id": "00420",
                "wmo_station_id": "10338",
                "station_name": "Berlin-Tempelhof",
                "observation_type": "historical",
                "first_record": "2010-01-01T00:00:00+00:00",
                "last_record": "2023-08-07T23:00:00+00:00",
                "lat": 52.4675,
                "lon": 13.4021,
                "height": 48.0,
                "distance": 1250.8
            }
        ]
    }"#;

    Mock::given(method("GET"))
        .and(path("/weather"))
        .and(query_param("date", "2023-08-07"))
        .and(query_param("lat", "52.52"))
        .and(query_param("lon", "13.4"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_response))
        .mount(&mock_server)
        .await;

    let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
    let query = WeatherQueryBuilder::new()
        .with_date(date)
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();
    let response_text = response.text().await.unwrap();
    let weather_response: WeatherResponse = serde_json::from_str(&response_text).unwrap();

    assert_eq!(weather_response.weather.len(), 2);
    assert_eq!(weather_response.weather[0].temperature, Some(25.7));
    assert_eq!(weather_response.weather[1].temperature, Some(26.1));
    assert_eq!(
        weather_response.weather[0].condition,
        Some(WeatherCondition::Dry)
    );
    assert_eq!(
        weather_response.weather[0].icon,
        Some(WeatherIcon::ClearDay)
    );
    assert_eq!(weather_response.sources.len(), 1);
    assert_eq!(
        weather_response.sources[0].station_name,
        Some("Berlin-Tempelhof".to_string())
    );
}

#[tokio::test]
async fn test_alerts_api_success() {
    let mock_server = MockServer::start().await;

    let mock_response = r#"{
        "alerts": [
            {
                "id": 9876,
                "alert_id": "2.49.0.1.276.0.DWD.PVW.1691234567.abc123",
                "status": "actual",
                "effective": "2023-08-07T06:00:00+00:00",
                "onset": "2023-08-07T12:00:00+00:00",
                "expires": "2023-08-07T20:00:00+00:00",
                "category": "met",
                "response_type": "prepare",
                "urgency": "future",
                "severity": "moderate",
                "certainty": "likely",
                "event_code": 61,
                "event_en": "thunderstorms",
                "event_de": "GEWITTER",
                "headline_en": "Official WARNING of THUNDERSTORMS",
                "headline_de": "Amtliche WARNUNG vor GEWITTERN",
                "description_en": "There is a risk of severe thunderstorms with heavy rain and hail.",
                "description_de": "Es besteht die Gefahr von schweren Gewittern mit Starkregen und Hagel.",
                "instruction_en": "Avoid outdoor activities and seek shelter indoors.",
                "instruction_de": "Vermeiden Sie Aktivitäten im Freien und suchen Sie Schutz in Gebäuden."
            }
        ],
        "location": {
            "warn_cell_id": 804159000,
            "name": "Stadt Berlin",
            "name_short": "Berlin",
            "district": "Berlin",
            "state": "Berlin",
            "state_short": "BE"
        }
    }"#;

    Mock::given(method("GET"))
        .and(path("/alerts"))
        .and(query_param("lat", "52.52"))
        .and(query_param("lon", "13.4"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_response))
        .mount(&mock_server)
        .await;

    let query = AlertsQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();
    let response_text = response.text().await.unwrap();
    let alerts_response: AlertsResponse = serde_json::from_str(&response_text).unwrap();

    assert_eq!(alerts_response.alerts.len(), 1);
    let alert = &alerts_response.alerts[0];
    assert_eq!(alert.id, 9876);
    assert_eq!(alert.status, AlertStatus::Actual);
    assert_eq!(alert.category, Some(AlertCategory::Met));
    assert_eq!(alert.response_type, Some(AlertResponseType::Prepare));
    assert_eq!(alert.urgency, Some(AlertUrgency::Future));
    assert_eq!(alert.severity, Some(AlertSeverity::Moderate));
    assert_eq!(alert.certainty, Some(AlertCertainty::Likely));
    assert_eq!(alert.event_en, Some("thunderstorms".to_string()));
    assert_eq!(alert.headline_en, "Official WARNING of THUNDERSTORMS");

    let location = alerts_response.location.unwrap();
    assert_eq!(location.name_short, "Berlin");
    assert_eq!(location.state_short, "BE");
}

#[tokio::test]
async fn test_radar_api_success() {
    let mock_server = MockServer::start().await;

    let mock_response = r#"{
        "radar": [
            {
                "timestamp": "2023-08-07T12:45:00+00:00",
                "source": "RADOLAN::RV::2023-08-07T12:45:00+00:00",
                "precipitation_5": [[0, 5, 10], [15, 20, 25]]
            }
        ],
        "geometry": {
            "type": "Polygon",
            "coordinates": [[7.5, 52.0], [7.6, 52.0], [7.6, 52.1], [7.5, 52.1], [7.5, 52.0]]
        },
        "bbox": [0, 0, 2, 2],
        "latlon_position": {
            "x": 1.5,
            "y": 1.2
        }
    }"#;

    Mock::given(method("GET"))
        .and(path("/radar"))
        .and(query_param("lat", "52.0"))
        .and(query_param("lon", "7.6"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_response))
        .mount(&mock_server)
        .await;

    let query = RadarWeatherQueryBuilder::new()
        .with_lat_lon((52.0, 7.6))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();
    let response_text = response.text().await.unwrap();
    let radar_response: RadarResponse = serde_json::from_str(&response_text).unwrap();

    assert_eq!(radar_response.radar.len(), 1);
    let radar = &radar_response.radar[0];
    assert_eq!(radar.timestamp, "2023-08-07T12:45:00+00:00");
    assert!(radar.source.contains("RADOLAN"));

    match &radar.precipitation_5 {
        MaybeCompressedPrecipitation::Plain(data) => {
            assert_eq!(data.len(), 2);
            assert_eq!(data[0], vec![0, 5, 10]);
            assert_eq!(data[1], vec![15, 20, 25]);
        }
        _ => panic!("Expected Plain precipitation data"),
    }

    assert!(radar_response.geometry.is_some());
    assert!(radar_response.bbox.is_some());
    assert!(radar_response.latlon_position.is_some());

    let position = radar_response.latlon_position.unwrap();
    assert_eq!(position.x, 1.5);
    assert_eq!(position.y, 1.2);
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;

    let error_response = r#"{
        "detail": "Invalid coordinates provided"
    }"#;

    Mock::given(method("GET"))
        .and(path("/current_weather"))
        .respond_with(ResponseTemplate::new(400).set_body_string(error_response))
        .mount(&mock_server)
        .await;

    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();

    assert_eq!(response.status(), 400);
    let error_text = response.text().await.unwrap();
    assert!(error_text.contains("Invalid coordinates"));
}

#[tokio::test]
async fn test_api_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/weather"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
    let query = WeatherQueryBuilder::new()
        .with_date(date)
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();

    assert_eq!(response.status(), 500);
}

#[tokio::test]
async fn test_malformed_json_response() {
    let mock_server = MockServer::start().await;

    let malformed_json = r#"{ "weather": { "invalid": json }"#;

    Mock::given(method("GET"))
        .and(path("/current_weather"))
        .respond_with(ResponseTemplate::new(200).set_body_string(malformed_json))
        .mount(&mock_server)
        .await;

    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();
    let response_text = response.text().await.unwrap();

    let result: Result<CurrentWeatherResponse, serde_json::Error> =
        serde_json::from_str(&response_text);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_empty_weather_array() {
    let mock_server = MockServer::start().await;

    let empty_response = r#"{
        "weather": [],
        "sources": []
    }"#;

    Mock::given(method("GET"))
        .and(path("/weather"))
        .respond_with(ResponseTemplate::new(200).set_body_string(empty_response))
        .mount(&mock_server)
        .await;

    let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
    let query = WeatherQueryBuilder::new()
        .with_date(date)
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();
    let response_text = response.text().await.unwrap();
    let weather_response: WeatherResponse = serde_json::from_str(&response_text).unwrap();

    assert_eq!(weather_response.weather.len(), 0);
    assert_eq!(weather_response.sources.len(), 0);
}

#[tokio::test]
async fn test_query_parameters_in_url() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/current_weather"))
        .and(query_param("lat", "52.52"))
        .and(query_param("lon", "13.4"))
        .and(query_param("max_dist", "10000"))
        .and(query_param("tz", "Europe/Berlin"))
        .and(query_param("units", "si"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(r#"{"weather": {}, "sources": []}"#),
        )
        .mount(&mock_server)
        .await;

    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .with_max_dist(10000)
        .with_tz("Europe/Berlin")
        .with_units(UnitType::Si)
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_multiple_station_ids_in_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/weather"))
        .and(query_param("date", "2023-08-07"))
        .and(query_param("dwd_station_id", "01766"))
        .and(query_param("dwd_station_id", "00420"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(r#"{"weather": [], "sources": []}"#),
        )
        .mount(&mock_server)
        .await;

    let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
    let query = WeatherQueryBuilder::new()
        .with_date(date)
        .with_dwd_station_id(vec!["01766", "00420"])
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url.as_str()).await.unwrap();

    assert_eq!(response.status(), 200);
}
