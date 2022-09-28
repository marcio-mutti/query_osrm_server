use crate::types::Waypoint;
use geo::Point;
use types::{NearestResponse, RoutedResponse};

mod types;

pub async fn find_nearest(
    host: &str,
    port: u32,
    profile: &str,
    point: &Point<f32>,
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

pub async fn route(
    host: &str,
    port: u32,
    profile: &str,
    origin: &Point<f32>,
    destiny: &Point<f32>,
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
#[cfg(test)]
mod tests;
