use tokio::{
    io::{
        AsyncBufReadExt,
        AsyncWriteExt,
        BufReader
    },
    net::TcpListener, sync::broadcast
};
use colored::Colorize;



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("[::]:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10); // on peut changer. c'est le nombre de receveurs;

    loop {
        let (mut socket, my_addr) = listener.accept().await.unwrap();


        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
        
            let mut reader = BufReader::new(reader);
        
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), my_addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (message, sender_addr) = result.unwrap();
                        if sender_addr != my_addr {
                            let info = format!("Message venant de {}\n", sender_addr.to_string().red());
                            writer.write_all(info.as_bytes()).await.unwrap();
                            let message = format!("{}", message.red());
                            writer.write_all(message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}