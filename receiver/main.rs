use tokio::net::{TcpListener};
use tokio::spawn;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

// What does this do?
#[tokio::main]
async fn main () {
    println!("hello world");
    // What does unwrap do
    let listener = TcpListener::bind("0.0.0.0:9999").await.unwrap();


    loop {
        let Ok((mut socket, _)) = listener.accept().await else {
            eprintln!("Failed to accept client");
            continue;
        };
    
    // What does split do
    // What does async move do
    // What does Spawn do
    // What are readHalf and writeHalf?
    // Handling failure here
    spawn(async move {
        let (reader, writer) = socket.split();
    });

    }
    
}

// What do the Where statements do
async fn handle_connection<Reader, Writer>(
    reader: Reader,
    mut writer: Writer,
) -> std::io::Result<()>
where
    Reader: AsyncRead + Unpin,
    Writer: AsyncWrite + Unpin,
{
    let mut line = String::new();
    let mut reader = BufReader::new(reader);

    // Walk through this and talk about what we are reading vs writing
    loop {
        if let Ok(bytes_read) = reader.read_line(&mut line).await {
            println!("Message : {}", &line);
            println!("Bytes to read: {}", bytes_read);
            if bytes_read == 0 {
                break Ok(());
            }
            writer
                .write_all(format!("Thanks for your message.\r\n").as_bytes())
                .await
                .unwrap();
        }
        line.clear();
    }
}

// Tokio test vs using rust test
// How to print during rust tests
#[tokio::test]
 // what does this do?
async fn test_main(){
        let reader = tokio_test::io::Builder::new()
        .read(b"Hi there\r\n")
        .read(b"How are you doing?\r\n")
        .build();
    let writer = tokio_test::io::Builder::new()
        .write(b"Thanks for your message.\r\n")
        .write(b"Thanks for your message.\r\n")
        .build();
    let _ = handle_connection(reader, writer).await;
}


