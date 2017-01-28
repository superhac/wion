extern crate rand;
use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::io::{Read, Write};
use std;
use std::str;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;
use std::time;
use rand::Rng;

const messaging_port: u16 = 80;
const broadcast_port: u16 = 5888; // or 25

// Commands
const CMD_BASCI_MODIFY_SWITCH: u32 = 327702;
const CMD_BASCI_GET_SWITCH_STATUS: u32 = 327703;

//#[allow(dead_code)]

#[derive(Default)]
pub struct Header {
    pub cmd: u32,
    pub req_conn_id: u32,
    pub cmd_type: u16,
    pub version: [u8;6], //Convert to String
    pub model: [u8;32],  //Convert to String
    pub dev_name: [u8;32],  //Convert to Strorder rust
    pub serial: [u8;32],  //Convert to String
    pub resp_status: u32,
    pub seq_counter: u32,
    pub unknown: u32,
    pub resp_conn_id: u32,
    pub operation: u8, // read or write? Field only on requests.  Field only seen on requests.
    pub rw_byte: u8, // what byte to read or write to?  Field only on requests.
}

#[allow(dead_code)]
pub struct BroadcastResp {
    pub unknown: u32,
    pub version: [u8;6], //string
    pub dev_model: [u8;32],   //string
    pub dev_name: [u8;32],  //string
    pub dev_serial: [u8;34],  //string. fixed size 34 bytes
    pub unknown2: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub ssid: [u8;64],  //string fixed size 64 bytes
    pub wifi_pass: [u8;64],  //string fixed size 64 bytes
    pub unknown5: u32,
    pub unknown6: u32,
    pub unknown7: u32,
    pub zipcode: [u8;12],
    pub p2pm: [u8;16],  //string ip
    pub p2ps: [u8;16], //string ip
    pub paw: [u8;16], //string ip
    pub unknown8: u32,
    pub unknown9: u32,
    pub unknown10: u32,
    pub unknown11: u32,
    pub unknown12: u32,
    pub unknown13: u32,
    pub unknown14: u32,
    pub unknown15: u32,
    pub unknown16: u32,
    pub unknown17: u32,
    pub unknown18: u32,
    pub unknown19: u32,
    pub dev_mac: [u8;18], //string
    pub dev_ip: [u8;18], //string
    pub dev_port: u32,
  }

  impl Default for BroadcastResp {
      fn default() -> Self {
          BroadcastResp {
              unknown: 0u32,
              version: [0;6],
              dev_model: [0;32],
              dev_name: [0;32],
              dev_serial: [0; 34],
              unknown2: 0u32,
              unknown3: 0u32,
              unknown4: 0u32,
              ssid: [0; 64],  //string
              wifi_pass: [0; 64],  //string
              unknown5: 0u32,
              unknown6: 0u32,
              unknown7: 0u32,
              zipcode: [0;12],
              p2pm: [0;16],  //string ip
              p2ps: [0;16], //string ip
              paw: [0;16], //string ip
              unknown8: 0u32,
              unknown9: 0u32,
              unknown10: 0u32,
              unknown11: 0u32,
              unknown12: 0u32,
              unknown13: 0u32,
              unknown14: 0u32,
              unknown15: 0u32,
              unknown16: 0u32,
              unknown17: 0u32,
              unknown18: 0u32,
              unknown19: 0u32,
              dev_mac: [0;18], //string
              dev_ip: [0;18], //string
              dev_port: 0u32,
          }
      }
  }

pub fn head_parse(buf: Vec<u8>) -> Result<Box<Header>, std::io::Error>  {
    // req = 130 bytes, resp is 128 bytes
    let mut head: Box<Header> = Box::new(Default::default());
    let mut buf =  Cursor::new(buf);
    head.cmd = try!(buf.read_u32::<LittleEndian>());
    head.req_conn_id = try!(buf.read_u32::<LittleEndian>());
    head.cmd_type = try!(buf.read_u16::<LittleEndian>());
    try!(buf.read_exact(&mut head.version));
    try!(buf.read_exact(&mut head.model));
    try!(buf.read_exact(&mut head.dev_name));
    try!(buf.read_exact(&mut head.serial));
    head.resp_status = try!(buf.read_u32::<LittleEndian>());
    head.seq_counter = try!(buf.read_u32::<LittleEndian>());
    head.unknown = try!(buf.read_u32::<LittleEndian>());
    head.resp_conn_id = try!(buf.read_u32::<LittleEndian>());
    if buf.get_ref().len() > 128 { // only requests contain the last two bytes
        head.operation = try!(buf.read_u8());
        head.rw_byte = try!(buf.read_u8());
    }
    Ok(head)
}

pub fn recv_msg (sockAddr: SocketAddr, buf_size: usize, buf: Vec<u8>) {
    let head;
    match head_parse(buf) {
        Ok(n) => {
            head = *n;
            dump_packet_header(head);
        },
        Err(err) => println!("Failed to get a complete packet. Dropping: Debug {:?} ",err),
    }
}

pub fn parse_broadcast(buf: Vec<u8>) -> Result<Box<BroadcastResp>, std::io::Error> {
    let mut bresp: Box<BroadcastResp> = Box::new(Default::default());
    let mut buf =  Cursor::new(buf);
    if buf.get_ref().len() == 408 { // if its not 408 itsour own broadcast comming back.
        bresp.unknown = try!(buf.read_u32::<LittleEndian>());
        try!(buf.read_exact(&mut bresp.version));
        try!(buf.read_exact(&mut bresp.dev_model));
        try!(buf.read_exact(&mut bresp.dev_name));
        try!(buf.read_exact(&mut bresp.dev_serial));
        bresp.unknown2 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown3 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown4 = try!(buf.read_u32::<LittleEndian>());
        try!(buf.read_exact(&mut bresp.ssid));
        try!(buf.read_exact(&mut bresp.wifi_pass));
        bresp.unknown5 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown6 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown7 = try!(buf.read_u32::<LittleEndian>());
        try!(buf.read_exact(&mut bresp.zipcode));
        try!(buf.read_exact(&mut bresp.p2pm));
        try!(buf.read_exact(&mut bresp.p2ps));
        try!(buf.read_exact(&mut bresp.paw));
        bresp.unknown8 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown9 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown10 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown11 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown12 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown13 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown14 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown15 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown16 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown17 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown18 = try!(buf.read_u32::<LittleEndian>());
        bresp.unknown19 = try!(buf.read_u32::<LittleEndian>());
        try!(buf.read_exact(&mut bresp.dev_mac));
        try!(buf.read_exact(&mut bresp.dev_ip));
        bresp.dev_port = try!(buf.read_u32::<LittleEndian>());
    }
    Ok(bresp)
}

pub fn dump_packet_header(head: Header) {
    println!("Msg_Header");
    println!("[Cmd: 0x{:X}, Req Conn ID: 0x{:X}, cmd_type: 0x{:X}, \n Version: {}, Model: {}, Dev_name: {}, Serial: {},\n Resp_Status: 0x{:X}, Seq Counter: {}, Unknown: {} \
     Resp Conn ID: 0x{:X}, \n Operation: {}, rwByte: {}]", head.cmd,  head.req_conn_id, head.cmd_type, str::from_utf8(&head.version).unwrap(), str::from_utf8(&head.model).unwrap(),
     str::from_utf8(&head.dev_name).unwrap(), str::from_utf8(&head.serial).unwrap(), head.resp_status, head.seq_counter, head.unknown, head.resp_conn_id,
      head.operation, head.rw_byte);
}

pub fn dump_packet_broadcast(bresp: BroadcastResp) {
    println!("Unknown: 0x{:X}", bresp.unknown);
    println!("Version: {}", str::from_utf8(&bresp.version).unwrap());
    println!("Model: {}", str::from_utf8(&bresp.dev_model).unwrap());
    println!("Name: {}", str::from_utf8(&bresp.dev_name).unwrap());
    println!("Serial: {}", str::from_utf8(&bresp.dev_serial).unwrap());
    println!("Unknown2: 0x{:X}", bresp.unknown2);
    println!("Unknown3: 0x{:X}", bresp.unknown3);
    println!("Unknown4: 0x{:X}", bresp.unknown4);
    println!("SSID: {}", str::from_utf8(&bresp.ssid).unwrap());
    println!("WiFi Pass: {}", str::from_utf8(&bresp.wifi_pass).unwrap());
    println!("Unknown5: 0x{:X}", bresp.unknown5);
    println!("Unknown6: 0x{:X}", bresp.unknown6);
    println!("Unknown7: 0x{:X}", bresp.unknown7);
    println!("Zipcode: {}", str::from_utf8(&bresp.zipcode).unwrap());
    println!("p2pm: {}", str::from_utf8(&bresp.p2pm).unwrap());
    println!("p2ps: {}", str::from_utf8(&bresp.p2ps).unwrap());
    println!("paw: {}", str::from_utf8(&bresp.paw).unwrap());
    println!("Unknown8: 0x{:X}", bresp.unknown8);
    println!("Unknown9: 0x{:X}", bresp.unknown9);
    println!("Unknown10: 0x{:X}", bresp.unknown10);
    println!("Unknown11: 0x{:X}", bresp.unknown11);
    println!("Unknown12: 0x{:X}", bresp.unknown12);
    println!("Unknown13: 0x{:X}", bresp.unknown13);
    println!("Unknown14: 0x{:X}", bresp.unknown14);
    println!("Unknown15: 0x{:X}", bresp.unknown15);
    println!("Unknown16: 0x{:X}", bresp.unknown16);
    println!("Unknown17: 0x{:X}", bresp.unknown17);
    println!("Unknown18: 0x{:X}", bresp.unknown18);
    println!("Unknown19: 0x{:X}", bresp.unknown19);
    println!("MAC: {}", str::from_utf8(&bresp.dev_mac).unwrap());
    println!("IP: {}", str::from_utf8(&bresp.dev_ip).unwrap());
    println!("Port: {}", bresp.dev_port);
}

pub fn open_socket(socketaddr: SocketAddr) -> UdpSocket  {
    match UdpSocket::bind(socketaddr) {
        Ok(sock) => {
            return sock;
        },
        Err(err) => panic!("Could not bind: {}", err)
    }
}

fn broadcast_listener(socket: UdpSocket) {
        let mut buf = vec![0;1500];
        let mut bresp;
        while true {
            match socket.recv_from(&mut buf) { //blocks
                Ok( (rcount,src_ip) ) =>  {
                    let buf = buf[0..rcount].to_vec(); // trim buf to actual recieved bytes
                    println!("recvd broadcast packet: size: {} SRC: {}", buf.len(), src_ip);
                    match parse_broadcast(buf) {
                        Ok(n) => {
                            bresp = *n;
                            dump_packet_broadcast(bresp);
                        },
                        Err(err) => println!("Failed to get a complete packet. Dropping: Debug {:?} ",err),
                    }
                },
                Err(err) => panic!("recv error: {}", err)
            }
        }
    }

pub fn broadcast_setup() -> UdpSocket {
    // broadcast only
    let bind_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let ip_addr_listen_on = SocketAddr::new(bind_ip, broadcast_port);
    let socket = open_socket(ip_addr_listen_on);
    let socket_listen = socket.try_clone().unwrap(); // have to clone it for thread
    thread::spawn(|| broadcast_listener(socket_listen));
    socket
}

pub fn msg_listener_setup() -> UdpSocket {
    // broadcast only
    let bind_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let ip_addr_listen_on = SocketAddr::new(bind_ip, 9191);
    let socket = open_socket(ip_addr_listen_on);
    let socket_listen = socket.try_clone().unwrap(); // have to clone it for thread
    thread::spawn(|| msg_listener(socket_listen));
    socket
}

pub fn msg_listener(socket: UdpSocket) {
    println!("my ip: {:?}", socket.local_addr());
    let mut buf = vec![0;1500];
    let mut hresp;
    while true {
        match socket.recv_from(&mut buf) {
            Ok( (rcount,src_ip) ) =>  {
                let buf = buf[0..rcount].to_vec(); // trim buf to actual recieved bytes
                println!("recvd msg resp: size: {} SRC: {}", buf.len(), src_ip);
                match head_parse(buf) {
                    Ok(n) => {
                        hresp = *n;
                        dump_packet_header(hresp)
                    },
                    Err(err) => println!("Failed to get a complete packet. Dropping: Debug {:?} ",err),
                }
            },
            Err(err) => panic!("recv error: {}", err)
        }
    }
}

pub fn send_broadcast(socket: &UdpSocket) {
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

fn send_basic_modify(socket: &UdpSocket, write_value:u8, device_ip: &SocketAddr ) {
    let mut rng = rand::thread_rng();
    let mut head: Header = Default::default();
    head.cmd = CMD_BASCI_MODIFY_SWITCH;
    head.req_conn_id = rng.gen::<u32>(); ; // needs to be changed each time or device is flakey with fast changes.  using rand now,
    head.cmd_type = 0x02;
    head.model = [0x45, 0x43, 0x4F, 0x2D, 0x37, 0x38, 0x30, 0x30, 0x34, 0x42, 0x30, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    head.seq_counter = 0x55555555; // needs to be changed each time or device is flakey with fast changes.  using rand now, but could be incremented
    head.operation = 0x01; // this appears not to make a difference on modify_switchh.  it can be 1 or 0 and still work???
    head.rw_byte = write_value;
    // convert struct to byte stream
    let buf = pack_header(head);
    //send packet
    socket.send_to(&buf, device_ip).expect("error sending");
}

fn send_basic_cmd(socket: &UdpSocket, cmd: u32, cmd_type: u16, device_ip: &SocketAddr ) {
    let mut rng = rand::thread_rng();
    let mut head: Header = Default::default();
    head.cmd = cmd;
    head.req_conn_id = rng.gen::<u32>(); ; // needs to be changed each time or device is flakey with fast changes.  using rand now,
    head.cmd_type = cmd_type;
    head.model = [0x45, 0x43, 0x4F, 0x2D, 0x37, 0x38, 0x30, 0x30, 0x34, 0x42, 0x30, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    head.seq_counter = 0x55555555; // needs to be changed each time or device is flakey with fast changes.  using rand now, but could be incremented
    // convert struct to byte stream
    let buf = pack_header(head);
    //send packet
    socket.send_to(&buf, device_ip).expect("error sending");
}

pub fn send_switch_toggle(switch: bool, device_ip: &str, socket: &UdpSocket) {
    let ip: std::net::IpAddr = device_ip.parse().unwrap();
    let dst = SocketAddr::new(ip, messaging_port);
    println!{"Toggling switch at dev ip: {:?}", device_ip};
    send_basic_modify(&socket, switch as u8, &dst );
}

pub fn get_switch_status(device_ip: &str, socket: &UdpSocket) {
    let ip: std::net::IpAddr = device_ip.parse().unwrap();
    let dst = SocketAddr::new(ip, messaging_port);
    println!{"getting switch status at dev ip: {:?}", device_ip};
    send_basic_cmd(&socket, CMD_BASCI_GET_SWITCH_STATUS, 0x00, &dst );
}

fn pack_header(head: Header) -> Box<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    buf.write_u32::<LittleEndian>(head.cmd).unwrap();
    buf.write_u32::<LittleEndian>(head.req_conn_id).unwrap();
    buf.write_u16::<LittleEndian>(head.cmd_type).unwrap();
    buf.write_all(&head.version).unwrap();
    buf.write_all(&head.model).unwrap();
    buf.write_all(&head.dev_name).unwrap();
    buf.write_all(&head.serial).unwrap();
    buf.write_u32::<LittleEndian>(head.resp_status).unwrap();
    buf.write_u32::<LittleEndian>(head.seq_counter).unwrap();
    buf.write_u32::<LittleEndian>(head.unknown).unwrap();
    buf.write_u32::<LittleEndian>(head.resp_conn_id).unwrap();
    if head.cmd == CMD_BASCI_MODIFY_SWITCH {
        buf.write_u8(head.operation).unwrap();
        buf.write_u8(head.rw_byte).unwrap();
    }
    return Box::new (buf)
}
