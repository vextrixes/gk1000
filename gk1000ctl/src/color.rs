use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('#').unwrap_or(s);

        if s.len() != 6 {
            return Err(format!(
                "invalid color '{s}': expected 6 hex chars, e.g. 6700ff"
            ));
        }

        let bytes = hex::decode(s)
            .map_err(|_| format!("invalid color '{s}': must be valid hex"))?;

        Ok(Color {
            red: bytes[0],
            green: bytes[1],
            blue: bytes[2],
        })
    }
}