use tokio::task::spawn;
use tokio::time::sleep;
use tokio::io::{Result, Error, ErrorKind, AsyncWriteExt, BufReader, AsyncBufReadExt, copy};
use std::time::Duration;
use tokio::sync::mpsc::unbounded_channel as channel;
use tokio::fs::File;
use tokio::net::{TcpListener, TcpStream};

pub async fn server_tokio(host: &str, port: u16) {
    let (dispatch_sender, mut dispatch_receiver) = channel::<DispatchMessage>();

    let listener = TcpListener::bind((host, port)).await.unwrap();
    let dispatch_sender1 = dispatch_sender.clone();
    let _accept_loop = spawn(async move {
        while let Ok((stream, _addr)) = listener.accept().await {
            dispatch_sender1.send(DispatchMessage::Connected(stream)).unwrap();
        }
    });
    println!("server started at http://{}:{}/ serving files in {:?}", host, port, std::env::current_dir().unwrap_or_default());

    while let Some(dispatch_message) = dispatch_receiver.recv().await {
        match dispatch_message {
            DispatchMessage::Connected(stream) => {
                let dispatch_sender = dispatch_sender.clone();
                spawn(async move {
                    if let Ok(RequestResult::Quit) = handle_connection(stream).await {
                        dispatch_sender.send(DispatchMessage::Quit).unwrap();
                    }
                });
            }
            DispatchMessage::Quit => { break; }
        }
    }

}

#[derive(Debug)]
enum DispatchMessage {
    Connected(TcpStream),
    Quit,
}

enum RequestResult {
    Ok,
    Quit,
}

async fn handle_connection(mut stream: TcpStream) -> Result<RequestResult> {
    let mut str = String::new();
    BufReader::new(&mut stream).read_line(&mut str).await?;

    let strsubs: Vec<_> = str.split(" ").collect();
    if strsubs.len() < 3 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    let _method = strsubs[0];
    let path = strsubs[1];

    let (path, query) = match path.find("?") {
        Some(pos) => (&path[..pos], &path[(pos+1)..]),
        None => (path, ""),
    };

    if query == "sleep" {
        sleep(Duration::new(4, 0)).await;
    }

    if path == "/" {
        stream.write("HTTP/1.1 200 OK\r\n\r\n<html><body>Welcome</body></html>".as_bytes()).await?;
    } else {
        let relative_path = match path.strip_prefix("/") {
            Some(p) => p,
            None => path,
        };
        match File::open(relative_path).await {
            Ok(mut f) => {
                stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).await?;
                copy(&mut f, &mut stream).await?;
            }
            Err(_err) => {
                stream.write(format!("HTTP/1.1 404 NOT FOUND\r\n\r\n<html><body>Not Found {}</body></html>", path).as_bytes()).await?;
            }
        }
    }
    stream.flush().await?;

    if query == "quit" {
        return Ok(RequestResult::Quit);
    }
    return Ok(RequestResult::Ok);
}
