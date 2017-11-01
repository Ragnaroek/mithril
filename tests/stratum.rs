extern crate mithril;
extern crate serde;
extern crate serde_json;

use std::sync::mpsc::{channel};
use std::thread;

use mithril::stratum::stratum_data;
use mithril::stratum::stratum;

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
fn test_parse_method_with_method_field() {
    let method : stratum_data::Method = serde_json::from_str("{\"jsonrpc\":\"2.0\",\"method\":\"job\",\"params\":{}}").unwrap();
    assert_eq!(method.method, "job".to_string());
}

#[test]
fn test_parse_method_without_method_field() {
    let result : Result<stratum_data::Method, serde_json::Error> = serde_json::from_str("{\"jsonrpc\":\"2.0\",\"params\":{}}");
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_parse_line_dispatch_result_initial_job() {

    let (tx, rx) = channel();

    let line = r#"{
        "id":1,
        "jsonrpc":"2.0",
        "error":null,
        "result":{
            "id": "930717205908149",
            "job":{
                "blob":"0606fdb09bcf056875870cb2750c2db9d179d1e8cf22a2c89e4e43bc4aaaabda227e2fd1ad14f2000000007e6fe370e8ec9594b111fe7fa47d9a0f2efc52454d24fc610f59acbb399d098806",
                "job_id":"738478949642740",
                "target":"169f0200"
            },
            "status":"OK"
        }}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &vec![tx]);
    });

    let result = rx.recv().unwrap();

    match result {
        stratum::StratumAction::Job{miner_id, blob, job_id, target} => {
            assert_eq!(miner_id, "930717205908149");
            assert_eq!(blob, "0606fdb09bcf056875870cb2750c2db9d179d1e8cf22a2c89e4e43bc4aaaabda227e2fd1ad14f2000000007e6fe370e8ec9594b111fe7fa47d9a0f2efc52454d24fc610f59acbb399d098806");
            assert_eq!(job_id, "738478949642740");
            assert_eq!(target, "169f0200");
        },
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_result_initial_job_with_non_ok_result() {

    let (tx, rx) = channel();

    //Only difference here is that status is != OK
    let line = r#"{
        "id":1,
        "jsonrpc":"2.0",
        "error":null,
        "result":{
            "id": "930717205908149",
            "job":{
                "blob":"0606fdb09bcf056875870cb2750c2db9d179d1e8cf22a2c89e4e43bc4aaaabda227e2fd1ad14f2000000007e6fe370e8ec9594b111fe7fa47d9a0f2efc52454d24fc610f59acbb399d098806",
                "job_id":"738478949642740",
                "target":"169f0200"
            },
            "status":"NOT_OK"
        }}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &vec![tx]);
    });

    let result = rx.recv().unwrap();

    match result {
        stratum::StratumAction::Error{err: _} => assert!(true),
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_unknown_method() {

    let (tx, rx) = channel();

    let line = r#"{
        "jsonrpc":"2.0",
        "method":"UNKNOWN",
        "params":{
            "arg": "unknown"
        }}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &vec![tx]);
    });

    let result = rx.recv().unwrap();

    match result {
        stratum::StratumAction::Error{err: _} => assert!(true),
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_job_method() {

    let (tx, rx) = channel();

    let line = r#"{
        "jsonrpc":"2.0",
        "method":"job",
        "params":{
            "blob":"0606fcb29bcf051b9c7bfc60c98885de404ef48f721f09b8f51d37faf280470880bd120d4e9e0500000000577192c076fed53a24372bc43a3bed1d448a061ad06a262ac5e7f6803a28ccc705",
            "job_id":"878440772206522",
            "target":"169f0200"
        }}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &vec![tx]);
    });

    let result = rx.recv().unwrap();
    match result {
        stratum::StratumAction::Job{miner_id, blob, job_id, target} => {
            assert_eq!(miner_id, ""); //TODO Find a way to transport the miner_id to a normal job. Until this is not fixed this test should fail.
            assert_eq!(blob, "0606fcb29bcf051b9c7bfc60c98885de404ef48f721f09b8f51d37faf280470880bd120d4e9e0500000000577192c076fed53a24372bc43a3bed1d448a061ad06a262ac5e7f6803a28ccc705");
            assert_eq!(job_id, "878440772206522");
            assert_eq!(target, "169f0200");
        },
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}
