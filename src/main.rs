extern crate byteorder;
extern crate rand;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use rand::Rng;
use std::str;
mod wion_comm;

//use structs::{Header, BroadcastResp, head_parse, dump_packet_header, recv_msg, parse_broadcast,
//dump_packet_broadcast};
use std::thread;
use std::time;

fn main() {
    //let mut rng = rand::thread_rng();
    while(false) {
    let bind_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let bind_port: u16 = 3000;
    let ip_addr_listen_on = SocketAddr::new(bind_ip, bind_port);
    let socket = wion_comm::open_socket(ip_addr_listen_on);
    let send_broadcast_sock = wion_comm::broadcast_setup();
    wion_comm::send_broadcast(&send_broadcast_sock);
    thread::sleep(time::Duration::from_millis(5000));
    wion_comm::send_broadcast(&send_broadcast_sock);
    thread::sleep(time::Duration::from_millis(5000));
}
    //let ip: std::net::Ipv4Addr = "127.0.0.1".parse().unwrap();
    //let dev_ip = std::net::Ipv4Addr::from_str("127.0.0.1");
    send_switch_toggle(true, "192.168.0.237");
    thread::sleep(time::Duration::from_millis(5000));
    send_switch_toggle(false, "192.168.0.237");
    thread::sleep(time::Duration::from_millis(5000));
    //listen and send commands
    //let (src_address, readcount, buf) = read_msg(socket); // blocks!

    //println!("sender: {}, Recd Bytes: {}", src_address.ip(), readcount);
    //println!("bte0: {:?}", buf);
    //recv_msg(src_address, readcount, buf);

}

fn send_switch_toggle(switch: bool, device_ip: &str) {
    let ip: std::net::IpAddr = device_ip.parse().unwrap();
    let bind_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let ip_addr_listen_on = SocketAddr::new(bind_ip, 3333);
    let socket = wion_comm::open_socket(ip_addr_listen_on);
    let dst = SocketAddr::new(ip, 80);
    println!{"dev ip: {:?}", device_ip};
    wion_comm::send_basic_cmd(&socket, 327702, 1, switch as u8, &dst )
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
