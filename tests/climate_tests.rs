use eco_weave::validation::climate::{validate_dew_point, validate_humidity, validate_pressure, validate_rainfall, validate_temperature, validate_uv_index, validate_wind_direction, validate_wind_speed};

 #[test]
    fn test_validate_temperature() {
        let valid_payload = r#"{"temperature": 75.0}"#;
        assert!(validate_temperature(valid_payload).is_ok());

        let invalid_payload = r#"{"temperature": 200.0}"#;
        let err = validate_temperature(invalid_payload).unwrap_err();
        assert_eq!(err, "temperatureOutOfRange:200");
    }

    #[test]
    fn test_validate_humidity() {
        let valid_payload = r#"{"humidity": 50.0}"#;
        assert!(validate_humidity(valid_payload).is_ok());

        let invalid_payload = r#"{"humidity": 120.0}"#;
        let err = validate_humidity(invalid_payload).unwrap_err();
        assert_eq!(err, "humidityOutOfRange:120");
    }

    #[test]
    fn test_validate_pressure() {
        let valid_payload = r#"{"pressure": 1013.0}"#;
        assert!(validate_pressure(valid_payload).is_ok());

        let invalid_payload = r#"{"pressure": 250.0}"#;
        let err = validate_pressure(invalid_payload).unwrap_err();
        assert_eq!(err, "pressureOutOfRange:250");
    }

    #[test]
    fn test_validate_dew_point() {
        let valid_payload = r#"{"dew_point": 60.0}"#;
        assert!(validate_dew_point(valid_payload).is_ok());

        let invalid_payload = r#"{"dew_point": -150.0}"#;
        let err = validate_dew_point(invalid_payload).unwrap_err();
        assert_eq!(err, "dewPointOutOfRange:-150");
    }

    #[test]
    fn test_validate_wind_speed() {
        let valid_payload = r#"{"wind_speed": 10.0}"#;
        assert!(validate_wind_speed(valid_payload).is_ok());

        let invalid_payload = r#"{"wind_speed": 150.0}"#;
        let err = validate_wind_speed(invalid_payload).unwrap_err();
        assert_eq!(err, "windSpeedOutOfRange:150");
    }

    #[test]
    fn test_validate_wind_direction() {
        let valid_payload = r#"{"wind_direction": 180.0}"#;
        assert!(validate_wind_direction(valid_payload).is_ok());

        let invalid_payload = r#"{"wind_direction": 400.0}"#;
        let err = validate_wind_direction(invalid_payload).unwrap_err();
        assert_eq!(err, "windDirectionOutOfRange:400");
    }

    #[test]
    fn test_validate_rainfall() {
        let valid_payload = r#"{"rainfall": 1.0}"#;
        assert!(validate_rainfall(valid_payload).is_ok());

        let invalid_payload = r#"{"rainfall": 60.0}"#;
        let err = validate_rainfall(invalid_payload).unwrap_err();
        assert_eq!(err, "rainfallOutOfRange:60");
    }

    #[test]
    fn test_validate_uv_index() {
        let valid_payload = r#"{"uv_index": 5}"#;
        assert!(validate_uv_index(valid_payload).is_ok());

        let invalid_payload = r#"{"uv_index": 20}"#;
        let err = validate_uv_index(invalid_payload).unwrap_err();
        assert_eq!(err, "uvIndexTooHigh:20");
    }

    #[test]
    fn test_invalid_json_format() {
        let invalid_payload = r#"{invalid_json}"#;
        let err = validate_temperature(invalid_payload).unwrap_err();
        assert_eq!(err, "invalidJson");
    }
