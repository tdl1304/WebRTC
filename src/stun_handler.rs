use std::convert::TryFrom;
use std::net::{IpAddr, SocketAddr};
#[allow(deprecated)]
use bincode::{config};
use serde::{Deserialize, Serialize};

// Header constants
const MAGIC_COOKIE: u32 = 0x2112a442;
const SUCCESS_RESPONSE: u16 = 0x0101;
const BINDING_REQUEST: u16 = 0x0001;
const ERROR_RESPONSE: u16 = 0x0111;
// Attribute types constants
const ERROR_ATTRIBUTE: u16 = 0x0009;
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
///     transaction_id: [34,a7,6b,c5,f4,a9,d7,64,d9,68,01,40],
/// };
/// ```
/// This is then followed up by an attribute header if this were a response
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Headers {
    /// Type of STUN-message
    pub message_type: u16,
    /// Length of attribute payload
    pub message_length: u16,
    /// Magic cookie is always set to 0x2112a442
    pub magic_cookie: u32,
    /// 96 bit identifier, this is always set by the client
    pub transaction_id: [u8; 12], //12 bytes
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Attribute {
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

/// XOR Mapped Address attribute for IPv6
///
/// Attribute in a STUN response for a binding request
///
/// # Examples
/// ```
///  let attribute = XorMappedAddress {
///     reserved: 0x00,
///     family: 0x01,
///     x_port: 0x4acd,
///     x_address: 0x76c084f741254523544467123aed3453,
/// }
/// ```
///
/// For more information about format visit [`RFC5389`]
///
/// [`RFC5389`]:https://tools.ietf.org/html/rfc8489#section-14.1
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct XorMappedAddressV6 {
    /// Always set to 0
    pub reserved: u8,
    /// IPv4 or IPv6
    pub family: u8,
    /// XOR'ed port number with the most significant 16 bits of the magic cookie
    pub x_port: u16,
    /// IPv6: Ip address XOR'ed with [`magic cookie`] extended with [`Transaction ID`]
    ///
    /// [`magic cookie`]:struct.Headers.html#structfield.MAGIC_COOKIE
    /// [`Transaction ID`]:struct.Headers.html#structfield.transaction_id
    pub x_address: u128,
}

/// XOR Mapped Address attribute for IPv4
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct XorMappedAddressV4 {
    /// Always set to 0
    pub reserved: u8,
    /// IPv4 or IPv6
    pub family: u8,
    /// XOR'ed port number with the most significant 16 bits of the magic cookie
    pub x_port: u16,
    /// IPv4: XOR'ed ip address with the [`magic cookie`]
    /// [`magic cookie`]:struct.Headers.html#structfield.MAGIC_COOKIE
    /// [`Transaction ID`]:struct.Headers.html#structfield.transaction_id
    pub x_address: u32,
}

/// Error Code Attribute
///
/// Attribute struct for an error response
///
/// # Examples
/// ```
/// let error = Error {
///     error_code: [0,0,5,0],
///     reason: String::from("Internal server error"),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Error {
    ///Number between 300-600
    ///
    ///third byte is class, 4th byte is error code
    pub error_code: [u8; 4],
    /// UTF-8 String, length must be divisble by 4
    pub reason: String,
}

/// Error Code 420 wrapper
///
/// Attribute struct for an error response 420
///
/// # Examples
/// ```
/// let error = Error420 {
///     error_code: [0,0,4,20],
///     unknown_attributes: [0 as u16, 0, 0 ,0],
/// };
/// ```
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Error420 {
    pub error_code: [u8; 4],
    pub unknown_attributes: [u16; 4],
}

/// Validating headers
///
/// This server only accepts binding requests
/// Returns [`false`] and array of unknown attributes if one of the attributes are not valid, else [`true`] and array with zeros
fn validate_headers(headers: Headers) -> (bool, [u16; 4]) {
    if headers.message_type == BINDING_REQUEST {
        return (false, [headers.message_type, 0, 0, 0]);
    } else if headers.magic_cookie != MAGIC_COOKIE {
        return (
            false,
            [
                (headers.magic_cookie >> 16) as u16,
                headers.magic_cookie as u16,
                0,
                0,
            ],
        );
    }
    (true, [0, 0, 0, 0])
}

/// Creates bytes for a successful binding request
///
/// Supports IPv4 and IPv6.
/// Creates success headers, attribute headers and xor-mapped-ipaddress, then serializes it using
/// bincode and serde with big endian configuration.
///
/// # Examples
/// ```
/// let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 45321);
/// let transaction_id = [ec, cb, 9a, 1a, aa, a9, cd, 9, 0, 0, 0, 0]
/// let bytes = success_response(socket_address, transaction_id);
/// println!("{:#x?}", bytes);
/// // Expected output: [1, 1, 0, c, 21, 12, a4, 42, ec, cb, 9a, 1a, aa, a9, cd, 9, 0, 0, 0, 0, 0, 20, 0, 8, 0, 1, 90, 1b, 5e, 12, a4, 43]
/// ```
fn success_response(socket_address: SocketAddr, transaction_id: [u8; 12]) -> Vec<u8> {
    let x_port = socket_address.port() ^ ((MAGIC_COOKIE >> 16) as u16);

    let headers = Headers {
        message_type: SUCCESS_RESPONSE,
        message_length: 0,
        magic_cookie: MAGIC_COOKIE,
        transaction_id,
    };

    let attribute = Attribute {
        attribute_type: XOR_MAPPED_ADDRESS, // 2 bytes
        attribute_length: 0,                // 2 bytes
    };

    return match socket_address.ip() {
        IpAddr::V6(ip) => {
            let xor_mapped_address = XorMappedAddressV6 {
                reserved: 0x0,
                family: IPV6,
                x_port,
                x_address: (format_bytes128(&ip.octets())
                    ^ (((MAGIC_COOKIE as u128) << 96) | format_bytes96(&transaction_id))),
            };
            _serialize(xor_mapped_address, attribute, headers)
        }
        IpAddr::V4(ip) => {
            let xor_mapped_address = XorMappedAddressV4 {
                reserved: 0x0,
                family: IPV4,
                x_port,
                x_address: (format_bytes32(&ip.octets()) ^ MAGIC_COOKIE),
            };
            _serialize(xor_mapped_address, attribute, headers)
        }
    };
}

/// Creates bytes for a error message binding request
///
/// Creates error headers, attribute headers and error message, then serializes it using
/// bincode and serde with big endian configuration.
///
/// # Examples
/// ```
/// let transaction_id = [ec, cb, 9a, 1a, aa, a9, cd, 9, 0, 0, 0, 0]
/// let bytes = error_response(500, String::from("Foo bar"), transaction_id);
/// println!("{:#x?}", bytes);
/// // Expected output: [1, 11, 0, 10, 21, 12, a4, 42, ec, cb, 9a, 1a, aa, a9, cd, 9, 0, 0, 0, 0, 0, 9, 0, a, 0, 0, 0, 1, f4, 46 6f 6f 20 62 61 72]
/// ```
fn error_response(error_code: usize, reason: String, transaction_id: [u8; 12]) -> Vec<u8> {
    let error: [u8; 4] = [0, 0, (error_code / 100) as u8, (error_code % 100) as u8];
    let headers = Headers {
        message_type: ERROR_RESPONSE,
        message_length: 0,
        magic_cookie: MAGIC_COOKIE,
        transaction_id,
    };
    let attribute = Attribute {
        attribute_type: ERROR_ATTRIBUTE,
        attribute_length: 0,
    };

    let error = Error {
        error_code: error,
        reason,
    };

    _serialize(error, attribute, headers)
}

/// Creates bytes for a error message 420 binding request
///
/// Creates error headers, attribute headers and error message, then serializes it using
/// bincode and serde with big endian configuration.
/// Supports a total of 4 unknown attributes, must be 16 bits each.
///
/// # Examples
/// ```
/// let transaction_id = [ec, cb, 9a, 1a, aa, a9, cd, 9, 0, 0, 0, 0];
///
/// let bytes = error_response420([0x1212,0,0,0] as [u16;4], transaction_id);
/// println!("{:#x?}", bytes);
/// ```
fn error_response420(unknown_attributes: [u16; 4], transaction_id: [u8; 12]) -> Vec<u8> {
    let headers = Headers {
        message_type: ERROR_RESPONSE,
        message_length: 0,
        magic_cookie: MAGIC_COOKIE,
        transaction_id,
    };

    let attribute = Attribute {
        attribute_type: UNKNOWN_ATTRIBUTES,
        attribute_length: 0,
    };

    let error = Error420 {
        error_code: [0, 0, 4, 20],
        unknown_attributes,
    };

    _serialize(error, attribute, headers)
}

/// Process bytes of an request
///
/// This server only accepts binding requests
/// Returns byte buffer response when headers are not missing or invalid, else error buffer
pub fn process_request(buffer: &[u8], socket_address: SocketAddr) -> Vec<u8> {
    if buffer.len() < 20 {
        return error_response(
            400,
            String::from("Bad request."),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] as [u8; 12],
        );
    }

    let headers = Headers {
        message_type: format_bytes16(&buffer[0..2]),
        message_length: format_bytes16(&buffer[2..4]),
        magic_cookie: format_bytes32(&buffer[4..8]),
        transaction_id: <[u8; 12]>::try_from(&buffer[8..20]).unwrap(),
    };

    let (valid, unknown_attributes) = validate_headers(headers);

    return if valid {
        //When headers are valid
        success_response(socket_address, headers.transaction_id)
    } else {
        //When some elements are unknown
        error_response420(unknown_attributes, headers.transaction_id)
    };
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
fn format_bytes16(buf: &[u8]) -> u16 {
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
fn format_bytes32(buf: &[u8]) -> u32 {
    let number = ((format_bytes16(&buf[0..2]) as u32) << 16) | (format_bytes16(&buf[2..4]) as u32);
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
fn format_bytes64(buf: &[u8]) -> u64 {
    let number = ((format_bytes32(&buf[0..4]) as u64) << 16) | (format_bytes32(&buf[4..8]) as u64);
    number
}

/// Formats byte array of 12 bytes into a unsigned 96 bit number as u128
///
/// # Examples
/// ```
/// let mut buf:[u8;12] = [0x1, 0x1, 0x4, 0x2, 0x1, 0x4, 0x1, 0xa, 0x4, 0x3, 0x2, 0x9];
/// let answer = 0x1142141a4329;
/// let number = format_bytes96(&buf);
/// assert_eq!(number, answer);
/// ```
fn format_bytes96(buf: &[u8; 12]) -> u128 {
    let number = (format_bytes32(&buf[0..4]) as u128) << 64
        | (format_bytes32(&buf[4..8]) as u128) << 32
        | (format_bytes32(&buf[8..12]) as u128);
    number
}

/// Formats byte array of 16 bytes into a unsigned 128 bit number as u128
///
/// # Examples
/// ```
/// let mut buf:[u8;16] = [0x1, 0x1, 0x4, 0x2, 0x1, 0x4, 0x1, 0xa, 0x4, 0x3, 0x2, 0x9, 0x3, 0x2, 0xa, 0x1];
/// let answer = 0x1142141a432932a1;
/// let number = format_bytes96(&buf);
/// assert_eq!(number, answer);
/// ```
fn format_bytes128(buf: &[u8]) -> u128 {
    let number = (format_bytes64(&buf[0..8]) as u128) << 64 | (format_bytes64(&buf[8..16]) as u128);
    number
}

/// Serialize structs (T, [`Attribute`], [`Headers`])
///
/// Big endian mode
///
/// [`Attribute`]:struct.Attribute.html
/// [`Headers`]:struct.Headers.html
#[allow(deprecated)]
fn _serialize<T: Serialize>(_struct: T, mut attribute: Attribute, mut headers: Headers) -> Vec<u8> {
    let t_bytes = config().big_endian().serialize(&_struct).unwrap();
    attribute.attribute_length = t_bytes.len() as u16;
    let attribute_bytes = config().big_endian().serialize(&attribute).unwrap();
    headers.message_length = (attribute_bytes.len() + t_bytes.len()) as u16;
    let headers_bytes = config().big_endian().serialize(&headers).unwrap();
    [headers_bytes, attribute_bytes, t_bytes].concat()
}
