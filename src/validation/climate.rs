use serde_json::Value;

pub fn validate_climate_payload(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;

    if data.get("temperature").is_some() {
        validate_temperature(payload)?;
    }
    if data.get("humidity").is_some() {
        validate_humidity(payload)?;
    }
    if data.get("pressure").is_some() {
        validate_pressure(payload)?;
    }
    if data.get("dew_point").is_some() {
        validate_dew_point(payload)?;
    }
    if data.get("wind_speed").is_some() {
        validate_wind_speed(payload)?;
    }
    if data.get("wind_direction").is_some() {
        validate_wind_direction(payload)?;
    }
    if data.get("rainfall").is_some() {
        validate_rainfall(payload)?;
    }
    if data.get("uv_index").is_some() {
        validate_uv_index(payload)?;
    }

    Ok(())
}

pub fn validate_temperature(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(temp) = data.get("temperature").and_then(|v| v.as_f64()) {
        if !(-100.0..=150.0).contains(&temp) {
            return Err(format!("temperatureOutOfRange:{}", temp));
        }
    }
    Ok(())
}

pub fn validate_humidity(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(humidity) = data.get("humidity").and_then(|v| v.as_f64()) {
        if !(0.0..=100.0).contains(&humidity) {
            return Err(format!("humidityOutOfRange:{}", humidity));
        }
    }
    Ok(())
}

pub fn validate_pressure(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(pressure) = data.get("pressure").and_then(|v| v.as_f64()) {
        if !(300.0..=1100.0).contains(&pressure) {
            return Err(format!("pressureOutOfRange:{}", pressure));
        }
    }
    Ok(())
}

pub fn validate_dew_point(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(dew_point) = data.get("dew_point").and_then(|v| v.as_f64()) {
        if !(-100.0..=150.0).contains(&dew_point) {
            return Err(format!("dewPointOutOfRange:{}", dew_point));
        }
    }
    Ok(())
}

pub fn validate_wind_speed(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(wind_speed) = data.get("wind_speed").and_then(|v| v.as_f64()) {
        if !(0.0..=100.0).contains(&wind_speed) {
            return Err(format!("windSpeedOutOfRange:{}", wind_speed));
        }
    }
    Ok(())
}

pub fn validate_wind_direction(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(wind_direction) = data.get("wind_direction").and_then(|v| v.as_f64()) {
        if !(0.0..=360.0).contains(&wind_direction) {
            return Err(format!("windDirectionOutOfRange:{}", wind_direction));
        }
    }
    Ok(())
}

pub fn validate_rainfall(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(rainfall) = data.get("rainfall").and_then(|v| v.as_f64()) {
        if !(0.0..=50.0).contains(&rainfall) {
            return Err(format!("rainfallOutOfRange:{}", rainfall));
        }
    }
    Ok(())
}

pub fn validate_uv_index(payload: &str) -> Result<(), String> {
    let data: Value = serde_json::from_str(payload).map_err(|_| "invalidJson".to_string())?;
    if let Some(uv_index) = data.get("uv_index").and_then(|v| v.as_u64()) {
        if uv_index > 15 {
            return Err(format!("uvIndexTooHigh:{}", uv_index));
        }
    }
    Ok(())
}
