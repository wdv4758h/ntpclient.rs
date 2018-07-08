/// NTP v3: https://tools.ietf.org/html/rfc1305
/// NTP v4: https://tools.ietf.org/html/rfc5905
/// NTP v4 extend: https://tools.ietf.org/html/rfc7822
#[derive(Debug, Default)]
#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0")]
#[packed_struct(endian="msb")]
pub struct NTPPacket {
    #[packed_field(bits="0..=1")]
    pub leap_indicator: u8,
    #[packed_field(bits="2..=4")]
    pub version_number: u8,
    #[packed_field(bits="5..=7")]
    pub mode: u8,
    #[packed_field(bits="8..=15")]
    pub stratum: u8,
    #[packed_field(bits="16..=23")]
    pub poll: u8,
    #[packed_field(bits="24..=31")]
    pub precision: u8,
    #[packed_field(bits="32..=63")]
    pub root_delay: u32,
    #[packed_field(bits="64..=95")]
    pub root_dispersion: u32,
    #[packed_field(bits="96..=127")]
    pub reference_id: u32,
    #[packed_field(bits="128..=159")]
    pub reference_timestamp: u32,
    #[packed_field(bits="160..=191")]
    pub reference_timestamp_fraction: u32,
    #[packed_field(bits="192..=223")]
    pub original_timestamp: u32,
    #[packed_field(bits="224..=255")]
    pub original_timestamp_fraction: u32,
    #[packed_field(bits="256..=287")]
    pub receive_timestamp: u32,
    #[packed_field(bits="288..=319")]
    pub receive_timestamp_fraction: u32,
    #[packed_field(bits="320..=351")]
    pub transmit_timestamp: u32,
    #[packed_field(bits="352..=383")]
    pub transmit_timestamp_fraction: u32,
}

impl NTPPacket {
    pub fn new() -> Self {
        NTPPacket {
            leap_indicator: 0,
            version_number: 3,      // NTP version
            mode: 3,                // client mode
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay: 0,
            root_dispersion: 0,
            reference_id: 0,
            reference_timestamp: 0,
            reference_timestamp_fraction: 0,
            original_timestamp: 0,
            original_timestamp_fraction: 0,
            receive_timestamp: 0,
            receive_timestamp_fraction: 0,
            transmit_timestamp: 0,
            transmit_timestamp_fraction: 0,
        }
    }
}
