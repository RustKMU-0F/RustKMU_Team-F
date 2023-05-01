use std::net::TcpListener;
use std::{io, thread};
use std::io::{Read, Write};

pub(crate) fn make_socket_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");
    let (mut client_1_socket, client_1_addr) = listener.accept()?;
    println!("Client 1 connected from {}", client_1_addr);

    let (mut client_2_socket, client_2_addr) = listener.accept()?;
    println!("Client 2 connected from {}", client_2_addr);

    let mut client_1_clone = client_1_socket.try_clone()?;
    let mut client_2_clone = client_2_socket.try_clone()?;
    let client_2 = thread::spawn(move || {
        let mut buffer = [0u8; 4];
        loop {
            match client_1_socket.read_exact(&mut buffer) {
                Ok(_) => {
                    // println!("{:?}", buffer);
                    client_2_socket.write_all(&buffer).unwrap();
                },
                Err(e) => {
                    println!("Failed to receive data from server: {}", e);
                    break;
                }
            }
        }
    });
    let client_1 = thread::spawn(move || {
        let mut buffer = [0u8; 4];
        loop {
            match client_2_clone.read_exact(&mut buffer) {
                Ok(_) => {
                    // println!("{:?}", buffer);
                    client_1_clone.write_all(&buffer).unwrap();
                },
                Err(e) => {
                    println!("Failed to receive data from server: {}", e);
                    break;
                }
            }
        }
    });
    client_1.join().unwrap();
    client_2.join().unwrap();
    Ok(())
}