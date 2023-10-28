//! Handles being a client. Take a look at the [`run`] function for more details.

use std::{io, net::TcpStream, sync::Arc, thread};

/// Handles being a client. This should be the code run when the `--server` flag is not passed.
pub fn run_client(address: &str) {
    let stream = match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Could not connect to {}: {}", address, e);
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
