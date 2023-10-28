//! Handles being a server. Take a look at the [`run_server`] function for more details.

use std::{io, net::TcpListener, sync::Arc, thread};

/// Handles being a server. This should be the code run when the `--server` flag is passed.
pub fn run_server(address: &str) {
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Could not bind to {}: {}", address, e);
            return;
        }
    };

    let (stream, _) = match listener.accept() {
        Ok((stream, _)) => (stream, true),
        Err(e) => {
            eprintln!("Could not accept connection: {}", e);
            return;
        }
    };

    // Since `io::copy` is blocking, we use a separate thread to copy from stdin to the stream and
    // use the main thread to copy from the stream to stdout.
    let stream = Arc::new(stream);
    let stream_clone = Arc::clone(&stream);
    thread::spawn(move || -> io::Result<()> {
        io::copy(&mut io::stdin(), &mut &*stream_clone)?;
        Ok(())
    });
    io::copy(&mut &*stream, &mut io::stdout()).unwrap();
}
