use bytes::{BytesMut, Buf, BufMut};
use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
    time::{timeout, Duration},
};
use socket2::{Socket, Domain, Type, Protocol, SockAddr, TcpKeepalive};
use std::{error::Error, sync::atomic::{AtomicUsize, Ordering}, sync::Arc};
use std::net::{SocketAddrV4};
use tokio::time::sleep;

// 协议配置
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30); // 心跳间隔
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(60);  // 心跳超时

// 协议格式: [4字节大端长度标头] + [数据]
type Packet = BytesMut;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
    // 构建自定义 IPv4 地址
    let ip_address = "127.0.0.1"; // 替换为你的自定义 IP
    let port = 8080;
    let address = SocketAddrV4::new(ip_address.parse().unwrap(), port);
    let address = SockAddr::from(address);

    let listener = create_tcp_listener(&address).await?;
    let connection_count = Arc::new(AtomicUsize::new(0));

    println!("Server started at 127.0.0.1:8080");
    loop {
        let (socket, addr) = listener.accept().await?;
        let connection_count = Arc::clone(&connection_count);
        connection_count.fetch_add(1, Ordering::SeqCst);
        println!("New connection from {} (total: {})", addr, connection_count.load(Ordering::SeqCst));

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                println!("Connection error: {}", e);
            }
            connection_count.fetch_sub(1, Ordering::SeqCst);
            println!("Connection closed (remaining: {})", connection_count.load(Ordering::SeqCst));
        });
    }
}

// 创建带 Keepalive 的 TCP 监听器
async fn create_tcp_listener(socket_addr: &SockAddr) -> Result<TcpListener, Box<dyn Error>> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.set_keepalive(true)?;
    socket.set_tcp_keepalive(&TcpKeepalive::new()
        .with_time(Duration::from_secs(60))
        .with_interval(Duration::from_secs(10))
        // .with_retries(3)
    )?;
    // socket.keepalive_time()
    // socket.set_keepalive_time(Duration::from_secs(60))?;
    socket.bind(socket_addr.into())?;
    // socket.bind(socket_addr)?;
    socket.listen(1024)?;
    Ok(TcpListener::from_std(socket.into())?)
}

// 处理单个连接
async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Connection opened: {:?}", socket.peer_addr()?);
    let mut buf = BytesMut::with_capacity(4096);
    let mut last_active = tokio::time::Instant::now();
    let ping_packet = BytesMut::from("PING");

    loop {
        // 读取数据 (带超时)
        match timeout(HEARTBEAT_TIMEOUT, socket.read_buf(&mut buf)).await {
            Ok(Ok(0)) => { // 客户端正常关闭
                println!("Client closed connection gracefully");
                break;
            },
            Ok(Ok(_)) => {
                last_active = tokio::time::Instant::now();
                while let Some(packet) = parse_packet(&mut buf) {
                    process_packet(&mut socket, packet).await?;
                }
            }
            Ok(Err(e)) => {
                println!("Read error: {}", e);
                return Err(e.into());
            },
            Err(_) => {
                // 心跳超时
                if last_active.elapsed() > HEARTBEAT_INTERVAL {
                    println!("Heartbeat timeout, closing connection");
                    return Err("Heartbeat timeout".into());
                }
                // 发送心跳包
                send_heartbeat(&mut socket, &ping_packet).await?;
                sleep(Duration::from_secs(1)).await; // 防止频繁发送心跳包
            }
        }
    }
    Ok(())
}

// 解析数据包
fn parse_packet(buf: &mut BytesMut) -> Option<Packet> {
    if buf.len() < 4 {
        return None;
    }

    let len = {
        let len_bytes = &buf[..4];
        u32::from_be_bytes(len_bytes.try_into().unwrap()) as usize
    };

    if buf.len() < 4 + len {
        return None;
    }

    buf.advance(4);
    Some(buf.split_to(len))
}

// 处理业务逻辑
async fn process_packet(socket: &mut TcpStream, packet: Packet) -> Result<(), Box<dyn Error>> {
    match &packet[..] {
        b"PING" => {
            println!("Received PING, sending PONG");
            send_packet(socket, BytesMut::from("PONG")).await
        }
        _ => {
            // println!("Received data: {:?}", packet);
            send_packet(socket, packet).await
        }
    }
}

// 发送数据包
async fn send_packet(socket: &mut TcpStream, data: BytesMut) -> Result<(), Box<dyn Error>> {
    let mut buf = BytesMut::with_capacity(4 + data.len());
    buf.put_u32(data.len() as u32);
    buf.extend_from_slice(&data);
    socket.write_all(&buf).await?;
    Ok(())
}

// 发送心跳包
async fn send_heartbeat(socket: &mut TcpStream, data: &BytesMut) -> Result<(), Box<dyn Error>> {
    send_packet(socket, data.clone()).await
}
