pub fn to_type(val: u64) -> String {
    match val {
        4 => "ip4",
        6 => "tcp",
        273 => "udp",
        33 => "dccp",
        41 => "ip6",
        42 => "ip6zone",
        43 => "ipcidr",
        53 => "dns",
        54 => "dns4",
        55 => "dns6",
        56 => "dnsaddr",
        132 => "sctp",
        301 => "udt",
        302 => "utp",
        400 => "unix",
        421 => "p2p",
        421 => "ipfs",
        444 => "onion",
        445 => "onion3",
        446 => "garlic64",
        447 => "garlic32",
        448 => "tls",
        449 => "sni",
        454 => "noise",
        460 => "quic",
        461 => "quic-v1",
        465 => "webtransport",
        466 => "certhash",
        480 => "http",
        443 => "https",
        477 => "ws",
        478 => "wss",
        479 => "p2p-websocket-star",
        277 => "p2p-stardust",
        275 => "p2p-webrtc-star",
        276 => "p2p-webrtc-direct",
        280 => "webrtc",
        290 => "p2p-circuit",
        777 => "memory",
        _ => unreachable!(),
    }
    .to_string()
}

pub fn to_code(val: &str) -> u64 {
    match val {
        "ip4" => 4,
        "tcp" => 6,
        "udp" => 273,
        "dccp" => 33,
        "ip6" => 41,
        "ip6zone" => 42,
        "ipcidr" => 43,
        "dns" => 53,
        "dns4" => 54,
        "dns6" => 55,
        "dnsaddr" => 56,
        "sctp" => 132,
        "udt" => 301,
        "utp" => 302,
        "unix" => 400,
        "p2p" => 421,
        "ipfs" => 421,
        "onion" => 444,
        "onion3" => 445,
        "garlic64" => 446,
        "garlic32" => 447,
        "tls" => 448,
        "sni" => 449,
        "noise" => 454,
        "quic" => 460,
        "quic-v1" => 461,
        "webtransport" => 465,
        "certhash" => 466,
        "http" => 480,
        "https" => 443,
        "ws" => 477,
        "wss" => 478,
        "p2p-websocket-star" => 479,
        "p2p-stardust" => 277,
        "p2p-webrtc-star" => 275,
        "p2p-webrtc-direct" => 276,
        "webrtc" => 280,
        "p2p-circuit" => 290,
        "memory" => 777,
        _ => unreachable!(),
    }
}