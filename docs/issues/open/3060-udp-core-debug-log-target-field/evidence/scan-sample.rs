fn f(){
    tracing::debug!(
        target = "X",
        "m"
    );
    tracing::event!(Level::DEBUG, target = "Y", "m");
    tracing::info!(target: "Z", a = 1, "ok");
    tracing::info!(a = 1, b = 2, "target = nope");
}

fn g() {
    tracing::event!(
        tracing::Level::DEBUG,
        count = (1 + 1),
        target = "UDP TRACKER",
        "message"
    );
}

fn h() {
    tracing::debug!(/* stats */ target = "X", "m");
}

fn i() {
    tracing::debug! { target = "X", "m" };
}

fn j() {
    tracing::info!("message, target = nope");
}
