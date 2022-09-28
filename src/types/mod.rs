use geo::{coord, Geometry};
use serde::Deserialize;
use wkt::ToWkt;
//use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct Waypoint {
    name: String,
    location: [f64; 2],
    distance: f64,
    hint: String,
    nodes: Option<[u64; 2]>,
}
impl Waypoint {
    pub fn geometry(&self) -> geo::Point<f64> {
        geo::Point::new(self.location[0], self.location[1])
    }
}
#[derive(Deserialize)]
pub struct Route {
    distance: f64,
    duration: f64,
    geometry: GeoJSONGeometry,
}

impl<'a> Route {
    pub fn geometry(&'a self) -> &'a GeoJSONGeometry {
        &self.geometry
    }
}
#[derive(Deserialize)]
pub struct GeoJSONGeometry {
    //At this  time, focus on linestring, eventually find out if other geometries needed
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<[f64; 2]>,
}
impl GeoJSONGeometry {
    pub fn geometry(&self) -> geo::Geometry<f64> {
        //At this time, only linestrings.
        geo::LineString::new(
            self.coordinates
                .iter()
                .map(|cs| coord! {x: cs[0], y:cs[1]})
                .collect(),
        )
        .into()
    }
    pub fn wkt_geometry(&self) -> String {
        let geom = geo::LineString::new(
            self.coordinates
                .iter()
                .map(|cs| coord! {x: cs[0], y:cs[1]})
                .collect(),
        );
        geom.wkt_string()
    }
}

#[derive(Deserialize)]
pub struct RouteLeg {
    distance: f64,
    duration: f64,
    weight: f64,
    summary: String,
    steps: Vec<RouteStep>,
}

#[derive(Deserialize)]
pub struct Lane {
    indications: Vec<String>,
    valid: bool,
}

#[derive(Deserialize)]
pub struct Intersection {
    location: [f64; 2],
    bearings: Vec<u16>,
    classes: Vec<String>,
    entry: Vec<bool>,
    #[serde(rename = "in")]
    in_bearing: usize,
    #[serde(rename = "out")]
    out_bearing: usize,
    lanes: Vec<Lane>,
}

#[derive(Deserialize)]
pub struct RouteStep {
    distance: f64,
    duration: f64,
    geometry: String,
    weight: f64,
    name: String,
    #[serde(rename = "ref")]
    route_ref: i32,
    pronunciation: String,
    destination: Vec<Waypoint>,
    exits: Vec<String>,
    mode: String,
    maneuver: StepManeuver,
    intersections: Vec<Intersection>,
    rotary_name: String,
    rotary_pronunciation: String,
    driving_side: String,
}

#[derive(Deserialize)]
pub struct StepManeuver {
    location: [f64; 2],
    #[serde(rename = "type")]
    maneuver_type: String,
}

#[derive(Deserialize)]
pub struct NearestResponse {
    code: String,
    waypoints: Vec<Waypoint>,
}

impl NearestResponse {
    pub fn is_ok(&self) -> bool {
        self.code == "Ok"
    }
    pub fn waypoints(self) -> Vec<Waypoint> {
        self.waypoints
    }
}

#[derive(Deserialize)]
pub struct RoutedResponse {
    code: String,
    routes: Vec<Route>,
    waypoints: Vec<Waypoint>,
}
impl<'a> RoutedResponse {
    pub fn routes(&'a self) -> &'a [Route] {
        &self.routes
    }
}
impl RoutedResponse {
    pub fn nth_geometry(&self, nth: usize) -> Option<Geometry<f64>> {
        self.routes.get(nth).map(|r| r.geometry().geometry())
    }
}
