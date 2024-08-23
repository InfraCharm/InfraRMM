use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{Sender, self};
use tokio::time::{self, Duration};
use tokio::sync::Mutex;

type Clients = Arc<Mutex<HashMap<usize, (String, Sender<String>)>>>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("0.0.0.0:8888").await?;

    println!("Server active on port 8888...");

    let clients_ref = clients.clone();
    tokio::spawn(async move {
        handle_ui_update(clients_ref).await;
    });

    let clients_ref = clients.clone();
    tokio::spawn(async move {
        handle_cli(clients_ref).await;
    });

    let mut client_id = 0;

    loop {
        let (socket, _) = listener.accept().await?;
        let clients_ref = clients.clone();
        client_id += 1;
        tokio::spawn(async move {
            handle_client(client_id, socket, clients_ref).await;
        });
    }
}

async fn handle_client(
    id: usize,
    mut socket: TcpStream,
    clients: Clients,
) -> std::io::Result<()> {
    let (tx, mut rx) = mpsc::channel::<String>(32);
    let mut buffer = vec![0; 1024];
    let n = socket.read(&mut buffer).await?;
    let hostname = String::from_utf8_lossy(&buffer[..n]).to_string();
    clients.lock().await.insert(id, (hostname.clone(), tx));
    loop {
        tokio::select! {
            result = socket.read(&mut buffer) => {
                if result.unwrap() == 0 {
                    break;
                }
                let response = String::from_utf8_lossy(&buffer).to_string();
                println!("{}: {}", hostname, response);
            }
            Some(command) = rx.recv() => {
                socket.write_all(command.as_bytes()).await?;
                println!("Command sent to {}: {}", hostname, command);
            }
        }
    }
    println!("{} lost connection", hostname);
    clients.lock().await.remove(&id);
    Ok(())
}

async fn handle_ui_update(clients: Clients) {
    let mut interval = time::interval(Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        clear_screen();
        display_ui(&clients).await;
    }
}

async fn handle_cli(clients: Clients) {
    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                clear_screen();
                display_ui(&clients).await;
            }
            _ = async {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_string();

                if input.is_empty() {
                    return;
                }

                if input == "All" {
                    let command = get_command_from_user();
                    let clients = clients.lock().await;
                    for (_, (_, tx)) in clients.iter() {
                        let _ = tx.send(command.clone()).await;
                    }
                    println!("Command sent to all clients: {}", command);
                } else if let Ok(num) = input.parse::<usize>() {
                    let clients = clients.lock().await;
                    if let Some((hostname, tx)) = clients.get(&num) {
                        let command = get_command_from_user();
                        let _ = tx.send(command.clone()).await;
                        println!("Command sent to {}: {}", hostname, command);
                    } else {
                        println!("Invalid client number.");
                    }
                } else {
                    println!("Invalid input.");
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;
                clear_screen();
                display_ui(&clients).await;
            } => {},
        }
    }
}


async fn display_ui(clients: &Clients) {
    let clients_locked = clients.lock().await;
    let mut client_list: Vec<_> = clients_locked.iter().collect();
    client_list.sort_by_key(|(id, _)| *id);

    println!("=====================================");
    println!("    InfraCharm Management Network");
    println!("=====================================");
    println!("Clients Connected: {}", clients_locked.len());

    for (index, (_id, (hostname, _))) in client_list.iter().enumerate() {
        println!("{}: {}", index + 1, hostname);
    }

    println!("All");
    println!();
    println!("Select a client number, 'All', or an option:");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_command_from_user() -> String {
    println!("Enter command to execute:");
    let mut command = String::new();
    std::io::stdin().read_line(&mut command).unwrap();
    command.trim().to_string()
}
