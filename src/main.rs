use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::State> = Default::default();
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
            // ip header
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                // ip level protocol
                if iph.protocol() != 0x06 {
                    // ignore packet that are not sent with tcp client
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buff[4 + iph.slice().len()..nbytes]) {
                    // tcp header
                    Ok(tcph) => {
                        let datai = 4 + iph.slice().len() + tcph.slice().len();
                        connections
                            .entry(Quad {
                                src: (src, tcph.source_port()),
                                dst: (dst, tcph.destination_port()),
                            })
                            .or_default()
                            .on_packet(iph, tcph, &buff[datai..nbytes]);
                        // connection identity -> (srcip, srcport, dstip, dstport)
                    }
                    Err(e) => {
                        eprintln!("WARNING: ignoring weird tcp packet: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("WARNING: ignoring the packet: {:?}", e);
            }
        }
    }
}
