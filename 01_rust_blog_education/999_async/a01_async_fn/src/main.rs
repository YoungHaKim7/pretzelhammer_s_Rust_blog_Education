use std::{
    collections::HashMap,
    fmt::{write, Display},
    fs,
    path::PathBuf,
    sync::Arc,
};

// struct Headers(HashMap<String, String>);
struct HeaderError;
struct FileName(String);

impl Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, "Header not found")
    }
}

impl std::error::Error for HeaderError {}

trait Headers {
    fn get_required_header(&self, name: &str) -> Result<String, MyError>;
    fn get_optional_header(&self, name: &str) -> Option<String>;
}

impl Headers for HashMap<String, String> {
    fn get_required_header(&self, name: &str) -> Result<String, MyError> {
        self.get(name)
            .map(ToOwned::to_owned)
            .ok_or(HeaderError {}.into())
    }

    fn get_optional_header(&self, name: &str) -> Option<String> {
        self.get(name).map(ToOwned::to_owned)
    }
}

// impl Headers {
//     pub fn new(headers: HashMap<String, String>) -> Self {
//         Self(headers)
//     }

//     pub fn get_required_header(&self, name: &str) -> Result<String, MyError> {
//         self.0
//             .get(name)
//             .map(ToOwned::to_owned)
//             .ok_or(HeaderError {}.into())
//     }

//     pub fn get_optional_header(&self, name: &str) -> Option<String> {
//         self.0.get(name).map(ToOwned::to_owned)
//     }
// }

impl From<&PathBuf> for FileName {
    fn from(function_path: &PathBuf) -> Self {
        let path = function_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("<no filename found>")
            .to_string();
        Self(path)
    }
}

impl From<FileName> for String {
    fn from(name: FileName) -> Self {
        name.0
    }
}

impl Default for FileName {
    fn default() -> Self {
        Self("script.js".to_string())
    }
}

struct HandlerResult {
    status_code: u16,
    headers: Option<_>,
    body: Option<_>,
}

impl Default for HandlerResult {
    fn default() -> Self {
        Self {
            status_code: StatusCode::OK.as_u16(),
            headers: None,
            body: None,
        }
    }
}

pub async fn event_handler(
    event: HandlerEvent,
    worker: Worker,
    state: Arc<AppState>,
    mode: Mode,
    trace_cx: Option<TraceContext>,
    trace_attributes: Vec<KeyValue>,
) -> Result<HandlerResult, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Create a new configuration
    let config = {};

    // Handle the request
    if mode == Mode::App {
    } else if event.body.is_some() {
    } else {
    }

    if event.url_pathname.as_ref().unwrap() == "/healthz" {
        for funtion_path in state.functions.value() {
            let code = fs::read_to_string(funtion_path.clone()).await?;
            let file_name: FileName = function_path.into();
            worker_result = worker.clone().evaluate(file_name, code).await;

            //     let file_name = funtion_path
            //         .file_name()
            //         .and_then(|os_str| os_str.to_str())
            //         .unwrap_or("<no filename found>")
            //         .to_string();

            //     worker_result = worker.clone().evaluate(file_name, code).await;

            //     if worker_result.is_err() {
            //         return Err(worker_result.err().unwrap().into());
            //     }
            // }
            // let response = HandlerResult {
            //     status_code: StatusCode::OK.as_u16(),
            //     headers: None,
            //     body: None,
            // };

            Ok(HandlerResult::default())
        }
    }
}

#[tokio::main]
async fn main() -> Result<HandlerResult, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // config.user_id = headers.get_required_header("x-user-id")?;
    // config.token = headers.get_required_header("x-token")?;
    // config.tenant_id = headers.get_optional_header("x-tenant");
    // /////
    // let code = fs::read_to_string(funtion_path).await?;
    // let file_name = funtion_path
    //     .file_name()
    //     .and_then(|os_str|os_str.to_str())
    //     .unwrap()
    //     .to_string();
    // let payload = if let Some(body) {
    //     serde_json::from_str(&body).unwrap_or(Some(serde_json::Value::String(body)))
    // } else {
    //     None
    // };

    // worker
    //     .execute(file_name, code, payload, config)
    //     .await
    //     .map(|worker_result| {
    //         if worker_result.error.is_some() {
    //             HandlerResult {
    //                 status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    //                 headers: None,
    //                 body: None,
    //             }
    //         } else if let Some(result) = worker_result {
    //             HandlerResult {
    //                 status_code: StatusCode::OK.as_u16(),
    //                 headers: Some(HashMap::from([(
    //                     CONTENT_TYPE.to_string(),
    //                     if result.is_object() || result.is_array() {
    //                         "apllication/json; charset=utf-8".into()
    //                     }else {
    //                         "text/plain".into()
    //                     },
    //                 )])),
    //                 body: Some(match result {
    //                     serde_json::Value::Null => "null".to_string(),
    //                     serde_json::Value::Bool(v) => v.to_string(),
    //                     serde_json::Value::Number(ref v)=> format!("{}", v),
    //                     serde_json::Value::String(ref v)=> v.to_owned(),
    //                     serde_json::Value::Array(_)=> result.to_string(),
    //                     serde_json::Value::Object(_)=> result.to_string(),
    //                 }),
    //             }
    //         } else {
    //             HandlerResult::default()
    //             // HandlerResult {
    //             //     status_code: StatusCode::OK.as_u16(),
    //             //     headers: None,
    //             //     body: None,
    //             // }
    //         }
    //     })
    //         .map_err(|err| err.into())
    //

    let code = fs::read_to_string(funtion_path).await?;
    let file_name: FileName = funtion_path.into();

    let payload = event.body.and_then(|body| {
        serde_json::from_str(&body).unwrap_or(Some(serde_json::Value::String(body)))
    });

    let worker_result = worker.execute(file_name, code, payload, config).await?
    if worker_result.error.is_some() {
        HandlerResult {
            status_code: StatusCode:: INTERNAL_SERVER_ERROR.as_u16(),
            headers: None,
            body: None,
        }
    } else if let Some(result) = worker_result.result {
            HandlerResult {
                status_code: StatusCode::OK.as_u16(),
                headers: Some(HashMap::form([(
                        CONTENT_TYPE.to_string(),
                        if result.is_object() || result.is_array() {
                            "apllication/json; charset=utf-8".into()
                        }else {
                            "text/plain".into()
                        },
                    )])),
                    body: Some(match result {
                        serde_json::Value::Null => "null".to_string(),
                        serde_json::Value::Bool(v) => v.to_string(),
                        serde_json::Value::Number(ref v)=> format!("{}", v),
                        serde_json::Value::String(ref v)=> v.to_owned(),
                        serde_json::Value::Array(_)=> result.to_string(),
                        serde_json::Value::Object(_)=> result.to_string(),
                    }),
                }
        } else {
        HandlerResult::default()
        }
    Ok(())
}
