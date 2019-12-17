mod common;

use common::{runtime, test_bind, test_connect, ECHO_SERVER_ADDR, PROXY_ADDR};
use tokio_socks::{
    tcp::{Socks5Listener, Socks5Stream},
    Result,
};

#[test]
fn connect_username_auth() -> Result<()> {
    let mut runtime = runtime().lock().unwrap();
    let conn = runtime.block_on(Socks5Stream::connect_with_password(
        PROXY_ADDR,
        ECHO_SERVER_ADDR,
        "mylogin",
        "mypassword",
    ))?;
    runtime.block_on(test_connect(conn))
}

#[test]
fn bind_username_auth() -> Result<()> {
    let bind = {
        let mut runtime = runtime().lock().unwrap();
        runtime.block_on(Socks5Listener::bind_with_password(
            PROXY_ADDR,
            ECHO_SERVER_ADDR,
            "mylogin",
            "mypassword",
        ))
    }?;
    test_bind(bind)
}
