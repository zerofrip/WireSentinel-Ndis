use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IpProtocol {
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
    Other = 255,
}

impl IpProtocol {
    pub fn from_number(value: u8) -> Self {
        match value {
            1 => Self::Icmp,
            6 => Self::Tcp,
            17 => Self::Udp,
            _ => Self::Other,
        }
    }

    pub fn number(self) -> u8 {
        match self {
            Self::Icmp => 1,
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Other => 255,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPacket {
    pub protocol: IpProtocol,
    pub src_ip: IpAddr,
    pub dst_ip: IpAddr,
    pub src_port: u16,
    pub dst_port: u16,
    pub payload_len: usize,
}

impl ParsedPacket {
    pub fn parse_ipv4(raw: &[u8]) -> Option<Self> {
        if raw.len() < 20 {
            return None;
        }
        let version = raw[0] >> 4;
        if version != 4 {
            return None;
        }
        let ihl = (raw[0] & 0x0f) as usize * 4;
        if raw.len() < ihl {
            return None;
        }
        let protocol = IpProtocol::from_number(raw[9]);
        let src = Ipv4Addr::new(raw[12], raw[13], raw[14], raw[15]);
        let dst = Ipv4Addr::new(raw[16], raw[17], raw[18], raw[19]);
        let (src_port, dst_port) = parse_l4_ports(raw, ihl, protocol)?;
        Some(Self {
            protocol,
            src_ip: IpAddr::V4(src),
            dst_ip: IpAddr::V4(dst),
            src_port,
            dst_port,
            payload_len: raw.len().saturating_sub(ihl),
        })
    }

    pub fn parse_ipv6(raw: &[u8]) -> Option<Self> {
        if raw.len() < 40 {
            return None;
        }
        if (raw[0] >> 4) != 6 {
            return None;
        }
        let protocol = IpProtocol::from_number(raw[6]);
        let src = Ipv6Addr::from([
            raw[8], raw[9], raw[10], raw[11], raw[12], raw[13], raw[14], raw[15], raw[16], raw[17],
            raw[18], raw[19], raw[20], raw[21], raw[22], raw[23],
        ]);
        let dst = Ipv6Addr::from([
            raw[24], raw[25], raw[26], raw[27], raw[28], raw[29], raw[30], raw[31], raw[32],
            raw[33], raw[34], raw[35], raw[36], raw[37], raw[38], raw[39],
        ]);
        let (src_port, dst_port) = parse_l4_ports(raw, 40, protocol)?;
        Some(Self {
            protocol,
            src_ip: IpAddr::V6(src),
            dst_ip: IpAddr::V6(dst),
            src_port,
            dst_port,
            payload_len: raw.len().saturating_sub(40),
        })
    }
}

fn parse_l4_ports(raw: &[u8], header_len: usize, protocol: IpProtocol) -> Option<(u16, u16)> {
    match protocol {
        IpProtocol::Tcp | IpProtocol::Udp => {
            if raw.len() < header_len + 4 {
                return None;
            }
            let src = u16::from_be_bytes([raw[header_len], raw[header_len + 1]]);
            let dst = u16::from_be_bytes([raw[header_len + 2], raw[header_len + 3]]);
            Some((src, dst))
        }
        _ => Some((0, 0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ipv4_udp() {
        let mut pkt = vec![0u8; 28];
        pkt[0] = 0x45;
        pkt[9] = 17;
        pkt[12..16].copy_from_slice(&[10, 0, 0, 1]);
        pkt[16..20].copy_from_slice(&[8, 8, 8, 8]);
        pkt[20..22].copy_from_slice(&1234u16.to_be_bytes().as_ref());
        pkt[22..24].copy_from_slice(&53u16.to_be_bytes().as_ref());
        let parsed = ParsedPacket::parse_ipv4(&pkt).expect("udp packet");
        assert_eq!(parsed.protocol, IpProtocol::Udp);
        assert_eq!(parsed.src_port, 1234);
        assert_eq!(parsed.dst_port, 53);
    }
}
