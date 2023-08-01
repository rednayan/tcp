use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buff = [0u8; 1504];
    loop {
        //passing a mutable reference of the buffer to the recv function
        let nbytes = nic.recv(&mut buff[..])?;
        let flags = u16::from_be_bytes([buff[0], buff[1]]);
        let proto = u16::from_be_bytes([buff[2], buff[3]]);
        eprintln!(
            "read {} bytes \n flags: {:x?}, \n proto: {:x?} \n buffer: {:x?} \n",
            nbytes - 4,
            &flags,
            &proto,
            &buff[4..nbytes]
        );
    }
}
