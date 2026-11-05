# Manual verification evidence - issue #2733

## Red: regression tests before the fix (temporary uncommitted run)

Tests added, `check_fn_with_client` unchanged. The `500` test fails in both packages; the `200` guard passes.

```text
test server::tests::it_should_report_an_error_with_the_status_when_the_health_endpoint_answers_a_failing_status ... FAILED
test server::tests::it_should_report_ok_when_the_health_endpoint_answers_a_success_status ... ok
  left: Ok("500 Internal Server Error")
 right: Err("500 Internal Server Error")
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 159 filtered out; finished in 0.00s
test server::tests::it_should_report_ok_when_the_health_endpoint_answers_a_success_status ... ok
test server::tests::it_should_report_an_error_with_the_status_when_the_health_endpoint_answers_a_failing_status ... FAILED
  left: Ok("500 Internal Server Error")
 right: Err("500 Internal Server Error")
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 49 filtered out; finished in 0.00s
```

## Green: after the fix

Commands: `cargo test -p torrust-tracker-axum-rest-api-server` and `cargo test -p torrust-tracker-axum-http-server`.

```text
test server::tests::it_should_report_ok_when_the_health_endpoint_answers_a_success_status ... ok
test server::tests::it_should_report_an_error_with_the_status_when_the_health_endpoint_answers_a_failing_status ... ok
test result: ok. 161 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.30s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test server::tests::it_should_report_ok_when_the_health_endpoint_answers_a_success_status ... ok
test server::tests::it_should_report_an_error_with_the_status_when_the_health_endpoint_answers_a_failing_status ... ok
test result: ok. 51 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s
test result: ok. 62 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.23s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Manual scenario (real tracker, development configuration, fixed build)

Ran `target/debug/torrust-tracker --config-toml-path share/default/config/tracker.development.sqlite3.toml` and fetched `curl localhost:1313/health_check`. Real endpoints answer `200`, so no regression (step 4 of the scenario):

```json
{
  "status": "Ok",
  "details": [
    {
      "info": "checking api health check at: http://0.0.0.0:1212/api/health_check",
      "result": {
        "Ok": "200 OK"
      }
    },
    {
      "info": "checking http tracker health check at: http://0.0.0.0:7070/health_check",
      "result": {
        "Ok": "200 OK"
      }
    },
    {
      "info": "checking http tracker health check at: http://0.0.0.0:7171/health_check",
      "result": {
        "Ok": "200 OK"
      }
    }
  ]
}
```

Steps 2-3 (a `500` endpoint in a real run) were not reproduced end to end: the real endpoint always answers `200` and a throwaway server cannot take the address the tracker already holds. The `500` case is covered by the regression tests, which drive the same `check_fn_with_client` code with a real `reqwest` client against a local TCP server answering `500 Internal Server Error`.
