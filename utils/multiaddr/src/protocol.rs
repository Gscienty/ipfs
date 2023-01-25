use std::{
    borrow::Cow,
    io::{Cursor, Write},
    net::{Ipv4Addr, Ipv6Addr},
    str,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use multicodec::to_type;
use varint::Varint;

use crate::{to_code, UnknownProtocolError};

pub enum Protocol<'s> {
    DCCP(u16),
    DNS(Cow<'s, str>),
    DNS4(Cow<'s, str>),
    DNS6(Cow<'s, str>),
    DNSAddr(Cow<'s, str>),
    HTTP,
    HTTPS,
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
    P2PWebRTCDirect,
    P2PWebRTCStar,
    WebRTC,
    CertHash(Vec<u8>),
    P2PWebSocketStar,
    Memory(u64),
    P2P(Vec<u8>),
    P2PCircuit,
    QUIC,
    QUICv1,
    SCTP(u16),
    TCP(u16),
    TLS,
    Noise,
    UDP(u16),
    UDT,
    Unix(Cow<'s, str>),
    UTP,
    WebTransport,
    WS(Cow<'s, str>),
    WSS(Cow<'s, str>),
}

macro_rules! protocol_write_type_bytes {
    ($w: expr, $type: expr) => {{
        let mut buf = [0u8; 4];
        let len = to_code($type).encode_varint(&mut buf);
        $w.write_all(&buf[..len]).or(Err(UnknownProtocolError))?;
    }};
}

macro_rules! protocol_read_str {
    ($type: expr, $len: expr, $input: expr) => {{
        let (strlen, stroff) = u64::decode_varint($input).ok_or(UnknownProtocolError)?;
        let cnt = str::from_utf8(&$input[stroff..stroff + strlen as usize])
            .or(Err(UnknownProtocolError))?;

        Ok(($type(Cow::Borrowed(cnt)), $len + stroff + strlen as usize))
    }};
}

macro_rules! protocol_write_str {
    ($w: expr, $type: expr, $val: expr) => {{
        protocol_write_type_bytes!($w, $type);

        let mut buf = [0u8; 8];
        let len = ($val.len() as u64).encode_varint(&mut buf);
        $w.write_all(&buf[..len]).or(Err(UnknownProtocolError))?;
        $w.write_all($val.as_bytes())
            .or(Err(UnknownProtocolError))?;
    }};
}

macro_rules! protocol_read_vec {
    ($type: expr, $len: expr, $input: expr) => {{
        let (veclen, vecoff) = u64::decode_varint($input).ok_or(UnknownProtocolError)?;
        let cnt = Vec::from(&$input[vecoff..vecoff + veclen as usize]);

        Ok(($type(cnt), $len + vecoff + veclen as usize))
    }};
}

macro_rules! protocol_write_vec {
    ($w: expr, $type: expr, $val: expr) => {{
        protocol_write_type_bytes!($w, $type);

        let mut buf = [0u8; 8];
        let len = ($val.len() as u64).encode_varint(&mut buf);
        $w.write_all(&buf[..len]).or(Err(UnknownProtocolError))?;
        $w.write_all($val).or(Err(UnknownProtocolError))?;
    }};
}

macro_rules! protocol_read_u16 {
    ($type: expr, $len: expr, $input: expr) => {{
        let mut reader = Cursor::new($input);
        let val = reader
            .read_u16::<BigEndian>()
            .or(Err(UnknownProtocolError))?;

        Ok(($type(val), $len + 2))
    }};
}

macro_rules! protocol_write_u16 {
    ($w: expr, $type: expr, $val: expr) => {{
        protocol_write_type_bytes!($w, $type);
        $w.write_u16::<BigEndian>($val)
            .or(Err(UnknownProtocolError))?;
    }};
}

macro_rules! protocol_read_u64 {
    ($type: expr, $len: expr, $input: expr) => {{
        let mut reader = Cursor::new($input);
        let val = reader
            .read_u64::<BigEndian>()
            .or(Err(UnknownProtocolError))?;

        Ok(($type(val), $len + 8))
    }};
}

macro_rules! protocol_write_u64 {
    ($w: expr, $type: expr, $val: expr) => {{
        protocol_write_type_bytes!($w, $type);
        $w.write_u64::<BigEndian>($val)
            .or(Err(UnknownProtocolError))?;
    }};
}

impl<'s> Protocol<'s> {
    pub(crate) fn from_bytes(input: &'s [u8]) -> Result<(Self, usize), UnknownProtocolError> {
        let (id, len) = u64::decode_varint(input).ok_or(UnknownProtocolError)?;
        let input = &input[len..];

        match to_type(id).as_str() {
            "ip4" => Ok((
                Protocol::IPv4(Ipv4Addr::new(input[0], input[1], input[2], input[3])),
                len + 4,
            )),
            "ip6" => {
                let mut reader = Cursor::new(input);

                let mut seg = [0u16; 8];
                for x in seg.iter_mut() {
                    *x = reader
                        .read_u16::<BigEndian>()
                        .or(Err(UnknownProtocolError))?;
                }

                Ok((
                    Protocol::IPv6(Ipv6Addr::new(
                        seg[0], seg[1], seg[2], seg[3], seg[4], seg[5], seg[6], seg[7],
                    )),
                    len + 16,
                ))
            }
            "dccp" => protocol_read_u16!(Protocol::DCCP, len, input),
            "sctp" => protocol_read_u16!(Protocol::SCTP, len, input),
            "tcp" => protocol_read_u16!(Protocol::TCP, len, input),
            "udp" => protocol_read_u16!(Protocol::UDP, len, input),
            "memory" => protocol_read_u64!(Protocol::Memory, len, input),
            "dns" => protocol_read_str!(Protocol::DNS, len, input),
            "dns4" => protocol_read_str!(Protocol::DNS4, len, input),
            "dns6" => protocol_read_str!(Protocol::DNS6, len, input),
            "dnsaddr" => protocol_read_str!(Protocol::DNSAddr, len, input),
            "unix" => protocol_read_str!(Protocol::Unix, len, input),
            "ws" => protocol_read_str!(Protocol::WS, len, input),
            "wss" => protocol_read_str!(Protocol::WSS, len, input),
            "certhash" => protocol_read_vec!(Protocol::CertHash, len, input),
            "p2p" => protocol_read_vec!(Protocol::P2P, len, input),
            "p2p-webrtc-star" => Ok((Protocol::P2PWebRTCStar, len)),
            "p2p-webrtc-direct" => Ok((Protocol::P2PWebRTCDirect, len)),
            "webrtc" => Ok((Protocol::WebRTC, len)),
            "p2p-circuit" => Ok((Protocol::P2PCircuit, len)),
            "udt" => Ok((Protocol::UDT, len)),
            "utp" => Ok((Protocol::UTP, len)),
            "https" => Ok((Protocol::HTTPS, len)),
            "tls" => Ok((Protocol::TLS, len)),
            "noise" => Ok((Protocol::Noise, len)),
            "quic" => Ok((Protocol::QUIC, len)),
            "quic-v1" => Ok((Protocol::QUICv1, len)),
            "webtransport" => Ok((Protocol::WebTransport, len)),
            "p2p-websocket-star" => Ok((Protocol::P2PWebSocketStar, len)),
            "http" => Ok((Protocol::HTTP, len)),

            _ => Err(UnknownProtocolError),
        }
    }

    pub(crate) fn write_bytes<W: Write>(&self, w: &mut W) -> Result<(), UnknownProtocolError> {
        match self {
            Self::IPv4(addr) => {
                protocol_write_type_bytes!(w, "ip4");
                w.write_all(&addr.octets()).or(Err(UnknownProtocolError))?;
            }
            Self::IPv6(addr) => {
                protocol_write_type_bytes!(w, "ip6");
                w.write_all(&addr.octets()).or(Err(UnknownProtocolError))?;
            }
            Self::DCCP(val) => protocol_write_u16!(w, "dccp", *val),
            Self::SCTP(val) => protocol_write_u16!(w, "sctp", *val),
            Self::TCP(val) => protocol_write_u16!(w, "tcp", *val),
            Self::UDP(val) => protocol_write_u16!(w, "udp", *val),
            Self::Memory(val) => protocol_write_u64!(w, "memory", *val),
            Self::DNS(val) => protocol_write_str!(w, "dns", val),
            Self::DNS4(val) => protocol_write_str!(w, "dns4", val),
            Self::DNS6(val) => protocol_write_str!(w, "dns6", val),
            Self::DNSAddr(val) => protocol_write_str!(w, "dnsaddr", val),
            Self::Unix(val) => protocol_write_str!(w, "unix", val),
            Self::WS(val) => protocol_write_str!(w, "ws", val),
            Self::WSS(val) => protocol_write_str!(w, "wss", val),
            Self::CertHash(val) => protocol_write_vec!(w, "certhash", val),
            Self::P2P(val) => protocol_write_vec!(w, "p2p", val),
            Self::P2PWebRTCStar => protocol_write_type_bytes!(w, "p2p-webrtc-star"),
            Self::P2PWebRTCDirect => protocol_write_type_bytes!(w, "p2p-webrtc-direct"),
            Self::WebRTC => protocol_write_type_bytes!(w, "webrtc"),
            Self::P2PCircuit => protocol_write_type_bytes!(w, "p2p-circuit"),
            Self::UDT => protocol_write_type_bytes!(w, "udt"),
            Self::UTP => protocol_write_type_bytes!(w, "utp"),
            Self::HTTPS => protocol_write_type_bytes!(w, "https"),
            Self::TLS => protocol_write_type_bytes!(w, "tls"),
            Self::Noise => protocol_write_type_bytes!(w, "noise"),
            Self::QUIC => protocol_write_type_bytes!(w, "quic"),
            Self::QUICv1 => protocol_write_type_bytes!(w, "quic-v1"),
            Self::WebTransport => protocol_write_type_bytes!(w, "webtransport"),
            Self::P2PWebSocketStar => protocol_write_type_bytes!(w, "p2p-websocket-star"),
            Self::HTTP => protocol_write_type_bytes!(w, "http"),
        }

        Ok(())
    }

    pub(crate) fn acquire<'b>(self) -> Protocol<'b> {
        match self {
            Self::IPv4(v) => Protocol::IPv4(v),
            Self::IPv6(v) => Protocol::IPv6(v),
            Self::DCCP(v) => Protocol::DCCP(v),
            Self::SCTP(v) => Protocol::SCTP(v),
            Self::TCP(v) => Protocol::TCP(v),
            Self::UDP(v) => Protocol::UDP(v),
            Self::Memory(v) => Protocol::Memory(v),
            Self::DNS(v) => Protocol::DNS(Cow::Owned(v.into_owned())),
            Self::DNS4(v) => Protocol::DNS4(Cow::Owned(v.into_owned())),
            Self::DNS6(v) => Protocol::DNS6(Cow::Owned(v.into_owned())),
            Self::DNSAddr(v) => Protocol::DNSAddr(Cow::Owned(v.into_owned())),
            Self::Unix(v) => Protocol::Unix(Cow::Owned(v.into_owned())),
            Self::WS(v) => Protocol::WS(Cow::Owned(v.into_owned())),
            Self::WSS(v) => Protocol::WSS(Cow::Owned(v.into_owned())),
            Self::CertHash(v) => Protocol::CertHash(v),
            Self::P2P(v) => Protocol::P2P(v),
            Self::P2PWebRTCStar => Protocol::P2PWebRTCStar,
            Self::P2PWebRTCDirect => Protocol::P2PWebRTCDirect,
            Self::WebRTC => Protocol::WebRTC,
            Self::P2PCircuit => Protocol::P2PCircuit,
            Self::UDT => Protocol::UDT,
            Self::UTP => Protocol::UTP,
            Self::HTTPS => Protocol::HTTPS,
            Self::TLS => Protocol::TLS,
            Self::Noise => Protocol::Noise,
            Self::QUIC => Protocol::QUIC,
            Self::QUICv1 => Protocol::QUICv1,
            Self::WebTransport => Protocol::WebTransport,
            Self::P2PWebSocketStar => Protocol::P2PWebSocketStar,
            Self::HTTP => Protocol::HTTP,
        }
    }
}
