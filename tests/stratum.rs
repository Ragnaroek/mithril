extern crate mithril;
extern crate serde;
extern crate serde_json;

use mithril::stratum::stratum_data;

#[test]
fn test_deser_login_json() {
    let result = stratum_data::parse_login_response("{\"id\":1,\"jsonrpc\":\"2.0\",\"error\":null,\"result\":{\"id\":\"102368431002832\",\"job\":{\"blob\":\"0606a98bbece052423b128ae15482563d93d4c004034a051d8246236e6abd0cac766f0f54f28b90000000079c072d1fda66d20f0660fa9d374e2a940fcc938a46a837d6b21ff6515b5b58906\",\"job_id\":\"138133069709874\",\"target\":\"169f0200\"},\"status\":\"OK\"}}\n").unwrap();

    assert_eq!(result.id, 1);
    assert_eq!(result.result.id, "102368431002832");
    assert_eq!(result.result.status, "OK");
    assert_eq!(result.result.job.blob, "0606a98bbece052423b128ae15482563d93d4c004034a051d8246236e6abd0cac766f0f54f28b90000000079c072d1fda66d20f0660fa9d374e2a940fcc938a46a837d6b21ff6515b5b58906");
    assert_eq!(result.result.job.target, "169f0200");
    assert_eq!(result.result.job.job_id, "138133069709874");
}

#[test]
fn test_ser_submit_json() {
    let submit_req = stratum_data::SubmitRequest{
        id: 1,
        method: "submit".to_string(),
        params: stratum_data::SubmitParams {
            id: "id".to_string(),
            job_id: "job_id".to_string(),
            nonce: "nonce".to_string(),
            result: "result".to_string()
        }
    };

    assert_eq!(serde_json::to_string(&submit_req).unwrap(), "{\"id\":1,\"method\":\"submit\",\"params\":{\"id\":\"id\",\"job_id\":\"job_id\",\"nonce\":\"nonce\",\"result\":\"result\"}}");
}

#[test]
fn test_target_u64() {
    assert_eq!(stratum_data::target_u64(171798), 737869762948382);
}
