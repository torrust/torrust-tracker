use rand::{thread_rng, Rng};

pub fn ephemeral_random_port() -> u16 {
    // todo: this may produce random test failures because two tests can try to bind the same port.
    // We could create a pool of available ports (with read/write lock)
    let mut rng = thread_rng();
    rng.gen_range(49152..65535)
}
