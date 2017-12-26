extern crate serde;
extern crate serde_json;

/// For checking the method in the json content and parsing further
#[derive(Deserialize, Debug)]
pub struct Method {
    pub method: String
}

#[derive(Deserialize, Debug)]
pub struct ErrorDetails {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResult {
    pub error: ErrorDetails
}

#[derive(Deserialize)]
pub struct Job {
    pub blob: String,
    pub job_id: String,
    pub target: String,
}

#[derive(Deserialize)]
pub struct LoginResult {
    pub id: String,
    pub job: Job,
    pub status: String
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub id: u32,
    pub result: LoginResult
}

#[derive(Deserialize, Clone)]
pub struct OkResult {
    pub id: Option<String>,
    pub status: String
}

#[derive(Deserialize, Clone)]
pub struct OkResponse {
    pub id: u32,
    pub result: OkResult
}

#[derive(Deserialize)]
pub struct JobResponse {
    pub params: Job
}


#[derive(Serialize)]
pub struct LoginParams {
    pub login: String,
    pub pass: String
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub id: u32,
    pub method: String,
    pub params: LoginParams,
}

#[derive(Serialize)]
pub struct KeepAliveParams {
    pub id: String
}

#[derive(Serialize)]
pub struct KeepAliveRequest {
    pub id: u32,
    pub method: String,
    pub params: KeepAliveParams
}

#[derive(Serialize)]
pub struct SubmitParams {
    pub id: String,
    pub job_id: String,
    pub nonce: String,
    pub result: String
}

#[derive(Serialize)]
pub struct SubmitRequest {
    pub id: u32,
    pub method: String,
    pub params: SubmitParams
}

#[derive(Debug)]
pub struct Share {
    pub miner_id: String,
    pub job_id: String,
    pub nonce: String,
    pub hash: String
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub pool_address: String,
    pub wallet_address: String,
    pub pool_password: String
}
