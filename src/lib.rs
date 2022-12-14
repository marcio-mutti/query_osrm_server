use crate::types::Waypoint;
use geo::Point;
use types::{NearestResponse, RoutedResponse};

pub mod types;

pub async fn find_nearest(
    host: &str,
    port: u32,
    profile: &str,
    point: &Point<f64>,
    number: u8,
) -> Result<Vec<Waypoint>, Box<dyn std::error::Error>> {
    let query_string = format!(
        "http:{}:{}/nearest/v1/{}/{},{}?number={}",
        host,
        port,
        profile,
        point.x(),
        point.y(),
        number
    );
    let resp: NearestResponse = reqwest::get(query_string).await?.json().await?;
    Ok(resp.waypoints())
}

pub fn find_nearest_blocking(
    host: &str,
    port: u32,
    profile: &str,
    point: &Point<f64>,
    number: u8,
) -> Result<Vec<Waypoint>, Box<dyn std::error::Error>> {
    let query_string = format!(
        "http:{}:{}/nearest/v1/{}/{},{}?number={}",
        host,
        port,
        profile,
        point.x(),
        point.y(),
        number
    );
    let resp: NearestResponse = reqwest::blocking::get(query_string)?.json()?;
    Ok(resp.waypoints())
}

pub async fn route(
    host: &str,
    port: u32,
    profile: &str,
    origin: &Point<f64>,
    destiny: &Point<f64>,
) -> Result<RoutedResponse, Box<dyn std::error::Error>> {
    let query_string = format!(
        "http://{}:{}/route/v1/{}/{},{};{},{}?geometries=geojson&overview=full&annotations=false",
        host,
        port,
        profile,
        origin.x(),
        origin.y(),
        destiny.x(),
        destiny.y()
    );
    let resp: RoutedResponse = reqwest::get(query_string).await?.json().await?;
    Ok(resp)
}
pub fn route_blocking(
    host: &str,
    port: u32,
    profile: &str,
    origin: &Point<f64>,
    destiny: &Point<f64>,
) -> Result<RoutedResponse, Box<dyn std::error::Error>> {
    let query_string = format!(
        "http://{}:{}/route/v1/{}/{},{};{},{}?geometries=geojson&overview=full&annotations=false",
        host,
        port,
        profile,
        origin.x(),
        origin.y(),
        destiny.x(),
        destiny.y()
    );
    let resp: RoutedResponse = reqwest::blocking::get(query_string)?.json()?;
    Ok(resp)
}
#[cfg(test)]
mod tests;
