mod common;

use crate::common::{runtime, test_bind};
use common::{test_connect, ECHO_SERVER_ADDR, PROXY_ADDR};
use tokio_socks::{
    tcp::{Socks5Listener, Socks5Stream},
    Result,
};

#[test]
fn connect_no_auth() -> Result<()> {
    let mut runtime = runtime().lock().unwrap();
    let conn = runtime.block_on(Socks5Stream::connect(PROXY_ADDR, ECHO_SERVER_ADDR))?;
    runtime.block_on(test_connect(conn))
}

#[test]
fn bind_no_auth() -> Result<()> {
    let bind = {
        let mut runtime = runtime().lock().unwrap();
        runtime.block_on(Socks5Listener::bind(PROXY_ADDR, ECHO_SERVER_ADDR))
    }?;
    test_bind(bind)
}
