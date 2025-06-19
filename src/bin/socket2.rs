use std::io;
use std::mem;
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;

use socket2::{Domain, SockAddr, Socket, Type};

fn main() {
    // Create a TCP listener bound to two addresses.
    let socket = Socket::new(Domain::PACKET, Type::RAW, None).expect("no socket");
    let mut addr_storage: libc::sockaddr_storage = unsafe { mem::zeroed() };
    let mut len = mem::size_of_val(&addr_storage) as libc::socklen_t;
    let socketfd_rawid = socket.as_raw_fd();
    //let mut socketfd_id = socket.as_fd();
    println!("Socket {}", socketfd_rawid);
    let res = unsafe {
        libc::getsockname(
            socket.as_raw_fd(),
            (&mut addr_storage as *mut libc::sockaddr_storage).cast(),
            &mut len,
        )
    };
    if res == -1 {
        println!("Error")
    }
    let address = unsafe { SockAddr::new(addr_storage, len) };
}
