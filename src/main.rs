use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

struct TcpState {}
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let _connections: HashMap<Quad, TcpState> = Default::default();
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buff = [0u8; 1504];

    loop {
        //passing a mutable reference of the buffer to the recv function
        let nbytes = nic.recv(&mut buff[..])?;
        let _eth_flags = u16::from_be_bytes([buff[0], buff[1]]);
        // set by thelink level protocol for the ethernet frame
        let eth_proto = u16::from_be_bytes([buff[2], buff[3]]);
        if eth_proto != 0x0800 {
            // ignore packets that are not ipv4
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buff[4..nbytes]) {
            Ok(p) => {
                let p_len = p.payload_len();
                let src = p.source_addr();
                let dst = p.destination_addr();
                // ip level protocol
                let proto = p.protocol();

                if proto != 0x06 {
                    // ignore packet that are not sent with tcp client
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buff[4 + p.slice().len()..]) {
                    Ok(p) => {
                        // connection identity -> (srcip, srcport, dstip, dstport)
                        eprintln!(
                            "{} -> {} {}bytes of tcp ot port {}",
                            src,
                            dst,
                            p.slice().len(),
                            p.destination_port(),
                        )
                    }
                    Err(e) => {
                        eprintln!("WARNING: ignoring weird tcp packet: {:?}", e);
                    }
                }

                eprintln!("{} -> {} {}bytes of protocol {}", src, dst, p_len, proto)
            }
            Err(e) => {
                eprintln!("WARNING: ignoring the packet: {:?}", e);
            }
        }
    }
}
