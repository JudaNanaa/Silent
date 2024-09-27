pub mod helper;

// use tokio::{
//     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
//     net::{TcpListener, TcpStream},
//     sync::broadcast,
// };
// use colored::Colorize;

// #[tokio::main]
// async fn main() {
//     let remote_addr = "[2001:861:3f04:4310:9fa:e317:11f6:fc40]:8080"; // Remplace par l'IP publique de l'autre ordinateur

//     // Partie serveur
//     let listener = TcpListener::bind("[::]:8080").await.unwrap(); // Écoute sur toutes les interfaces
//     println!("Serveur en écoute sur [::]:8080");

//     let (tx, _rx) = broadcast::channel(10); // Le nombre de récepteurs

//     loop {
//         let (mut socket, my_addr) = listener.accept().await.unwrap();
//         println!("Connexion acceptée de {:?}", my_addr);

//         let tx = tx.clone();
//         let mut rx = tx.subscribe();

//         tokio::spawn(async move {
//             let (reader, mut writer) = socket.split();
//             let mut reader = BufReader::new(reader);
//             let mut line = String::new();
//             loop {
//                 tokio::select! {
//                     result = reader.read_line(&mut line) => {
//                         if result.unwrap() == 0 {
//                             break; // Connexion fermée
//                         }
//                         tx.send((line.clone(), my_addr)).unwrap();
//                         line.clear();
//                     }
//                     result = rx.recv() => {
//                         let (message, sender_addr) = result.unwrap();
//                         if sender_addr != my_addr {
//                             let info = format!("Message venant de {}\n", sender_addr.to_string().red());
//                             writer.write_all(info.as_bytes()).await.unwrap();
//                             let message = format!("{}", message.red());
//                             writer.write_all(message.as_bytes()).await.unwrap();
//                         }
//                     }
//                 }
//             }
//         });
//     }

//     // Partie client
//     match TcpStream::connect(remote_addr).await {
//         Ok(mut socket) => {
//             println!("Connecté à {}", remote_addr);
//             let (reader, mut writer) = socket.split();
//             let mut reader = BufReader::new(reader);
//             let mut line = String::new();

//             // Lis des lignes depuis stdin pour les envoyer à l'autre machine
//             let mut stdin = BufReader::new(tokio::io::stdin()).lines();
//             loop {
//                 tokio::select! {
//                     // Lire depuis stdin
//                     Ok(Some(msg)) = stdin.next_line() => {
//                         writer.write_all(msg.as_bytes()).await.unwrap();
//                     }
//                     // Recevoir des messages de l'autre machine
//                     result = reader.read_line(&mut line) => {
//                         if result.unwrap() == 0 {
//                             break; // Connexion fermée
//                         }
//                         println!("Message reçu: {}", line.green());
//                         line.clear();
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             println!("Impossible de se connecter à {}: {:?}", remote_addr, e);
//         }
//     }
// }