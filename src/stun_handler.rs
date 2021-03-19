use std::net::{SocketAddr, IpAddr};
use serde::{Deserialize, Serialize};

// Header constants
const MAGIC_COOKIE: u32 = 0x2112a442;
const SUCCESS_RESPONSE: u16 = 0x0101;
const BINDING_REQUEST: u16 = 0x0001;

// Attribute types constants
const ERROR_CODE: u16 = 0x0009;
const UNKNOWN_ATTRIBUTES: u16 = 0x000a;
const XOR_MAPPED_ADDRESS: u16 = 0x0020;

// Address mapping constants
const IPV4: u8 = 0x01;
const IPV6: u8 = 0x02;

/// STUN-Headers
///
/// A stun message always includes headers as the first element.
///
/// # Examples
/// ```
/// let MAGIC_COOKIE = 0x2112a442;
///
/// // Binding request
/// let headers = Headers {
///     message_type: 0x0001,
///     message_length: 0x0000,
///     MAGIC_COOKIE: MAGIC_COOKIE,
///     transaction_id: 0x34a76bc5f4a9d764d9680140,
/// };
/// ```
/// This is then followed up by an attribute header if this were a response
#[derive(Debug,Copy,Clone, Serialize, Deserialize)]
pub struct Headers {
    /// Type of STUN-message
    pub message_type: u16,
    /// Length of attribute payload
    pub message_length: u16,
    /// Magic cookie is always set to 0x2112a442
    pub magic_cookie: u32,
    /// 96 bit identifier, this is always set by the client
    pub transaction_id: u128, //96 bit only accepted
}

/// Attribute-Headers
///
/// Every attribute comes with this header
///
/// # Examples
/// ```
/// // XOR Mapped Address
/// let attribute_headers = Attribute {
///     attribute_type: 0x0020,
///     attribute_length: 0x0008,
/// }
/// ```
/// ```
/// // Error
/// let attribute_headers = Attribute {
///     attribute_type: 0x0009,
///     attribute_length: 0x0008,
/// }
/// ```
#[derive(Debug,Copy,Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Type of attribute, predefined as:
    ///
    /// 0x0000: Reserved [`RFC8489`]
    ///
    /// 0x0001: MAPPED-ADDRESS [`RFC8489`]
    ///
    /// 0x0002: Reserved; was RESPONSE-ADDRESS prior to [`RFC5389`]
    ///
    /// 0x0003: Reserved; was CHANGE-REQUEST prior to [`RFC5389`]
    ///
    /// 0x0004: Reserved; was SOURCE-ADDRESS prior to [`RFC5389`]
    ///
    /// 0x0005: Reserved; was CHANGED-ADDRESS prior to [`RFC5389`]
    ///
    /// 0x0006: USERNAME [`RFC8489`]
    ///
    /// 0x0007: Reserved; was PASSWORD prior to [`RFC5389`]
    ///
    /// 0x0008: MESSAGE-INTEGRITY [`RFC8489`]
    ///
    /// 0x0009: ERROR-CODE [`RFC8489`]
    ///
    /// 0x000A: UNKNOWN-ATTRIBUTES [`RFC8489`]
    ///
    /// 0x000B: Reserved; was REFLECTED-FROM prior to [`RFC5389`]
    ///
    /// 0x0014: REALM [`RFC8489`]
    ///
    /// 0x0015: NONCE [`RFC8489`]
    ///
    /// 0x0020: XOR-MAPPED-ADDRESS [`RFC8489`]
    ///
    /// [`RFC8489`]: https://tools.ietf.org/html/rfc8489#section-18.3.1
    /// [`RFC5389`]: https://tools.ietf.org/html/rfc5389#section-18.2
    pub attribute_type: u16,
    /// Length of attribute in bytes
    pub attribute_length: u16, //in bytes
}

/// XOR Mapped Address attribute
///
/// Attribute in a STUN response for a binding request
///
/// # Examples
/// ```
///  let attribute = XorMappedAddress {
///     reserved: 0x00,
///     family: 0x01,
///     x_port: 0x4acd,
///     x_address: 0x76c084f7,
/// }
/// ```
///
/// For more information about format visit [`RFC5389`]
///
/// [`RFC5389`]:https://tools.ietf.org/html/rfc8489#section-14.1
#[derive(Debug,Copy,Clone, Serialize, Deserialize)]
pub struct XorMappedAddress {
    /// Always set to 0
    pub reserved: u8,
    /// IPv4 or IPv6
    pub family: u8,
    /// XOR'ed port number with the most significant 16 bits of the magic cookie
    pub x_port: u16,
    /// IPv4: XOR'ed ip address with the [`magic cookie`]
    ///
    /// IPv6: Ip address extended with [`magic cookie`] XOR'ed with [`Transaction ID`]
    ///
    /// [`magic cookie`]:struct.Headers.html#structfield.MAGIC_COOKIE
    /// [`Transaction ID`]:struct.Headers.html#structfield.transaction_id
    pub x_address: u128,
}

/// Error Code Attribute
///
/// Attribute struct for an error response
///
/// # Examples
/// ```
/// let error = Error {
///     error_code: 500,
///     reason: String::from("Internal server error"),
///     unknown_attributes: std::ptr::null(),
/// }
/// ```
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Error {
    ///Number between 300-600
    pub error_code: u32,
    /// UTF-8 String
    pub reason: String,
    /// If error code is 420 use this element, else ignore
    pub unknown_attributes: UnknownAttributes,
}

/// Unknown Attributes Error Struct
///
/// When error code is 420, use this struct
/// All 4 elements have to have a value, set to 0 if
///
/// # Examples
/// ```
/// let unknown_attributes = UnknownAttributes {
///     attribute_type1: 0x32c2,
///     attribute_type2: 0x2314,
///     attribute_type3: 0,
///     attribute_type4: 0,
/// }
/// ```
#[derive(Debug,Copy,Clone, Serialize, Deserialize)]
pub struct UnknownAttributes {
    pub attribute_type1: u16,
    pub attribute_type2: u16,
    pub attribute_type3: u16,
    pub attribute_type4: u16,
}

/// Validating headers
///
/// This server only accepts binding requests
/// Returns [`false`] if one of the attributes are not valid, else [`true`]
pub fn validate_headers(headers: Headers) -> bool{
    if headers.message_type != BINDING_REQUEST {
        return false;
    } else if headers.magic_cookie != MAGIC_COOKIE {
        return false;
    }
    true
}

pub fn success_response(socket_address: SocketAddr, transaction_id: u128) {
    let (iptype, x_address) = match socket_address.ip() {
        IpAddr::V6(ip) => (IPV6, (((format_bytes64(&mut ip.octets()) << 32) as u128) | (MAGIC_COOKIE as u128)) ^ transaction_id),
        IpAddr::V4(ip) => (IPV4, (format_bytes32(&mut ip.octets()) ^ MAGIC_COOKIE) as u128),
    };
    let x_port = socket_address.port() ^ ((MAGIC_COOKIE >> 16) as u16);

    let mut headers = Headers {
        message_type: SUCCESS_RESPONSE,
        message_length: 0,
        magic_cookie: MAGIC_COOKIE,
        transaction_id,
    };

    let mut attribute = Attribute {
        attribute_type: XOR_MAPPED_ADDRESS, // 2 bytes
        attribute_length: 0, // 2 bytes
    };

    let xor_mapped_address = XorMappedAddress {
        reserved: 0x0,
        family: iptype,
        x_port,
        x_address,
    };

    let xor_mapped_address_bytes = bincode::serialize(&xor_mapped_address).unwrap();
    attribute.attribute_length = xor_mapped_address_bytes.len() as u16;
    let attribute_bytes = bincode::serialize(&attribute).unwrap();
    headers.message_length = (attribute_bytes.len() + xor_mapped_address_bytes.len()) as u16;
    let headers_bytes = bincode::serialize(&headers).unwrap();

    println!("{:?}, {:?}, {:?}", headers, attribute, xor_mapped_address)

}

/// Process bytes of an request
///
/// This server only accepts binding requests
/// Returns [`false`] when headers is missing or invalid, else [`true`]
///
pub fn process_request(mut buffer: &mut[u8]) -> (bool, u128) {
    if buffer.len() < 20 {
        return (false, 0);
    }
    // Create headers
    let headers = Headers {
        message_type: format_bytes16(&mut buffer[0..2]),
        message_length: format_bytes16(&mut buffer[2..4]),
        magic_cookie: format_bytes32(&mut buffer[4..8]),
        transaction_id: format_bytes96(&mut buffer[8..20]),
    };

    if validate_headers(headers) {
        //When headers are valid
        return (true, headers.transaction_id);
    }
    (false, 0)
}

/// Formats byte array of 2 bytes into a unsigned 16 bit number
///
/// # Examples
/// ```
/// let mut buf:[u8;2] = [0x1, 0x1];
/// let answer = 0x11;
/// let number = format_bytes16(&buf);
/// assert_eq!(number, answer);
/// ```
pub fn format_bytes16( mut buf: &mut [u8]) -> u16 {
    let number = ((buf[0] as u16) << 8) | buf[1] as u16;
    number
}

/// Formats byte array of 4 bytes into a unsigned 32 bit number
///
/// # Examples
/// ```
/// let mut buf:[u8;4] = [0x1, 0x1, 0x2, 0x3];
/// let answer = 0x1123;
/// let number = format_bytes32(&buf);
/// assert_eq!(number, answer);
/// ```
pub fn format_bytes32( mut buf: &mut [u8]) -> u32 {
    let number = ((format_bytes16(&mut buf[0..2]) as u32) << 16) | (format_bytes16(&mut buf[2..4]) as u32);
    number
}

/// Formats byte array of 8 bytes into a unsigned 64 bit number
///
/// # Examples
/// ```
/// let mut buf:[u8;8] = [0x1, 0x1, 0x2, 0x3, 0x3, 0xb, 0xc, 0x9];
/// let answer = 0x11233bc9;
/// let number = format_bytes64(&buf);
/// assert_eq!(number, answer);
/// ```
pub fn format_bytes64( mut buf: &mut [u8]) -> u64 {
    let number = ((format_bytes32(&mut buf[0..4]) as u64) << 16) | (format_bytes32(&mut buf[4..8]) as u64);
    number
}
/// Formats byte array of 12 bytes into a unsigned 96 bit number as usize
///
/// # Examples
/// ```
/// let mut buf:[u8;12] = [0x1, 0x1, 0x4, 0x2, 0x1, 0x4, 0x1, 0xa, 0x4, 0x3, 0x2, 0x9];
/// let answer = 0x1142141a4329;
/// let number = format_bytes96(&buf);
/// assert_eq!(number, answer);
/// ```
pub fn format_bytes96( mut buf: &mut [u8]) -> u128 {
    let number = (format_bytes32(&mut buf[0..4]) as u128) << 64 | (format_bytes32(&mut buf[4..8]) as u128) << 32 | (format_bytes32(&mut buf[8..12]) as u128);
    number
}
