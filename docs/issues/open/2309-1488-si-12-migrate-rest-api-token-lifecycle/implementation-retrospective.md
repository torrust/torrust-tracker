# Implementation Retrospective

## Runtime Outcome Propagation

Independent review found that the token-aware REST runtime logged Axum serving
errors but returned successful task completion. The runtime now returns a typed
`RuntimeError`; the component converts it to a failed `http_api` outcome only
after joining the drain controller. A deterministic returned-error test holds
the controller after cancellation observation, proving no premature outcome.

## Test Design Review

Server tests own token drain, legacy compatibility, and registration rollback.
Component tests own normal completion, returned-error, and panic outcome
semantics. The application test owns `JobManager` token propagation. Each test
keeps its causal state, production action, and observable result visible.
