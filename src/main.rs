use warp::{http::StatusCode, Filter};
use chrono::Utc;

#[tokio::main]
async fn main() {

    let health_route = warp::path("health")
        .map(|| {
            let current_time = Utc::now();
            let response_text = format!("System OK - {}", current_time);
            warp::reply::html(response_text)
        });

    let routes = health_route
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}