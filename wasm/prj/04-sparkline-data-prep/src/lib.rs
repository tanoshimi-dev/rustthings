use wasm_bindgen::{JsValue, prelude::*};

#[wasm_bindgen]
pub fn sparkline_points_from_csv(csv: &str, width: f64, height: f64) -> Result<String, JsValue> {
    let series = parse_csv_series(csv).map_err(|error| JsValue::from_str(&error))?;
    sparkline_points(&series, width, height).map_err(|error| JsValue::from_str(&error))
}

#[wasm_bindgen]
pub fn sparkline_svg_from_csv(
    csv: &str,
    width: f64,
    height: f64,
    stroke: &str,
) -> Result<String, JsValue> {
    let points = sparkline_points_from_csv(csv, width, height)?;

    Ok(format!(
        r#"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg"><polyline fill="none" stroke="{}" stroke-width="2" points="{}" /></svg>"#,
        trim_float(width),
        trim_float(height),
        stroke,
        points,
    ))
}

fn parse_csv_series(csv: &str) -> Result<Vec<f64>, String> {
    let values = csv
        .split(|ch: char| ch == ',' || ch.is_ascii_whitespace())
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<f64>()
                .map_err(|_| format!("invalid number: {part}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if values.len() < 2 {
        return Err("provide at least two numeric values for a sparkline".to_string());
    }

    Ok(values)
}

fn sparkline_points(series: &[f64], width: f64, height: f64) -> Result<String, String> {
    if width <= 0.0 || height <= 0.0 {
        return Err("width and height must be positive".to_string());
    }

    let min = series.iter().copied().fold(f64::INFINITY, f64::min);
    let max = series.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = max - min;
    let step_x = width / (series.len().saturating_sub(1) as f64);

    let mut points = Vec::with_capacity(series.len());
    for (index, value) in series.iter().enumerate() {
        let x = step_x * (index as f64);
        let y = if span.abs() < f64::EPSILON {
            height / 2.0
        } else {
            height - (((value - min) / span) * height)
        };

        points.push(format!("{},{}", trim_float(x), trim_float(y)));
    }

    Ok(points.join(" "))
}

fn trim_float(value: f64) -> String {
    let formatted = format!("{value:.2}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_series_is_normalized_into_points() {
        let points = sparkline_points(&parse_csv_series("4, 8, 6, 10").unwrap(), 120.0, 40.0).unwrap();
        assert_eq!(points, "0,40 40,13.33 80,26.67 120,0");
    }

    #[test]
    fn flat_series_stays_centered() {
        let points = sparkline_points(&parse_csv_series("5,5,5").unwrap(), 100.0, 30.0).unwrap();
        assert_eq!(points, "0,15 50,15 100,15");
    }

    #[test]
    fn invalid_number_returns_error() {
        let error = parse_csv_series("2, nope, 8").unwrap_err();
        assert_eq!(error, "invalid number: nope");
    }
}
