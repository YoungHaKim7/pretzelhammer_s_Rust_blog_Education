use std::{
    collections::HashMap,
    fmt::{write, Display},
    fs,
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

// struct Headers(HashMap<String, String>);
struct HeaderError;
struct FileName(String);

struct AdhocStrategy {
    task: AdhocTask,
}

struct Executor {
    strategy: Box<dyn EventStrategy>,
    config: Config,
    worker: Worker,
}

impl Executor {
    pub fn new(strategy: Box<dyn EventStrategy>, config: Config, worker: Worker) -> Self {
        Self {
            strategy,
            config,
            worker,
        }
    }

    fn evaluate(&self) -> Result<HandlerResult, MyError> {
        self.strategy.health_check(&self.worker)
    }

    async fn execute(&self) -> Result<HandlerResult, MyError> {
        let file_name = self.strategy.file_name()?;
        let code = self.strategy.code()?;
        let payload = self.strategy.payload();

        let result = self
            .worker
            .clone()
            .execute(file_name.into(), code, payload, &self.config)
            .awati()?;

        self.strategy.parse_result(result)
    }
}

impl TryFrom<Rc<HandlerEvent>> for AdhocStrategy {
    type Error = MyError;

    fn try_from(value: Rc<HandlerEvent>) -> Result<Self, Self::Error> {
        let task = match event.body {
            Some(ref body) => serde_json::from_str::<AdhocTask>(body.as_str()),
            None => return Err(HeaderError.into()),
        }?;
        Ok(Self { task })
    }
}

impl Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, "Header not found")
    }
}

impl std::error::Error for HeaderError {}

impl EventStrategy for AdhocStrategy {
    fn file_name(&self) -> Result<FileName, MyError> {
        Ok(FileName::default())
    }

    fn code(&self) -> Result<String, MyError> {
        Ok(self.task.code.clone())
    }

    fn payload(&self) -> Option<Value> {
        self.task.payload.clone()
    }

    fn health_check(&self, _: &Worker) -> Result<HandlerResult, MyError> {
        Ok(HandlerResult::default())
    }

    fn parse_result(&self, worker_result: WorkerResult) -> Result<HandlerResult, MyError> {
        Ok(HandlerResult {
            status_code: if worker_result.error.is_some() {
                StatusCode::INTERNAL_SERVER_ERROR.as_u16()
            } else {
                StatusCode::OK.as_u16()
            },
            headers: Some(HashMap::from([(
                CONTENT_TYPE.to_string(),
                "application/json; charset=utf-8".into(),
            )])),
            body: Some(serde_json::to_string(&worker_result)),
        })
    }
}

trait Headers {
    fn get_required_header(&self, name: &str) -> Result<String, MyError>;
    fn get_optional_header(&self, name: &str) -> Option<String>;
}

trait EventStrategy {
    fn file_name() -> FileName;
    fn code() -> Result<String, MyError>;
    fn payload() -> Option<Value>;
    fn health_check() -> Result<HandlerResult, MyError>;
    fn parse_result(result: WorkerResult) -> Result<HeadlerResult, MyError>;
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

    let worker_result = worker.execute(file_name, code, payload, config).await?;
    if worker_result.error.is_some() {
        HandlerResult {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
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
                } else {
                    "text/plain".into()
                },
            )])),
            body: Some(match result {
                serde_json::Value::Null => "null".to_string(),
                serde_json::Value::Bool(v) => v.to_string(),
                serde_json::Value::Number(ref v) => format!("{}", v),
                serde_json::Value::String(ref v) => v.to_owned(),
                serde_json::Value::Array(_) => result.to_string(),
                serde_json::Value::Object(_) => result.to_string(),
            }),
        }
    } else {
        HandlerResult::default()
    }
    Ok(())
}
