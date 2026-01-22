use brightsky::types::*;
use brightsky::*;
use chrono::NaiveDate;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

#[cfg(feature = "reqwest")]
use brightsky::ext::BrightSkyReqwestExt;

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
                "station_name": "Munster/Osnabruck",
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

    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response: CurrentWeatherResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    assert_eq!(response.weather.temperature, Some(22.3));
    assert_eq!(response.weather.condition, Some(WeatherCondition::Rain));
    assert_eq!(response.weather.icon, Some(WeatherIcon::Rain));
    assert_eq!(response.weather.precipitation_60, Some(1.5));
    assert_eq!(response.sources.len(), 1);
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
    let response: WeatherResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    assert_eq!(response.weather.len(), 1);
    assert_eq!(response.weather[0].temperature, Some(25.7));
    assert_eq!(response.weather[0].condition, Some(WeatherCondition::Dry));
    assert_eq!(response.sources.len(), 1);
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
                "description_en": "Risk of severe thunderstorms.",
                "description_de": "Gefahr von schweren Gewittern.",
                "instruction_en": "Seek shelter.",
                "instruction_de": "Suchen Sie Schutz."
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
    let response: AlertsResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    assert_eq!(response.alerts.len(), 1);
    let alert = &response.alerts[0];
    assert_eq!(alert.id, 9876);
    assert_eq!(alert.status, AlertStatus::Actual);
    assert_eq!(alert.category, Some(AlertCategory::Met));

    let location = response.location.unwrap();
    assert_eq!(location.name_short, "Berlin");
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
    let response: RadarResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    assert_eq!(response.radar.len(), 1);
    let radar = &response.radar[0];
    assert!(radar.source.contains("RADOLAN"));

    match &radar.precipitation_5 {
        MaybeCompressedPrecipitation::Plain(data) => {
            assert_eq!(data.len(), 2);
            assert_eq!(data[0], vec![0, 5, 10]);
        }
        _ => panic!("Expected Plain precipitation data"),
    }
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/current_weather"))
        .respond_with(ResponseTemplate::new(400).set_body_string(r#"{"detail": "Invalid"}"#))
        .mount(&mock_server)
        .await;

    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()
        .unwrap();

    let url = query.to_url(&mock_server.uri()).unwrap();
    let response = reqwest::get(url).await.unwrap();

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn test_empty_weather_array() {
    let mock_server = MockServer::start().await;

    let empty_response = r#"{"weather": [], "sources": []}"#;

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
    let response: WeatherResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    assert_eq!(response.weather.len(), 0);
    assert_eq!(response.sources.len(), 0);
}

// Tests for the reqwest extension trait
#[cfg(feature = "reqwest")]
mod ext_tests {
    use super::*;

    #[tokio::test]
    async fn test_reqwest_ext_get_brightsky() {
        let mock_server = MockServer::start().await;

        let mock_response = r#"{
            "weather": {
                "timestamp": "2023-08-07T12:00:00+00:00",
                "source_id": 1234,
                "temperature": 22.5,
                "condition": "dry",
                "icon": "clear-day"
            },
            "sources": []
        }"#;

        Mock::given(method("GET"))
            .and(path("/current_weather"))
            .and(query_param("lat", "52.52"))
            .and(query_param("lon", "13.4"))
            .respond_with(ResponseTemplate::new(200).set_body_string(mock_response))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build()
            .unwrap();

        let response: CurrentWeatherResponse = client
            .get_brightsky_with_host(query, &mock_server.uri())
            .await
            .unwrap();

        assert_eq!(response.weather.temperature, Some(22.5));
        assert_eq!(response.weather.condition, Some(WeatherCondition::Dry));
    }

    #[tokio::test]
    async fn test_reqwest_ext_weather_query() {
        let mock_server = MockServer::start().await;

        let mock_response = r#"{
            "weather": [
                {
                    "timestamp": "2023-08-07T00:00:00+00:00",
                    "source_id": 5678,
                    "temperature": 25.0
                }
            ],
            "sources": []
        }"#;

        Mock::given(method("GET"))
            .and(path("/weather"))
            .and(query_param("date", "2023-08-07"))
            .respond_with(ResponseTemplate::new(200).set_body_string(mock_response))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let query = WeatherQueryBuilder::new()
            .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
            .with_lat_lon((52.52, 13.4))
            .build()
            .unwrap();

        let response: WeatherResponse = client
            .get_brightsky_with_host(query, &mock_server.uri())
            .await
            .unwrap();

        assert_eq!(response.weather.len(), 1);
        assert_eq!(response.weather[0].temperature, Some(25.0));
    }
}
