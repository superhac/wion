extern crate byteorder;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str;
mod wion_comm;
//use structs::{Header, BroadcastResp, head_parse, dump_packet_header, recv_msg, parse_broadcast,
//dump_packet_broadcast};
use std::thread;
use std::time;

fn main() {
    let bind_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let bind_port: u16 = 3000;
    let ip_addr_listen_on = SocketAddr::new(bind_ip, bind_port);
    let socket = wion_comm::open_socket(ip_addr_listen_on);
    let send_broadcast_sock = wion_comm::broadcast_setup();

    //listen and send commands
    //let (src_address, readcount, buf) = read_msg(socket); // blocks!

    //println!("sender: {}, Recd Bytes: {}", src_address.ip(), readcount);
    //println!("bte0: {:?}", buf);
    //recv_msg(src_address, readcount, buf);
    send_broadcast(&send_broadcast_sock);
    thread::sleep(time::Duration::from_millis(5000));
    send_broadcast(&send_broadcast_sock);
    thread::sleep(time::Duration::from_millis(5000));
}

fn send_broadcast(socket: &UdpSocket) {

    let mut buf = [0;128];
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 25);
    // You need these bytes starting at offset 24 in the 128 byte packet to receive responses.
    buf[24] = 0xE0;
    buf[25] = 0x07;
    buf[26] = 0x06;
    buf[27] = 0x07;
    buf[28] = 0x07;
    buf[29] = 0xE6;
    socket.set_broadcast(true);
    socket.send_to(&buf, broadcast_addr);
}

fn read_msg(socket: UdpSocket) -> (SocketAddr, usize, Vec<u8> ) {
    // 1500 byte buffer for socket read
    let mut buf = vec![0;1500];
    match socket.recv_from(&mut buf) {
        Ok( (rcount,src_ip) ) =>  {
            let buf = buf[0..rcount].to_vec(); // trim buf to actual recieved bytes
            return (src_ip, rcount, buf);
        },
        Err(err) => panic!("recv error: {}", err)
    }
}
