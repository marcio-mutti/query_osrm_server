use crate::{find_nearest, route};

#[tokio::test]
pub async fn try_nearest() {
    //coords -15.799119447072439, -47.86058395336634
    let host = "localhost";
    let port = 5_000;
    let profile = "driving";
    let point = geo::point! {x: -47.86058395336634, y: -15.799119447072439};
    let number = 2;
    let results = find_nearest(host, port, profile, &point, number)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);
}
#[tokio::test]
pub async fn try_route() {
    let host = "localhost";
    let port = 5_000;
    let profile = "driving";
    let origin = geo::point! {x: -47.86058395336634, y: -15.799119447072439};
    let destiny = geo::point! {x: -46.65629817429345, y: -23.561417146144493};
    let result = route(host, port, profile, &origin, &destiny).await.unwrap();
    println!("Rotas retornadas:{}", result.routes().len());
    println!(
        "Geometria:\n{}",
        result.routes()[0].geometry().wkt_geometry()
    );
    // -23.561417146144493, -46.65629817429345
}
