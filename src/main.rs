use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use clap::Parser;

use ncrs::Args;

fn main() {
    let args = Args::parse();

    if args.server {
        do_server(args)
    } else {
        do_client(args)
    }
}

fn do_server(args: Args) {
    let listener = match TcpListener::bind(&args.address) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Could not bind to {}: {}", args.address, e);
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

fn do_client(args: Args) {
    let stream = match TcpStream::connect(&args.address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Could not connect to {}: {}", args.address, e);
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
