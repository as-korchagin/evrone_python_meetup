use pyo3::prelude::*;
use std::f64::consts::PI;
const EARTH_RADIUS_M: f64 = 6372800.0;

fn deg_to_rad(degrees: f64) -> f64 {
    PI * degrees / 180.0
}

#[pyfunction]
fn haversine_m(lon1deg: f64, lat1deg: f64, lon2deg: f64, lat2deg: f64) -> PyResult<f64> {
    let lon1rad = deg_to_rad(lon1deg);
    let lat1rad = deg_to_rad(lat1deg);
    let lon2rad = deg_to_rad(lon2deg);
    let lat2rad = deg_to_rad(lat2deg);
    let dlon = lon2rad - lon1rad;
    let dlat = lat2rad - lat1rad;

    let c = ((dlat / 2.0).sin().powi(2)
        + lat1rad.cos() * lat2rad.cos() * (dlon / 2.0).sin().powi(2))
    .sqrt()
    .asin();
    Ok(c * 2.0 * EARTH_RADIUS_M)
}

#[pymodule]
fn haversine_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(haversine_m, m)?)?;
    Ok(())
}
