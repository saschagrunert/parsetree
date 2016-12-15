extern crate peel;
use peel::prelude::*;

extern crate nom;
use nom::{ErrorKind, Needed, Err};

static IPV4_HEADER: &'static [u8] = &[0x45, 0x00, 0x01, 0xa5, 0xd6, 0x63, 0x40, 0x00, 0x3f, 0x06, 0x9b, 0xfc, 0xc0,
                                      0xa8, 0x01, 0x0a, 0xad, 0xfc, 0x58, 0x44];

#[test]
fn ipv4_parser_variant() {
    let parser = Ipv4Parser;
    println!("{:?}", parser.variant());
}

#[test]
fn parse_ipv4_success() {
    let parser = Ipv4Parser;
    let res = parser.parse(IPV4_HEADER, None, None, None).unwrap();
    println!("{}", res.1);
    match res {
        (_, Layer::Ipv4(ipv4)) => {
            assert_eq!(Ipv4Packet {
                           version: 4,
                           ihl: 20,
                           tos: 0,
                           length: 421,
                           id: 54883,
                           flags_and_fragment_offset: 16384,
                           ttl: 63,
                           protocol: IpProtocol::Tcp,
                           checksum: 39932,
                           src: Ipv4Addr::new(192, 168, 1, 10),
                           dst: Ipv4Addr::new(173, 252, 88, 68),
                       },
                       ipv4)
        }
        _ => {}
    }
}

#[test]
fn parse_ipv4_success_ipprotocols() {
    let parser = Ipv4Parser;
    // TCP
    let mut input = Vec::from(IPV4_HEADER);
    parser.parse(&input, None, None, None).unwrap();

    // UDP
    input[9] = 17;
    parser.parse(&input, None, None, None).unwrap();
}

#[test]
fn parse_ipv4_failure_wrong_version() {
    let parser = Ipv4Parser;
    let mut input = Vec::from(IPV4_HEADER);
    input[0] = 0x55;
    let res = parser.parse(&input, None, None, None);
    assert_eq!(res,
               IResult::Error(Err::Position(ErrorKind::TagBits, &input[..])));
}

#[test]
fn parse_ipv4_failure_wrong_ipprotocol() {
    let parser = Ipv4Parser;
    let mut input = Vec::from(IPV4_HEADER);
    input[9] = 0xff;
    let res = parser.parse(&input, None, None, None);
    assert_eq!(res,
               IResult::Error(Err::Position(ErrorKind::MapOpt, &input[9..])));
}

#[test]
fn parse_ipv4_failure_too_small() {
    let parser = Ipv4Parser;
    let mut input = Vec::from(IPV4_HEADER);
    input.pop();
    let res = parser.parse(&input, None, None, None);
    assert_eq!(res, IResult::Incomplete(Needed::Size(20)));
}
