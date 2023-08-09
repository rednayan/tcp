use std::io;

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

pub struct Connection {
    state: State,
}

///  State of the Send Sequence Space (RFC 793 S3.2 F4)
///  ````
///               1         2          3          4
///          ----------|----------|----------|----------
///                 SND.UNA    SND.NXT    SND.UNA
///                                      +SND.WND

///    1 - old sequence numbers which have been acknowledged
///    2 - sequence numbers of unacknowledged data
///    3 - sequence numbers allowed for new data transmission
///    4 - future sequence numbers which are not yet allowed
///````

struct SendSequence {
    /// send unacknowledged
    una: usize,
    /// send next
    nxt: usize,
    /// send window
    wnd: usize,
    /// send urgnet pointers
    up: bool,
    /// segment sequence used for last window update
    wl1: usize,
    /// segment acknowledgement number used for last window update
    wl2: usize,
    /// initial send sequence number
    iss: usize,
}

impl Default for Connection {
    fn default() -> Self {
        // Connection::Closed
        // listen on all ports by default
        Connection {
            state: State::Listen,
        }
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buff = [0u8; 1504];

        match *self {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    // got unexpected syn packet
                    return Ok(0);
                }
                // need to start establishing connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    unimplemented!(),
                    unimplemented!(),
                );
                syn_ack.syn = true;
                syn_ack.ack = true;

                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::IpTrafficClass::Tcp,
                    iph.destination_addr().octets(),
                    iph.source_addr().octets(),
                );

                // write out the headers
                let unwritten = {
                    let mut unwritten = &mut buff[..];
                    ip.write(&mut unwritten).ok();
                    syn_ack.write(&mut unwritten)?;
                    unwritten.len()
                };
                nic.send(&buff[..unwritten])?;
                Ok(0)
            }
            State::SynRcvd => Ok(0),
            State::Estab => Ok(0),
        }
    }
}
