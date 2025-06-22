// use std::net::{Ipv4Addr, Ipv6Addr};
// use serde::{Deserialize, Serialize};
// 
// #[derive(Default, Debug, Clone, Serialize, Deserialize)]
// pub struct RTCSessionDescription {
//     #[serde(rename = "type")]
//     pub sdp_type: RTCSdpType,
// 
//     pub sdp: String,
// 
//     /// This will never be initialized by callers, internal use only
//     #[serde(skip)]
//     pub(crate) parsed: Option<SessionDescription>,
// }
// 
// #[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
// pub enum RTCSdpType {
//     #[serde(rename = "offer")]
//     #[default]
//     Offer,
//     #[serde(rename = "answer")]
//     Answer,
// }
// 
// pub type Version = isize;
// pub type SessionName = String;
// pub type Information = String;
// pub type EmailAddress = String;
// pub type PhoneNumber = String;
// 
// #[derive(Clone)]
// pub struct Url {
//     /// Syntax in pseudo-BNF:
//     ///
//     ///   url = scheme ":" [ hierarchical | non-hierarchical ] [ "?" query ]? [ "#" fragment ]?
//     ///   non-hierarchical = non-hierarchical-path
//     ///   non-hierarchical-path = /* Does not start with "/" */
//     ///   hierarchical = authority? hierarchical-path
//     ///   authority = "//" userinfo? host [ ":" port ]?
//     ///   userinfo = username [ ":" password ]? "@"
//     ///   hierarchical-path = [ "/" path-segment ]+
//     serialization: String,
// 
//     // Components
//     scheme_end: u32,   // Before ':'
//     username_end: u32, // Before ':' (if a password is given) or '@' (if not)
//     host_start: u32,
//     host_end: u32,
//     host: HostInternal,
//     port: Option<u16>,
//     path_start: u32,             // Before initial '/', if any
//     query_start: Option<u32>,    // Before '?', unlike Position::QueryStart
//     fragment_start: Option<u32>, // Before '#', unlike Position::FragmentStart
// }
// 
// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub(crate) enum HostInternal {
//     None,
//     Domain,
//     Ipv4(Ipv4Addr),
//     Ipv6(Ipv6Addr),
// }
// 
// #[derive(Debug, Default, Clone)]
// pub struct Origin {
//     pub username: String,
//     pub session_id: u64,
//     pub session_version: u64,
//     pub network_type: String,
//     pub address_type: String,
//     pub unicast_address: String,
// }
// 
// #[derive(Debug, Default, Clone)]
// pub struct SessionDescription {
//     /// `v=0`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.1>
//     pub version: Version,
// 
//     /// `o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.2>
//     pub origin: Origin,
// 
//     /// `s=<session name>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.3>
//     pub session_name: SessionName,
// 
//     /// `i=<session description>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.4>
//     pub session_information: Option<Information>,
// 
//     /// `u=<uri>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.5>
//     pub uri: Option<Url>,
// 
//     /// `e=<email-address>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.6>
//     pub email_address: Option<EmailAddress>,
// 
//     /// `p=<phone-number>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.6>
//     pub phone_number: Option<PhoneNumber>,
// 
//     /// `c=<nettype> <addrtype> <connection-address>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.7>
//     pub connection_information: Option<ConnectionInformation>,
// 
//     /// `b=<bwtype>:<bandwidth>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.8>
//     pub bandwidth: Vec<Bandwidth>,
// 
//     /// <https://tools.ietf.org/html/rfc4566#section-5.9>
//     /// <https://tools.ietf.org/html/rfc4566#section-5.10>
//     pub time_descriptions: Vec<TimeDescription>,
// 
//     /// `z=<adjustment time> <offset> <adjustment time> <offset> ...`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.11>
//     pub time_zones: Vec<TimeZone>,
// 
//     /// `k=<method>`
//     ///
//     /// `k=<method>:<encryption key>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.12>
//     pub encryption_key: Option<EncryptionKey>,
// 
//     /// `a=<attribute>`
//     ///
//     /// `a=<attribute>:<value>`
//     ///
//     /// <https://tools.ietf.org/html/rfc4566#section-5.13>
//     pub attributes: Vec<Attribute>,
// 
//     /// <https://tools.ietf.org/html/rfc4566#section-5.14>
//     pub media_descriptions: Vec<MediaDescription>,
// }