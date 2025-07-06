/// color code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl std::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err(format!(
                "Color string must be 6 characters long, got: {}",
                s
            ));
        }
        let red = u8::from_str_radix(&s[0..2], 16)
            .map_err(|_| format!("Invalid red component in color: {}", s))?;
        let green = u8::from_str_radix(&s[2..4], 16)
            .map_err(|_| format!("Invalid green component in color: {}", s))?;
        let blue = u8::from_str_radix(&s[4..6], 16)
            .map_err(|_| format!("Invalid blue component in color: {}", s))?;

        Ok(Color { red, green, blue })
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_color_from_str() {
        // 正常系
        let c = Color::from_str("E43F3B").unwrap();
        assert_eq!(c.red, 0xE4);
        assert_eq!(c.green, 0x3F);
        assert_eq!(c.blue, 0x3B);
        let _c = Color::from_str("000000").unwrap();
        let _c = Color::from_str("FFFFFF").unwrap();

        assert!(Color::from_str("E43F3").is_err());
        assert!(Color::from_str("GGGGGG").is_err());
        assert!(Color::from_str("E43F3B7").is_err());
    }

    #[test]
    fn test_color_display() {
        let c = Color {
            red: 0xE4,
            green: 0x3F,
            blue: 0x3B,
        };
        assert_eq!(c.to_string(), "E43F3B");
    }

    #[test]
    fn test_color_serialization() {
        let c = Color {
            red: 0xE4,
            green: 0x3F,
            blue: 0x3B,
        };
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "\"E43F3B\"");
    }

    #[test]
    fn test_color_deserialization() {
        let json = "\"E43F3B\"";
        let c: Color = serde_json::from_str(json).unwrap();
        assert_eq!(
            c,
            Color {
                red: 0xE4,
                green: 0x3F,
                blue: 0x3B
            }
        );

        // 異常系
        let json = "\"E43F3\"";
        let result: Result<Color, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
