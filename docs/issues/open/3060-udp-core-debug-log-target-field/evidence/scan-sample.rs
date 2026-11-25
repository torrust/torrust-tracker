fn f(){
    tracing::debug!(
        target = "X",
        "m"
    );
    tracing::event!(Level::DEBUG, target = "Y", "m");
    tracing::info!(target: "Z", a = 1, "ok");
    tracing::info!(a = 1, b = 2, "target = nope");
}
