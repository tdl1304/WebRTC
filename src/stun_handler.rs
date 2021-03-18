// Header constants
const MAGIC_COOKIE: u32 = 0x2112a442;
const SUCCESS_RESPONSE: u16 = 0x0101;
const BINDING_REQUEST: u16 = 0x0001;

// Attribute types constants
const ERROR_CODE: u16 = 0x0009;
const UNKNOWN_ATTRIBUTES: u16 = 0x000a;
const MAPPED_ADDRESS: u16 = 0x0001;
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
pub struct Headers {
    /// Type of STUN-message
    pub message_type: u16,
    /// Length of attribute payload
    pub message_length: u16,
    /// Magic cookie is always set to 0x2112a442
    pub magic_cookie: u32,
    /// 96 bit identifier, this is always set by the client
    pub transaction_id: usize, //96 bit only accepted
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
pub struct XorMappedAddress {
    /// Always set to 0
    pub reserved: u8,
    /// IPv4 or IPv6
    pub family: u8,
    /// XOR'ed port number with the most significant 16 bits of the magic cookie
    pub x_port: u16,
    /// IPv4: XOR'ed ip address with the [`magic cookie`]
    ///
    ///IPv6: Ip address extended with [`magic cookie`] XOR'ed with [`Transaction ID`]
    ///
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
///     error_code: 500,
///     reason: String::from("Internal server error"),
///     unknown_attributes: std::ptr::null(),
/// }
/// ```
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
pub struct UnknownAttributes {
    pub attribute_type1: u16,
    pub attribute_type2: u16,
    pub attribute_type3: u16,
    pub attribute_type4: u16,
}
