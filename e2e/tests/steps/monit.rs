//! E2E tests for `metrics` HTTP server.

use reqwest::{get, StatusCode};

use crate::{world::Response, World};

use cucumber::{then, when};

/// URL address where `metrics` HTTP server for E2E tests is served.
const URL: &str = "http://127.0.0.1:9372/metrics";

#[when(regex = "^I check metrics$")]
async fn check_metrics(w: &mut World) {
    let resp = get(URL).await.expect("HTTP request failed");
    w.metrics_responses()
        .push(Response::from_reqwest(resp).await);
}

#[then(regex = r"^response code is `(\d+)`$")]
fn response_code_is(w: &mut World, expected: StatusCode) {
    let resp = w
        .metrics_responses()
        .last()
        .expect("`World` has no `Response`");

    assert_eq!(resp.code, expected, "resp: {resp:#?}");
}

#[then(regex = r"^response contains `(\S+)` metrics?$")]
async fn metric_resp_contains(w: &mut World, metric: String) {
    let body = &w
        .metrics_responses()
        .last()
        .expect("No `Response`s have been collected")
        .body;

    assert!(
        body.contains(&format!("# TYPE {metric} gauge")),
        "Response has no `up` gauge metric",
    );
}
