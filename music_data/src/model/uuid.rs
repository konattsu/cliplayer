// ref: https://datatracker.ietf.org/doc/html/rfc9562#name-uuid-version-7

const UUID7_VER: u8 = 0b0111;
const UUID_VAR: u8 = 0b10;

static RE_UUID7: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(
    || {
        regex::Regex::new(
            r"^([0-9a-fA-F]{8})-([0-9a-fA-F]{4})-([0-9a-fA-F]{4})-([0-9a-fA-F]{4})-([0-9a-fA-F]{12})$",
        )
        .unwrap()
    },
);

/// UUIDv7 (RFC 9562)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UuidVer7 {
    /// `数字`を格納
    bytes: [u8; 16],
}

// ! 入力は case-insensitive, 出力は lowerを使用
// ! ref: https://datatracker.ietf.org/doc/html/rfc4122#autoid-3

// MARK: External traits impl

impl std::str::FromStr for UuidVer7 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err("UUIDv7 must be 36 characters long");
        } else if !s.is_ascii() {
            return Err("UUIDv7 must be ASCII");
        }

        let caps = RE_UUID7.captures(s).ok_or("invalid UUIDv7 format")?;

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

impl std::fmt::Display for UuidVer7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid_str = Self::bytes_to_uuid_string(&self.bytes);
        write!(f, "{}", uuid_str)
    }
}

impl<'de> serde::Deserialize<'de> for UuidVer7 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr;
        struct UuidVer7Visitor;

        impl<'de> serde::de::Visitor<'de> for UuidVer7Visitor {
            type Value = UuidVer7;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a valid UUIDv7 string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                UuidVer7::from_str(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(UuidVer7Visitor)
    }
}

impl serde::Serialize for UuidVer7 {
    /// lowercaseのUUIDv7の文字列
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// MARK: Methods

impl UuidVer7 {
    /// UUIDv7のバイト列から新規作成
    ///
    /// - Error: `bytes`がUUIDv7の形式でないとき
    /// - i.e. バージョンビットが0b0111でない || バリアントビットが0b10でない
    pub fn from_bytes(bytes: [u8; 16]) -> Result<Self, &'static str> {
        if !Self::is_uuid_ver7(&bytes) {
            return Err("invalid UUIDv7 format");
        }
        Ok(UuidVer7 { bytes })
    }

    /// 日付を基にUUIDv7を新規作成
    pub fn generate(datetime: &crate::model::VideoPublishedAt) -> Self {
        let (rand_a, rand_b) = Self::generate_rand();
        Self::generate_with_rand(datetime, rand_a, rand_b)
    }

    /// 日付と乱数を基にUUIDv7を作成
    ///
    /// - Error: `datetime`をタイムスタンプに変換すると, 48bit符号なし整数で表現できないとき
    ///   - i.e. 1970年1月1日 - 約10895年でないとき
    fn generate_with_rand(
        datetime: &crate::model::VideoPublishedAt,
        rand_a: u16,
        rand_b: u64,
    ) -> Self {
        let mut bytes = [0u8; 16];
        let datetime_millis = datetime.as_chrono_datetime().timestamp_millis();
        assert!((0..0xFFFF_FFFF_FFFF).contains(&datetime_millis));

        // `datetime_millis`の最大値が`0xFFFF_FFFF_FFFF - 1`なのでu64にキャスト変換できる
        let timestamp_millis_bytes = (datetime_millis as u64).to_be_bytes();

        // 1. timestamp (48bit, big-endian)
        //   64bitのうち上位16bitを捨てる
        bytes[0..6].copy_from_slice(&timestamp_millis_bytes[2..8]);

        // 2. version (4bit) + rand_a (12bit)
        //   bytes[6]: ver(上位4bitに配置) | rand_aの上位4bit(下位4bitに配置)
        bytes[6] = (UUID7_VER << 4) | ((rand_a >> 8) as u8 & 0x0F);
        //   bytes[7]: rand_aの下位8bit
        bytes[7] = (rand_a & 0xFF) as u8;

        // 3. variant (2bit) + rand_b (62bit)
        //   bytes[8]: variant(上位2bitに配置), rand_bの上位6bit(下位6bitに配置)
        bytes[8] = (UUID_VAR << 6) | ((rand_b >> 56) as u8 & 0b0011_1111);
        //   bytes[9..16]: rand_bの下位48bit
        let rand_b_bytes = rand_b.to_be_bytes();
        bytes[9..16].copy_from_slice(&rand_b_bytes[1..8]);

        UuidVer7 { bytes }
    }

    /// `rand_a`(12bit) かつ `rand_b`(62bit) の値をランダムに生成
    fn generate_rand() -> (u16, u64) {
        use rand::Rng;

        let mut rng = rand::rng();
        let rand_12bit: u16 = rng.random_range(0..1 << 12); // 0..2^12
        let rand_62bit: u64 = rng.random_range(0..1 << 62); // 0..2^62
        (rand_12bit, rand_62bit)
    }

    /// UUIDv7のバイト列がUUIDv7の形式であるかどうか
    fn is_uuid_ver7(bytes: &[u8; 16]) -> bool {
        // 上位4bitが0b0111
        let is_version7 = (bytes[6] >> 4) == UUID7_VER;
        // 上位2bitが0b10
        let is_variant = (bytes[8] >> 6) == UUID_VAR;

        is_version7 && is_variant
    }

    /// バイト列をUUIDv7の文字列に変換
    ///
    /// - 間に適切にハイフンを挿入する
    ///   - e.g. `0197c644-22af-7824-bc9e-24107fdd397b`
    fn bytes_to_uuid_string(bytes: &[u8; 16]) -> String {
        let mut s = String::with_capacity(36);
        for (i, byte) in bytes.iter().enumerate() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                s.push('-');
            }
            s.push_str(&format!("{:02x}", byte));
        }
        s
    }

    /// UUIDv7の値から内部に含まれるタイムスタンプを取得
    pub fn get_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        use chrono::TimeZone;

        // bytes[0..6]はtimestampの上位48bit
        let mut timestamp_millis_bytes: [u8; 8] = [0; 8];
        timestamp_millis_bytes[2..8].copy_from_slice(&self.bytes[0..6]);
        let timestamp_millis = i64::from_be_bytes(timestamp_millis_bytes);

        // 48bit符号なし整数なので, 負の値にはならない
        assert!(
            timestamp_millis >= 0,
            "timestamp_millis must be non-negative"
        );
        chrono::Utc.timestamp_millis_opt(timestamp_millis).unwrap()
    }
}

// MARK: Tests

#[cfg(test)]
impl UuidVer7 {
    /// `0193bac8-a560-7000-8000-000000000000`
    ///
    /// `2024-12-12T12:12:12Z`
    pub fn self_1() -> Self {
        use chrono::TimeZone;
        let datetime = crate::model::VideoPublishedAt::new(
            chrono::Utc
                .with_ymd_and_hms(2024, 12, 12, 12, 12, 12)
                .unwrap(),
        )
        .unwrap();
        // 2024-12-12T12:12:12Z
        Self::generate_with_rand(&datetime, 0x0, 0x0)
    }

    /// `01920ef6-46d0-70f0-8000-000f0f0f0f0f`
    ///
    /// `2024-09-20T10:24:34Z`
    pub fn self_2() -> Self {
        use chrono::TimeZone;
        let datetime = crate::model::VideoPublishedAt::new(
            chrono::Utc
                .with_ymd_and_hms(2024, 9, 20, 10, 24, 34)
                .unwrap(),
        )
        .unwrap();
        // 2024-09-20T10:24:34Z
        Self::generate_with_rand(&datetime, 0xF0, 0x0F0F0F0F0F)
    }

    // ref: https://datatracker.ietf.org/doc/html/rfc9562#name-example-of-a-uuidv7-value
    /// `017f22e2-79b0-7cc3-98c4-dc0c0c07398f`
    ///
    /// `2022-02-22T19:22:22Z`
    pub fn self_3() -> Self {
        use chrono::TimeZone;
        let datetime = crate::model::VideoPublishedAt::new(
            chrono::Utc
                .with_ymd_and_hms(2022, 2, 22, 19, 22, 22)
                .unwrap(),
        )
        .unwrap();
        // 2022-02-22T19:22:22Z
        Self::generate_with_rand(&datetime, 0xCC3, 0x18C4DC0C0C07398F)
    }

    /// `0193b63c-5eb0-70f0-8000-000f0f0f0f0f`
    ///
    /// `2024-12-12T00:00:30Z`
    pub fn self_4() -> Self {
        use chrono::TimeZone;
        let datetime = crate::model::VideoPublishedAt::new(
            chrono::Utc
                .with_ymd_and_hms(2024, 12, 12, 0, 0, 30)
                .unwrap(),
        )
        .unwrap();
        // 2024-12-12T00:00:30Z
        Self::generate_with_rand(&datetime, 0xF0, 0x0F0F0F0F0F)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_ver7_generate_1() {
        let uuid = UuidVer7::self_1();

        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                // [0..6] timestamp (2024-12-12T12:12:12Z)
                0x01, 0x93, 0xBA, 0xC8, 0xA5, 0x60,
                // [6] version (0x7) | rand_a (0x0)
                0x70,
                // [7] rand_a (0)
                0x00,
                // [8] variant (0b10) | rand_b (0b00_0000)
                0b1000_0000,
                // [9..16] rand_b (0)
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        );

        assert_eq!(uuid.to_string(), "0193bac8-a560-7000-8000-000000000000");
    }

    #[test]
    fn test_uuid_ver7_generate_2() {
        let uuid = UuidVer7::self_2();

        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                // [0..6] timestamp (2024-9-20T10:24:34Z)
                0x01, 0x92, 0x0E, 0xF6, 0x46, 0xD0,
                // [6] version (0x7) | rand_a
                0x70,
                // [7] rand_a
                0xF0,
                // [8] variant (0b10) | rand_b
                0b1000_0000,
                // [9..16] rand_b
                0x00, 0x00, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F,
            ]
        );

        assert_eq!(uuid.to_string(), "01920ef6-46d0-70f0-8000-000f0f0f0f0f");
    }

    #[test]
    fn test_uuid_ver7_generate_3() {
        let uuid = UuidVer7::self_3();

        // ref: https://datatracker.ietf.org/doc/html/rfc9562#name-example-of-a-uuidv7-value
        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                0x01, 0x7F, 0x22, 0xE2, 0x79, 0xB0, 0x7C, 0xC3,
                0x98, 0xC4, 0xDC, 0x0C, 0x0C, 0x07, 0x39, 0x8F,
            ]
        );

        assert_eq!(uuid.to_string(), "017f22e2-79b0-7cc3-98c4-dc0c0c07398f");
    }

    #[test]
    fn test_uuid_ver7_generate_4() {
        let uuid = UuidVer7::self_4();

        println!("uuid: {:02x?}", uuid);

        #[rustfmt::skip]
        assert_eq!(
            uuid.bytes,
            [
                // [0..6] timestamp (2024-12-12T00:00:30Z)
                0x01, 0x93, 0xB8, 0x2A, 0xC1, 0x30,
                // [6] version (0x7) | rand_a
                0x70,
                // [7] rand_a
                0xF0,
                // [8] variant (0b10) | rand_b
                0b1000_0000,
                // [9..16] rand_b
                0x00, 0x00, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F,
            ]
        );

        assert_eq!(uuid.to_string(), "0193b82a-c130-70f0-8000-000f0f0f0f0f");
    }

    #[test]
    fn test_uuid_ver7_is_uuid_ver7_valid() {
        let uuid = UuidVer7::self_1();
        assert!(UuidVer7::is_uuid_ver7(&uuid.bytes));

        let uuid = UuidVer7::self_2();
        assert!(UuidVer7::is_uuid_ver7(&uuid.bytes));
    }

    #[test]
    fn test_uuid_ver7_is_uuid_ver7_invalid() {
        // バージョンビットが違う
        let mut bytes = UuidVer7::self_1().bytes;
        bytes[6] = 0x50; // 上位4bitが0b0101 (ver5)
        assert!(!UuidVer7::is_uuid_ver7(&bytes));

        // バリアントビットが違う
        let mut bytes = UuidVer7::self_1().bytes;
        bytes[8] = 0x00; // 上位2bitが0b00
        assert!(!UuidVer7::is_uuid_ver7(&bytes));
    }

    #[test]
    fn test_uuid_ver7_get_datetime() {
        use chrono::TimeZone;

        let uuid_1 = UuidVer7::self_1();
        let datetime_1 = chrono::Utc
            .with_ymd_and_hms(2024, 12, 12, 12, 12, 12)
            .unwrap();
        assert_eq!(uuid_1.get_datetime(), datetime_1);

        let uuid_2 = UuidVer7::self_2();
        let datetime_2 = chrono::Utc
            .with_ymd_and_hms(2024, 9, 20, 10, 24, 34)
            .unwrap();
        assert_eq!(uuid_2.get_datetime(), datetime_2);

        let uuid_3 = UuidVer7::self_3();
        let datetime_3 = chrono::Utc
            .with_ymd_and_hms(2022, 2, 22, 19, 22, 22)
            .unwrap();
        assert_eq!(uuid_3.get_datetime(), datetime_3);
    }

    #[test]
    fn test_uuid_ver7_deserialization_valid() {
        let uuid = UuidVer7::self_1();

        // lower case
        let json = "\"0193bac8-a560-7000-8000-000000000000\"";
        let result: Result<UuidVer7, _> = serde_json::from_str(json);
        assert_eq!(result.unwrap(), uuid);

        // upper case
        let json = "\"0193BAC8-A560-7000-8000-000000000000\"";
        let result: Result<UuidVer7, _> = serde_json::from_str(json);
        assert_eq!(result.unwrap(), uuid);
    }
    #[test]
    fn test_uuid_ver7_deserialization_invalid() {
        // バージョンビットが違う
        let mut bytes = UuidVer7::self_1().bytes;
        bytes[6] = 0x50;
        let json = format!("\"{}\"", UuidVer7::bytes_to_uuid_string(&bytes));
        let result: Result<UuidVer7, _> = serde_json::from_str(&json);
        assert!(result.is_err());

        // バリアントビットが違う
        let mut bytes = UuidVer7::self_1().bytes;
        bytes[8] = 0x00;
        let json = format!("\"{}\"", UuidVer7::bytes_to_uuid_string(&bytes));
        let result: Result<UuidVer7, _> = serde_json::from_str(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_uuid_ver7_display() {
        let uuid = UuidVer7::self_1();
        let expected = "0193bac8-a560-7000-8000-000000000000";
        assert_eq!(uuid.to_string(), expected);
        let uuid = UuidVer7::self_2();
        let expected = "01920ef6-46d0-70f0-8000-000f0f0f0f0f";
        assert_eq!(uuid.to_string(), expected);
    }
}
