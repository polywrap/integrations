pub mod wrap;

pub use wrap::*;
use crate::imported::{
    HttpModule,
    ArgsPost,
    HttpRequest,
    HttpResponseType
};
use polywrap_wasm_rs::JSON;
use serde::Deserialize;

#[derive(Deserialize)]
struct ErrorLocation {
    column: i32,
    line: i32
}

#[derive(Deserialize)]
struct Error {
    locations: Option<Vec<ErrorLocation>>,
    message: String,
}

#[derive(Deserialize)]
struct RequestError {
    errors: Vec<Error>,
}

pub fn query_subgraph(args: ArgsQuerySubgraph) -> String {
    let ArgsQuerySubgraph { url, query } = args;
    // prepare http request
    let request: Option<HttpRequest> = Some(HttpRequest {
        headers: None,
        url_params: None,
        response_type: HttpResponseType::TEXT,
        body: Some(JSON::json!({ "query": &query }).to_string()),
        form_data: None,
        timeout: None
    });

    // send http request and get response body
    let data: String = match HttpModule::post(&ArgsPost { url, request }) {
        Err(e) => panic!("GraphNode Wrapper: errors encountered. Error: {}", &e),
        Ok(None) => panic!("GraphNode Wrapper: response is undefined."),
        Ok(Some(response)) => match response.body {
            None => panic!("GraphNode Wrapper: response body is undefined."),
            Some(body) => body,
        },
    };

    // check if response body contains error
    if let Ok(request_error) = JSON::from_str::<RequestError>(&data) {
        let e: String = request_error.errors.iter().map(|e| -> String {
            if e.locations.is_none() {
                return format!("\n-Message: {}", &e.message);
            }
            let locations: String = e.locations.as_ref().unwrap().iter()
                .map(|loc| format!("(col: {}, line: {})", loc.column, loc.line))
                .collect::<Vec<String>>()
                .join(", ");
            return format!("\n -Locations: {} \n-Message: {}", &locations, &e.message);
        }).collect::<Vec<String>>().join("\n");
        panic!("GraphNodePlugin: errors in query string. Errors: {}", &e);
    }

    return data;
}