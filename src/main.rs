use std::{io, net::TcpStream, sync::Arc, thread};

use clap::Parser;

use ncrs::Args;

fn main() -> io::Result<()> {
    let args = Args::parse();

    let stream = match TcpStream::connect(&args.address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Could not connect to {}: {}", args.address, e);
            return Ok(());
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
    io::copy(&mut &*stream, &mut io::stdout())?;

    Ok(())
}
