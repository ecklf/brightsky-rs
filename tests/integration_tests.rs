use brightsky::types::*;
use brightsky::*;
use chrono::NaiveDate;

#[cfg(test)]
mod types_tests {
    use super::*;

    #[tokio::test]
    async fn test_weather_icon_deserialization() {
        let json_values = vec![
            ("\"clear-day\"", WeatherIcon::ClearDay),
            ("\"clear-night\"", WeatherIcon::ClearNight),
            ("\"partly-cloudy-day\"", WeatherIcon::PartlyCloudyDay),
            ("\"partly-cloudy-night\"", WeatherIcon::PartlyCloudyNight),
            ("\"cloudy\"", WeatherIcon::Cloudy),
            ("\"fog\"", WeatherIcon::Fog),
            ("\"wind\"", WeatherIcon::Wind),
            ("\"rain\"", WeatherIcon::Rain),
            ("\"sleet\"", WeatherIcon::Sleet),
            ("\"snow\"", WeatherIcon::Snow),
            ("\"hail\"", WeatherIcon::Hail),
            ("\"thunderstorm\"", WeatherIcon::Thunderstorm),
            ("\"unknown\"", WeatherIcon::Unknown),
        ];

        for (json, expected) in json_values {
            let result: WeatherIcon = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_weather_condition_deserialization() {
        let json_values = vec![
            ("\"dry\"", WeatherCondition::Dry),
            ("\"fog\"", WeatherCondition::Fog),
            ("\"rain\"", WeatherCondition::Rain),
            ("\"sleet\"", WeatherCondition::Sleet),
            ("\"snow\"", WeatherCondition::Snow),
            ("\"hail\"", WeatherCondition::Hail),
            ("\"thunderstorm\"", WeatherCondition::Thunderstorm),
            ("\"unknown\"", WeatherCondition::Unknown),
        ];

        for (json, expected) in json_values {
            let result: WeatherCondition = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_observation_type_deserialization() {
        let json_values = vec![
            ("\"historical\"", ObservationType::Historical),
            ("\"current\"", ObservationType::Current),
            ("\"synop\"", ObservationType::Synop),
            ("\"forecast\"", ObservationType::Forecast),
        ];

        for (json, expected) in json_values {
            let result: ObservationType = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_unit_type_deserialization() {
        let json_values = vec![("\"si\"", UnitType::Si), ("\"dwd\"", UnitType::Dwd)];

        for (json, expected) in json_values {
            let result: UnitType = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_alert_status_deserialization() {
        let json_values = vec![
            ("\"actual\"", AlertStatus::Actual),
            ("\"test\"", AlertStatus::Test),
        ];

        for (json, expected) in json_values {
            let result: AlertStatus = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_alert_category_deserialization() {
        let json_values = vec![
            ("\"met\"", AlertCategory::Met),
            ("\"health\"", AlertCategory::Health),
        ];

        for (json, expected) in json_values {
            let result: AlertCategory = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_alert_response_type_deserialization() {
        let json_values = vec![
            ("\"prepare\"", AlertResponseType::Prepare),
            ("\"allclear\"", AlertResponseType::AllClear),
            ("\"none\"", AlertResponseType::None),
            ("\"monitor\"", AlertResponseType::Monitor),
        ];

        for (json, expected) in json_values {
            let result: AlertResponseType = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_alert_urgency_deserialization() {
        let json_values = vec![
            ("\"immediate\"", AlertUrgency::Immediate),
            ("\"future\"", AlertUrgency::Future),
        ];

        for (json, expected) in json_values {
            let result: AlertUrgency = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_alert_severity_deserialization() {
        let json_values = vec![
            ("\"minor\"", AlertSeverity::Minor),
            ("\"moderate\"", AlertSeverity::Moderate),
            ("\"severe\"", AlertSeverity::Severe),
            ("\"extreme\"", AlertSeverity::Extreme),
        ];

        for (json, expected) in json_values {
            let result: AlertSeverity = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_alert_certainty_deserialization() {
        let json_values = vec![
            ("\"observed\"", AlertCertainty::Observed),
            ("\"likely\"", AlertCertainty::Likely),
        ];

        for (json, expected) in json_values {
            let result: AlertCertainty = serde_json::from_str(json).unwrap();
            assert_eq!(result, expected, "Failed for JSON: {}", json);
        }
    }

    #[tokio::test]
    async fn test_maybe_compressed_precipitation_plain() {
        let json = "[[10, 20, 30], [40, 50, 60]]";
        let result: MaybeCompressedPrecipitation = serde_json::from_str(json).unwrap();

        match result {
            MaybeCompressedPrecipitation::Plain(data) => {
                assert_eq!(data.len(), 2);
                assert_eq!(data[0], vec![10, 20, 30]);
                assert_eq!(data[1], vec![40, 50, 60]);
            }
            _ => panic!("Expected Plain format"),
        }
    }

    #[tokio::test]
    async fn test_weather_response_deserialization() {
        let json = r#"{
            "weather": [],
            "sources": []
        }"#;

        let result: WeatherResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.weather.len(), 0);
        assert_eq!(result.sources.len(), 0);
    }

    #[tokio::test]
    async fn test_current_weather_response_deserialization() {
        let json = r#"{
            "weather": {
                "timestamp": "2023-08-07T12:00:00Z",
                "source_id": 1234,
                "cloud_cover": 50.0,
                "condition": "dry",
                "dew_point": 15.5,
                "icon": "clear-day",
                "pressure_msl": 1013.25,
                "relative_humidity": 65,
                "temperature": 22.5,
                "visibility": 10000
            },
            "sources": []
        }"#;

        let result: CurrentWeatherResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.weather.timestamp, "2023-08-07T12:00:00Z");
        assert_eq!(result.weather.source_id, 1234);
        assert_eq!(result.weather.cloud_cover, Some(50.0));
        assert_eq!(result.weather.condition, Some(WeatherCondition::Dry));
        assert_eq!(result.weather.icon, Some(WeatherIcon::ClearDay));
        assert_eq!(result.weather.temperature, Some(22.5));
    }

    #[tokio::test]
    async fn test_alerts_response_deserialization() {
        let json = r#"{
            "alerts": [],
            "location": null
        }"#;

        let result: AlertsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.alerts.len(), 0);
        assert!(result.location.is_none());
    }
}

#[cfg(test)]
mod error_tests {
    use super::*;

    #[tokio::test]
    async fn test_date_not_set_error() {
        let result = WeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BlindSkyClientError::DateNotSet => (),
            _ => panic!("Expected DateNotSet error"),
        }
    }

    #[tokio::test]
    async fn test_invalid_latitude_error() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((91.0, 13.4))
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BlindSkyClientError::InvalidLongitude(lat) => assert_eq!(lat, 91.0),
            _ => panic!("Expected InvalidLongitude error"),
        }
    }

    #[tokio::test]
    async fn test_invalid_longitude_error() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 181.0))
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BlindSkyClientError::InvalidLongitude(lon) => assert_eq!(lon, 181.0),
            _ => panic!("Expected InvalidLongitude error"),
        }
    }

    #[tokio::test]
    async fn test_invalid_max_distance_error() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_max_dist(500001)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BlindSkyClientError::InvalidMaxDistance(dist) => assert_eq!(dist, 500001),
            _ => panic!("Expected InvalidMaxDistance error"),
        }
    }

    #[tokio::test]
    async fn test_error_display() {
        let errors = vec![
            BlindSkyClientError::DateNotSet,
            BlindSkyClientError::InvalidLatitude(95.0),
            BlindSkyClientError::InvalidLongitude(190.0),
            BlindSkyClientError::InvalidMaxDistance(600000),
        ];

        for error in errors {
            let error_string = format!("{}", error);
            assert!(!error_string.is_empty());
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_brightsky_client_url_generation() {
        let current_weather_query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build()
            .unwrap();

        let url = current_weather_query
            .to_url("https://api.brightsky.dev")
            .unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("api.brightsky.dev"));
        assert_eq!(url.path(), "/current_weather");
    }

    #[tokio::test]
    async fn test_weather_query_builder_complete_flow() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let last_date = NaiveDate::from_ymd_opt(2023, 8, 10).unwrap();

        let query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_last_date(last_date)
            .with_lat_lon((52.52, 13.4))
            .with_max_dist(25000)
            .with_tz("Europe/Berlin")
            .with_units(UnitType::Si)
            .build()
            .unwrap();

        assert_eq!(query.date, Some(date));
        assert_eq!(query.last_date, Some(last_date));
        assert_eq!(query.lat, Some("52.52".to_string()));
        assert_eq!(query.lon, Some("13.4".to_string()));
        assert_eq!(query.max_dist, Some("25000".to_string()));
        assert_eq!(query.tz, Some("Europe/Berlin".to_string()));
        assert_eq!(query.units, Some(UnitType::Si));
    }

    #[tokio::test]
    async fn test_current_weather_query_builder_complete_flow() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_max_dist(15000)
            .with_dwd_station_id(vec!["01766".to_string(), "00420".to_string()])
            .with_wmo_station_id(vec!["10315".to_string()])
            .with_source_id(vec![1234, 5678])
            .with_tz("Europe/Berlin")
            .with_units(UnitType::Dwd)
            .build()
            .unwrap();

        assert_eq!(query.lat, Some("52.52".to_string()));
        assert_eq!(query.lon, Some("13.4".to_string()));
        assert_eq!(query.max_dist, Some("15000".to_string()));
        assert_eq!(
            query.dwd_station_id,
            Some(vec!["01766".to_string(), "00420".to_string()])
        );
        assert_eq!(query.wmo_station_id, Some(vec!["10315".to_string()]));
        assert_eq!(
            query.source_id,
            Some(vec!["1234".to_string(), "5678".to_string()])
        );
        assert_eq!(query.tz, Some("Europe/Berlin".to_string()));
        assert_eq!(query.units, Some(UnitType::Dwd));
    }

    #[tokio::test]
    async fn test_edge_case_coordinates() {
        let edge_cases = vec![
            (-90.0, -180.0), // Min values
            (90.0, 180.0),   // Max values
            (0.0, 0.0),      // Origin
            (-45.5, 123.7),  // Negative latitude
        ];

        for (lat, lon) in edge_cases {
            let result = CurrentWeatherQueryBuilder::new()
                .with_lat_lon((lat, lon))
                .build();

            assert!(result.is_ok(), "Failed for coordinates ({}, {})", lat, lon);
        }
    }

    #[tokio::test]
    async fn test_max_distance_edge_cases() {
        let distances = vec![0, 1, 50000, 500000];

        for dist in distances {
            let result = CurrentWeatherQueryBuilder::new()
                .with_lat_lon((52.52, 13.4))
                .with_max_dist(dist)
                .build();

            assert!(result.is_ok(), "Failed for distance {}", dist);
        }
    }

    #[tokio::test]
    async fn test_multiple_station_ids() {
        let query = WeatherQueryBuilder::new()
            .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
            .with_dwd_station_id(vec!["01766", "00420", "01224"])
            .with_wmo_station_id(vec!["10315", "10338"])
            .build()
            .unwrap();

        let url = query.to_url("https://api.brightsky.dev").unwrap();
        let query_str = url.query().unwrap();

        // Check that all station IDs are present
        assert!(query_str.contains("dwd_station_id=01766"));
        assert!(query_str.contains("dwd_station_id=00420"));
        assert!(query_str.contains("dwd_station_id=01224"));
        assert!(query_str.contains("wmo_station_id=10315"));
        assert!(query_str.contains("wmo_station_id=10338"));
    }

    #[tokio::test]
    async fn test_radar_query_builder() {
        let query = RadarWeatherQueryBuilder::new()
            .with_lat_lon((52.0, 7.6))
            .with_distance(50000)
            .with_compression_format(RadarCompressionFormat::Compressed)
            .with_tz("Europe/Berlin")
            .build()
            .unwrap();

        assert_eq!(query.lat, Some("52.0".to_string()));
        assert_eq!(query.lon, Some("7.6".to_string()));
        assert_eq!(query.distance, Some(50000));
        assert_eq!(
            query.compression_format,
            Some(RadarCompressionFormat::Compressed)
        );
        assert_eq!(query.tz, Some("Europe/Berlin".to_string()));
    }

    #[tokio::test]
    async fn test_radar_query_with_bbox() {
        let query = RadarWeatherQueryBuilder::new()
            .with_bbox(vec![100, 100, 300, 300])
            .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
            .build()
            .unwrap();

        assert_eq!(query.bbox, Some(vec![100, 100, 300, 300]));
        assert_eq!(
            query.date,
            Some(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
        );
    }

    #[tokio::test]
    async fn test_radar_query_url_generation() {
        let query = RadarWeatherQueryBuilder::new()
            .with_lat_lon((52.0, 7.6))
            .with_distance(25000)
            .with_compression_format(RadarCompressionFormat::Plain)
            .build()
            .unwrap();

        let url = query.to_url("https://api.brightsky.dev").unwrap();

        assert_eq!(url.path(), "/radar");
        assert!(url.query().unwrap().contains("lat=52"));
        assert!(url.query().unwrap().contains("lon=7.6"));
        assert!(url.query().unwrap().contains("distance=25000"));
        assert!(url.query().unwrap().contains("format=plain"));
    }

    #[tokio::test]
    async fn test_alerts_query_builder() {
        let query = AlertsQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_tz("Europe/Berlin")
            .build()
            .unwrap();

        assert_eq!(query.lat, Some("52.52".to_string()));
        assert_eq!(query.lon, Some("13.4".to_string()));
        assert_eq!(query.tz, Some("Europe/Berlin".to_string()));
    }

    #[tokio::test]
    async fn test_alerts_query_with_warn_cell() {
        let query = AlertsQueryBuilder::new()
            .with_warn_cell_id(803159016)
            .build()
            .unwrap();

        assert_eq!(query.warn_cell_id, Some("803159016".to_string()));
    }

    #[tokio::test]
    async fn test_alerts_query_empty() {
        let query = AlertsQueryBuilder::new().build().unwrap();

        assert!(query.lat.is_none());
        assert!(query.lon.is_none());
        assert!(query.warn_cell_id.is_none());
        assert!(query.tz.is_none());
    }

    #[tokio::test]
    async fn test_alerts_query_url_generation() {
        let query = AlertsQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_warn_cell_id(803159016)
            .with_tz("Europe/Berlin")
            .build()
            .unwrap();

        let url = query.to_url("https://api.brightsky.dev").unwrap();

        assert_eq!(url.path(), "/alerts");
        assert!(url.query().unwrap().contains("lat=52.52"));
        assert!(url.query().unwrap().contains("lon=13.4"));
        assert!(url.query().unwrap().contains("warn_cell_id=803159016"));
        assert!(
            url.query().unwrap().contains("tz=Europe") && url.query().unwrap().contains("Berlin")
        );
    }

    #[tokio::test]
    async fn test_radar_compression_formats() {
        let formats = vec![
            (RadarCompressionFormat::Compressed, "compressed"),
            (RadarCompressionFormat::Bytes, "bytes"),
            (RadarCompressionFormat::Plain, "plain"),
        ];

        for (format, expected_str) in formats {
            let query = RadarWeatherQueryBuilder::new()
                .with_lat_lon((52.0, 7.6))
                .with_compression_format(format)
                .build()
                .unwrap();

            let url = query.to_url("https://api.brightsky.dev").unwrap();
            assert!(
                url.query()
                    .unwrap()
                    .contains(&format!("format={}", expected_str))
            );
        }
    }

    #[tokio::test]
    async fn test_invalid_coordinates_radar() {
        let result = RadarWeatherQueryBuilder::new()
            .with_lat_lon((91.0, 13.4))
            .build();

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_coordinates_alerts() {
        let result = AlertsQueryBuilder::new()
            .with_lat_lon((52.52, 181.0))
            .build();

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_coordinate_precision_preservation() {
        // Test that coordinates with decimals are preserved correctly
        let radar_query = RadarWeatherQueryBuilder::new()
            .with_lat_lon((52.2, 7.6))
            .build()
            .unwrap();

        assert_eq!(radar_query.lat, Some("52.2".to_string()));
        assert_eq!(radar_query.lon, Some("7.6".to_string()));

        let alerts_query = AlertsQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build()
            .unwrap();

        assert_eq!(alerts_query.lat, Some("52.52".to_string()));
        assert_eq!(alerts_query.lon, Some("13.4".to_string()));

        let current_query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.0, 13.0))
            .build()
            .unwrap();

        // Whole numbers should get .0 appended
        assert_eq!(current_query.lat, Some("52.0".to_string()));
        assert_eq!(current_query.lon, Some("13.0".to_string()));

        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let weather_query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_lat_lon((52.123456789, 13.987654321))
            .build()
            .unwrap();

        // Test that all decimal places are preserved (up to f64 precision)
        assert_eq!(weather_query.lat, Some("52.123456789".to_string()));
        assert_eq!(weather_query.lon, Some("13.987654321".to_string()));

        // Test high precision coordinates
        let high_precision_query = RadarWeatherQueryBuilder::new()
            .with_lat_lon((52.123456789012345, 13.987654321098765))
            .build()
            .unwrap();

        // These should preserve whatever f64 can represent
        assert!(high_precision_query.lat.unwrap().starts_with("52.12345"));
        assert!(high_precision_query.lon.unwrap().starts_with("13.98765"));
    }
}
