extern crate byteorder;
extern crate rand;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use rand::Rng;
use std::str;
mod wion_comm;
use std::thread;
use std::time;

fn main() {
    // discover devices
    if false {
        let send_broadcast_sock = wion_comm::broadcast_setup();
        wion_comm::send_broadcast(&send_broadcast_sock);
        thread::sleep(time::Duration::from_millis(5000));
        wion_comm::send_broadcast(&send_broadcast_sock);
        thread::sleep(time::Duration::from_millis(5000));
    }
    //setup the socket for sending/recv msgs
    let msg_sock = wion_comm::msg_listener_setup();

    // get switch status
    if false {
        wion_comm::get_switch_status("192.168.0.237", &msg_sock);
        thread::sleep(time::Duration::from_millis(5000));
    }

    // toggle a switch by IP addr
    if false {
        wion_comm::send_switch_toggle(true, "192.168.0.237", &msg_sock);
        thread::sleep(time::Duration::from_millis(5000));
        wion_comm::send_switch_toggle(false, "192.168.0.237", &msg_sock);
        thread::sleep(time::Duration::from_millis(5000));
    }

    //get rom status
    if false {
        wion_comm::get_rom_status("192.168.0.237", &msg_sock);
        thread::sleep(time::Duration::from_millis(5000));
    }

    //get switch settings
    //get rom status
    if true {
        wion_comm::get_switch_settings("192.168.0.237", &msg_sock);
        thread::sleep(time::Duration::from_millis(5000));
    }


}
