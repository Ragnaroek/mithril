extern crate mithril;
extern crate serde;
extern crate serde_json;

use std::sync::mpsc::{channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration};

use mithril::stratum::stratum_data;
use mithril::stratum;

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
fn test_ser_login_json() {
    let login_req = stratum_data::LoginRequest {
        id: 1,
        method: "login".to_string(),
        params: stratum_data::LoginParams {
            login: "foo".to_string(),
            pass: "bar".to_string()
        }
    };

    assert_eq!(serde_json::to_string(&login_req).unwrap(), "{\"id\":1,\"method\":\"login\",\"params\":{\"login\":\"foo\",\"pass\":\"bar\"}}");
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
fn test_parse_line_dispatch_result_error() {
    let (tx, rx) = channel();
    let miner_id_mutex = Arc::new(Mutex::new(Option::None));

    let line = r#"{"id":1,"jsonrpc":"2.0","error":{"code":-1,"message":"Low difficulty share"}}"#;

    let mutex_thread = miner_id_mutex.clone();
    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &tx, &mutex_thread);
    });

    let result = rx.recv().unwrap();

    match result {
        stratum::StratumAction::Error{err} => assert_eq!(err, "error received: Low difficulty share (code -1, raw json {\"id\":1,\"jsonrpc\":\"2.0\",\"error\":{\"code\":-1,\"message\":\"Low difficulty share\"}})".to_string()),
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_result_initial_job() {

    let (tx, rx) = channel();
    let miner_id_mutex = Arc::new(Mutex::new(Option::None));

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

    let mutex_thread = miner_id_mutex.clone();
    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &tx, &mutex_thread);
    });

    let result = rx.recv().unwrap();

    let miner_id_guard = &*miner_id_mutex.lock().unwrap();
    assert_eq!(miner_id_guard.clone().unwrap(), "930717205908149");

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
    let miner_id_mutex = Arc::new(Mutex::new(Option::None));
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
        stratum::parse_line_dispatch_result(line, &tx, &miner_id_mutex);
    });

    let result = rx.recv().unwrap();

    match result {
        stratum::StratumAction::Error{..} => assert!(true),
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_unknown_method() {

    let (tx, rx) = channel();
    let miner_id_mutex = Arc::new(Mutex::new(Option::None));

    let line = r#"{
        "jsonrpc":"2.0",
        "method":"UNKNOWN",
        "params":{
            "arg": "unknown"
        }}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &tx, &miner_id_mutex);
    });

    let result = rx.recv().unwrap();

    match result {
        stratum::StratumAction::Error{..} => assert!(true),
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_job_method() {

    let (tx, rx) = channel();
    let miner_id_mutex = Arc::new(Mutex::new(Option::Some("test_miner_id".to_string())));

    let line = r#"{
        "jsonrpc":"2.0",
        "method":"job",
        "params":{
            "blob":"0606fcb29bcf051b9c7bfc60c98885de404ef48f721f09b8f51d37faf280470880bd120d4e9e0500000000577192c076fed53a24372bc43a3bed1d448a061ad06a262ac5e7f6803a28ccc705",
            "job_id":"878440772206522",
            "target":"169f0200"
        }}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &tx, &miner_id_mutex);
    });

    let result = rx.recv().unwrap();
    match result {
        stratum::StratumAction::Job{miner_id, blob, job_id, target} => {
            assert_eq!(miner_id, "test_miner_id");
            assert_eq!(blob, "0606fcb29bcf051b9c7bfc60c98885de404ef48f721f09b8f51d37faf280470880bd120d4e9e0500000000577192c076fed53a24372bc43a3bed1d448a061ad06a262ac5e7f6803a28ccc705");
            assert_eq!(job_id, "878440772206522");
            assert_eq!(target, "169f0200");
        },
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_job_method_missing_miner_id() {

    let (tx, rx) = channel();
    let miner_id_mutex = Arc::new(Mutex::new(Option::None));

    let line = r#"{
        "jsonrpc":"2.0",
        "method":"job",
        "params":{
            "blob":"0606fcb29bcf051b9c7bfc60c98885de404ef48f721f09b8f51d37faf280470880bd120d4e9e0500000000577192c076fed53a24372bc43a3bed1d448a061ad06a262ac5e7f6803a28ccc705",
            "job_id":"878440772206522",
            "target":"169f0200"
        }}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &tx, &miner_id_mutex);
    });

    let result = rx.recv().unwrap();

    match result {
        stratum::StratumAction::Error{err} => assert_eq!(err, "miner_id not available for first mining job (login failed previously, this is a bug)"),
        _ => assert!(false, "Wrong result returned: {:?}", result)
    }
}

#[test]
fn test_parse_line_dispatch_job_ok_result_share_submit() {

    let (tx, rx) = channel();
    let miner_id_mutex = Arc::new(Mutex::new(Option::Some("test_miner_id".to_string())));

    let line = r#"{"id":1,"jsonrpc":"2.0","error":null,"result":{"status":"OK"}}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &tx, &miner_id_mutex);
    });

    let result = rx.recv().unwrap();
    assert_eq!(stratum::StratumAction::Ok, result);
}

#[test]
fn test_parse_line_dispatch_keepalive() {
    let (tx, rx) = channel();
    let miner_id_mutex = Arc::new(Mutex::new(Option::Some("test_miner_id".to_string())));

    let line = r#"{"id":1,"jsonrpc":"2.0","error":null,"result":{"status":"KEEPALIVED"}}"#;

    thread::spawn(move || {
        stratum::parse_line_dispatch_result(line, &tx, &miner_id_mutex);
    });

    let result = rx.recv().unwrap();
    assert_eq!(stratum::StratumAction::KeepAliveOk, result);
}

#[test]
fn test_start_tick_thread_shutdown() {
    let (stop_tx, stop_rx) = channel();
    let (rx, hnd) = stratum::start_tick_thread(Duration::from_secs(60), stop_rx);
    stop_tx.send(()).expect("sending stop signal");
    let result = rx.recv().expect("stop signal");
    assert_eq!(stratum::Tick::Stop, result);
    hnd.join().expect("tick thread join");
}
