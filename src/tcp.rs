use std::io;

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for State {
    fn default() -> Self {
        // State::Closed
        // listen on all ports by default
        State::Listen
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
                    unimplemented(),
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
                    syn_ack.write(&mut unwritten);
                    unwritten.len();
                };
                nic.send(&buff[..unwritten])?;
                Ok(0)
            }
            State::SynRcvd => Ok(0),
            State::Estab => Ok(0),
        }
    }
}
