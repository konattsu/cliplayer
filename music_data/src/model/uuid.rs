// ref: https://datatracker.ietf.org/doc/html/rfc9562#name-uuid-version-4

const UUID4_VERSION: u8 = 0b0100;
const UUID_VARIANT: u8 = 0b10;

/// UUIDv4の正規表現
///
/// hyphen区切りの箇所でcapture
static RE_UUID4: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(
    || {
        regex::Regex::new(
            r"^([0-9a-fA-F]{8})-([0-9a-fA-F]{4})-([0-9a-fA-F]{4})-([0-9a-fA-F]{4})-([0-9a-fA-F]{12})$",
        )
        .unwrap()
    },
);

/// UUIDv4 (RFC 9562 | 4122)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct UuidVer4 {
    /// `数字`を格納
    bytes: [u8; 16],
}

// ! 入力は case-insensitive, 出力は lowerを使用
// ! ref: https://datatracker.ietf.org/doc/html/rfc4122#autoid-3

// MARK: External traits impl

impl std::str::FromStr for UuidVer4 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err("UUIDv4 must be 36 characters long");
        } else if !s.is_ascii() {
            return Err("UUIDv4 must be ASCII");
        }

        let caps = RE_UUID4.captures(s).ok_or("invalid UUIDv4 format")?;

        // 16進数の32文字 の文字列
        let hex_str = format!(
            "{}{}{}{}{}",
            &caps[1], &caps[2], &caps[3], &caps[4], &caps[5]
        );
        let hex_bytes = hex_str.as_bytes();

        /// asciiをhexに変換
        ///
        /// - `b'1'`は`0x31`, これを`0x01`に変換する
        /// - case-insensitive
        /// - `hex(0x0..=0xF)`以外は`None`を返す
        fn ascii_to_hex(b: u8) -> Option<u8> {
            match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
            }
        }

        let mut bytes = [0u8; 16];
        // 2^8 = 2^4*2 => 1バイトは2文字の16進数(hex)で表現される
        // そのため, 一度に2文字ずつ読み取る
        // 32(len) / 2 = 16
        for i in 0..16 {
            let high = ascii_to_hex(hex_bytes[i * 2]).ok_or("invalid hex character")?;
            let low =
                ascii_to_hex(hex_bytes[i * 2 + 1]).ok_or("invalid hex character")?;
            bytes[i] = (high << 4) | low;
        }

        // バージョン/バリアントをチェックする
        Self::from_bytes(bytes)
    }
}

impl std::fmt::Display for UuidVer4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid_str = Self::bytes_to_uuid_string(&self.bytes);
        write!(f, "{uuid_str}")
    }
}

impl std::fmt::Debug for UuidVer4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UuidVer4({self}, hex:{:?})", self.bytes_hex())
    }
}

impl<'de> serde::Deserialize<'de> for UuidVer4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr;
        struct UuidVer7Visitor;

        impl<'de> serde::de::Visitor<'de> for UuidVer7Visitor {
            type Value = UuidVer4;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a valid UUIDv4 string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                UuidVer4::from_str(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(UuidVer7Visitor)
    }
}

impl serde::Serialize for UuidVer4 {
    /// lowercaseのUUIDv4の文字列
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// MARK: Methods

impl UuidVer4 {
    pub(crate) fn generate() -> Self {
        // 乱数を生成
        let (rand_a, rand_b, rand_c) = Self::generate_rand();
        // UUIDv4を生成
        Self::generate_with_rand(rand_a, rand_b, rand_c)
    }

    /// UUIDv4のバイト列から新規作成
    ///
    /// - Error: `bytes`がUUIDv4の形式でないとき
    fn from_bytes(bytes: [u8; 16]) -> Result<Self, &'static str> {
        if !Self::is_uuid_ver4(&bytes) {
            return Err("invalid UUIDv4 format");
        }
        Ok(UuidVer4 { bytes })
    }

    /// 乱数を基にUUIDv4を作成
    fn generate_with_rand(rand_a: u64, rand_b: u16, rand_c: u64) -> Self {
        let mut bytes = [0u8; 16];

        // 1. rand_a (48bit)
        //   64bitのうち上位16bitを捨てる
        bytes[0..6].copy_from_slice(&rand_a.to_be_bytes()[2..8]);

        // 2. version (4bit) + rand_b (12bit)
        //   bytes[6]: ver(上位4bitに配置) | rand_bの上位4bit(下位4bitに配置)
        bytes[6] = (UUID4_VERSION << 4) | ((rand_b >> 8) as u8 & 0x0F);
        //   bytes[7]: rand_bの下位8bit
        bytes[7] = (rand_b & 0xFF) as u8;

        // 3. variant (2bit) + rand_c (62bit)
        //   bytes[8]: variant(上位2bitに配置), rand_cの上位6bit(下位6bitに配置)
        bytes[8] = (UUID_VARIANT << 6) | ((rand_c >> 56) as u8 & 0b0011_1111);
        //   bytes[9..16]: rand_cの下位56bit
        bytes[9..16].copy_from_slice(&rand_c.to_be_bytes()[1..8]);

        UuidVer4 { bytes }
    }

    /// `rand_a`(48bit), `rand_b`(12bit), `rand_c`(62bit) の値をランダムに生成
    fn generate_rand() -> (u64, u16, u64) {
        use rand::Rng;

        let mut rng = rand::rng();
        let rand_48bit: u64 = rng.random_range(0..1 << 48); // 0..2^48
        let rand_12bit: u16 = rng.random_range(0..1 << 12); // 0..2^12
        let rand_62bit: u64 = rng.random_range(0..1 << 62); // 0..2^62
        (rand_48bit, rand_12bit, rand_62bit)
    }

    /// UUIDv7のバイト列がUUIDv4の形式であるかどうか
    fn is_uuid_ver4(bytes: &[u8; 16]) -> bool {
        // 上位4bitが0b0100
        let is_version4 = (bytes[6] >> 4) == UUID4_VERSION;
        // 上位2bitが0b10
        let is_variant = (bytes[8] >> 6) == UUID_VARIANT;

        is_version4 && is_variant
    }

    /// バイト列をUUIDv4の文字列に変換
    ///
    /// - 間に適切にハイフンを挿入する
    fn bytes_to_uuid_string(bytes: &[u8; 16]) -> String {
        let mut s = String::with_capacity(36);
        for (i, byte) in bytes.iter().enumerate() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                s.push('-');
            }
            s.push_str(&format!("{byte:02x}"));
        }
        s
    }

    fn bytes_hex(&self) -> String {
        self.bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
}

// MARK: For Tests

#[cfg(test)]
impl UuidVer4 {
    /// `00000000-0000-4000-8000-000000000000`
    pub(crate) fn self_1() -> Self {
        let rand_a: u64 = 0x0;
        let rand_b: u16 = 0x0;
        let rand_c: u64 = 0x0;
        Self::generate_with_rand(rand_a, rand_b, rand_c)
    }

    /// `11111111-1111-4111-9111-111111111111`
    pub(crate) fn self_2() -> Self {
        let rand_a: u64 = 0x11_11_11_11_11_11;
        let rand_b: u16 = 0x1_11;
        // 上位6bit            variant付与後
        // 0b01_0001 (0x11)   0b1001_0001 (0x91)
        let rand_c: u64 = 0x11_11_11_11_11_11_11_11;
        Self::generate_with_rand(rand_a, rand_b, rand_c)
    }

    /// `22222222-2222-4222-a222-222222222222`
    pub(crate) fn self_3() -> Self {
        let rand_a: u64 = 0x22_22_22_22_22_22;
        let rand_b: u16 = 0x2_22;
        // 上位6bit            variant付与後
        // 0b10_0010 (0x22)   0b1010_0010 (0xa1)
        let rand_c: u64 = 0x22_22_22_22_22_22_22_22;
        Self::generate_with_rand(rand_a, rand_b, rand_c)
    }

    /// `33333333-3333-4333-b333-333333333333`
    pub(crate) fn self_4() -> Self {
        let rand_a: u64 = 0x33_33_33_33_33_33;
        let rand_b: u16 = 0x3_33;
        // 上位6bit            variant付与後
        // 0b11_0011 (0x03)   0b1011_0011 (0xb3)
        let rand_c: u64 = 0x33_33_33_33_33_33_33_33;
        Self::generate_with_rand(rand_a, rand_b, rand_c)
    }

    /// `33333333-(任意)-4333-b333-333333333333`
    pub(crate) fn self_partly_rand(rand: u16) -> Self {
        let uuid = Self::self_4();
        let mut bytes = uuid.bytes;
        let rand = rand.to_be_bytes();
        bytes[6] = rand[0];
        bytes[7] = rand[1];
        Self { bytes }
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_ver4_generate_1() {
        let uuid = UuidVer4::self_1();

        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                // [0..6] rand_a (0)
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                // [6] version (0b0100) | rand_b (0)
                0x40,
                // [7] rand_b (0)
                0x0,
                // [8] variant (0b10) | rand_c (0b00_0000)
                0b1000_0000,
                // [9..16] rand_c (0)
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
            ]
        );

        assert_eq!(uuid.to_string(), "00000000-0000-4000-8000-000000000000");
    }

    #[test]
    fn test_uuid_ver4_generate_2() {
        let uuid = UuidVer4::self_2();

        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                // [0..6] rand_a
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                // [6] version (0b0100) | rand_b
                0x41,
                // [7] rand_b
                0x11,
                // [8] variant (0b10) | rand_c (0b01_0001)
                0b1001_0001,
                // [9..16] rand_c
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11
            ]
        );

        assert_eq!(uuid.to_string(), "11111111-1111-4111-9111-111111111111");
    }

    #[test]
    fn test_uuid_ver4_generate_3() {
        let uuid = UuidVer4::self_3();

        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                // [0..6] rand_a
                0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                // [6] version (0b0100) | rand_b
                0x42,
                // [7] rand_b
                0x22,
                // [8] variant (0b10) | rand_c (0b0010_0010)
                0b1010_0010,
                // [9..16] rand_c
                0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22
            ]
        );

        assert_eq!(uuid.to_string(), "22222222-2222-4222-a222-222222222222");
    }

    #[test]
    fn test_uuid_ver4_generate_4() {
        let uuid = UuidVer4::self_4();

        println!("uuid: {uuid:02x?}");

        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                // [0..6] rand_a
                0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                // [6] version (0b0100) | rand_b
                0x43,
                // [7] rand_b
                0x33,
                // [8] variant (0b10) | rand_c (0b0010_0010)
                0b1011_0011,
                // [9..16] rand_c
                0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33
            ]
        );

        assert_eq!(uuid.to_string(), "33333333-3333-4333-b333-333333333333");
    }

    #[test]
    fn test_uuid_ver4_is_uuid_ver7_valid() {
        let uuid = UuidVer4::self_1();
        assert!(UuidVer4::is_uuid_ver4(&uuid.bytes));
        let uuid = UuidVer4::self_2();
        assert!(UuidVer4::is_uuid_ver4(&uuid.bytes));
        let uuid = UuidVer4::self_3();
        assert!(UuidVer4::is_uuid_ver4(&uuid.bytes));
        let uuid = UuidVer4::self_4();
        assert!(UuidVer4::is_uuid_ver4(&uuid.bytes));
    }

    #[test]
    fn test_uuid_ver4_is_uuid_ver7_invalid() {
        // バージョンビットが違う
        let mut bytes = UuidVer4::self_1().bytes;
        bytes[6] = 0x50; // 上位4bitが0b0101 (ver5)
        assert!(!UuidVer4::is_uuid_ver4(&bytes));

        // バリアントビットが違う
        let mut bytes = UuidVer4::self_1().bytes;
        bytes[8] = 0x00; // 上位2bitが0b00
        assert!(!UuidVer4::is_uuid_ver4(&bytes));
    }

    #[test]
    fn test_uuid_ver4_deserialization_valid() {
        let uuid = UuidVer4::self_3();

        // lower case
        let json = "\"22222222-2222-4222-a222-222222222222\"";
        let result: Result<UuidVer4, _> = serde_json::from_str(json);
        assert_eq!(result.unwrap(), uuid);

        // upper case
        let json = "\"22222222-2222-4222-A222-222222222222\"";
        let result: Result<UuidVer4, _> = serde_json::from_str(json);
        assert_eq!(result.unwrap(), uuid);
    }

    #[test]
    fn test_uuid_ver4_deserialization_invalid() {
        // バージョンビットが違う
        let mut bytes = UuidVer4::self_1().bytes;
        bytes[6] = 0x50;
        let json = format!("\"{}\"", UuidVer4::bytes_to_uuid_string(&bytes));
        let result: Result<UuidVer4, _> = serde_json::from_str(&json);
        assert!(result.is_err());

        // バリアントビットが違う
        let mut bytes = UuidVer4::self_1().bytes;
        bytes[8] = 0x00;
        let json = format!("\"{}\"", UuidVer4::bytes_to_uuid_string(&bytes));
        let result: Result<UuidVer4, _> = serde_json::from_str(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_uuid_ver4_display() {
        let uuid = UuidVer4::self_1();
        let expected = "00000000-0000-4000-8000-000000000000";
        assert_eq!(uuid.to_string(), expected);
        let uuid = UuidVer4::self_2();
        let expected = "11111111-1111-4111-9111-111111111111";
        assert_eq!(uuid.to_string(), expected);
    }
}
