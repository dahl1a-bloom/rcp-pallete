use thiserror::Error;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Error, Debug)]
pub enum ColorParseError {
    #[error("Недійсна довжина Hex-коду: {0} символів. Очікується 3 або 6.")]
    InvalidLength(usize),

    #[error("Недійсний шістнадцятковий компонент '{0}'. Деталі: {1}")]
    ComponentParseError(String, #[source] ParseIntError),

    #[error("Колір має починатися з символу '#'. Непідтримуваний формат.")]
    MissingHashPrefix,

    #[error("Непідтримуваний формат кольору.")]
    UnsupportedFormat,

    #[error("Некоректний формат rgb(). Очікується: rgb(r, g, b)")]
    RgbInvalidFormat,

    #[error("Недійсний числовий компонент rgb(): '{0}'")]
    RgbComponentParseError(String),

    #[error("Компонент rgb() поза діапазоном 0..=255: {0}")]
    RgbComponentOutOfRange(i32),
}

/// Перетворює рядкове представлення CSS-кольору у внутрішню структуру `Color`.

///
/// ### Граматичні Правила (Документація)
///
/// ### 1. Hex6: `#RRGGBB`
/// Опис. Парсить повний 6-значний шістнадцятковий код.
/// Наприклад: `#1A2B3C`.

///
/// ### 2. Hex3: `#RGB`
/// Опис. Парсить скорочений 3-значний код, виконуючи декомпресію (дублювання символів).
/// Наприклад: `#FA0` -> `#FFAA00`.

///
/// ### 3. InvalidLength
/// Опис. Обробляє Hex-коди, що мають довжину, відмінну від 3 або 6 символів.
/// Наприклад, `#1234`.

///
/// ### 4. InvalidChar
/// Опис. Обробляє наявність не-шістнадцяткових символів.
/// Наприклад, `#1A2B3G`.

///
/// ### 5. RGB: `rgb(R, G, B)`
/// Опис. Парсить десяткові компоненти у діапазоні 0..=255. Пробіли дозволені.
/// Наприклад, rgb(255, 170, 0).

///
/// ### Приклади
///
/// ```
/// use rcp_palette::{parse_color, Color};
/// // Правило 1: Hex6
/// assert_eq!(parse_color("#1A2B3C").unwrap(), Color { r: 26, g: 43, b: 60 });
/// // Правило 2: Hex3
/// assert_eq!(parse_color("#FA0").unwrap(), Color { r: 255, g: 170, b: 0 });
/// // Негативний приклад (Правило 3)
/// assert!(parse_color("#1234").is_err());
/// // Негативний приклад (Правило 4)
/// assert!(parse_color("#1A2B3G").is_err());
/// // Правило 5: rgb()
/// assert_eq!(parse_color("rgb(255, 170, 0)").unwrap(), Color { r: 255, g: 170, b: 0 });
/// ```
pub fn parse_color(input: &str) -> Result<Color, ColorParseError> {
    if input.starts_with('#') {
        let hex_str = &input[1..];

        return match hex_str.len() {
            3 => {
                let r_str = format!("{0}{0}", hex_str.chars().nth(0).unwrap());
                let g_str = format!("{0}{0}", hex_str.chars().nth(1).unwrap());
                let b_str = format!("{0}{0}", hex_str.chars().nth(2).unwrap());
                let r = parse_component(&r_str)?;
                let g = parse_component(&g_str)?;
                let b = parse_component(&b_str)?;
                Ok(Color { r, g, b })
            }
            6 => {
                let r = parse_component(&hex_str[0..2])?;
                let g = parse_component(&hex_str[2..4])?;
                let b = parse_component(&hex_str[4..6])?;
                Ok(Color { r, g, b })
            }
            _ => Err(ColorParseError::InvalidLength(hex_str.len())),
        };
    } else if input.trim_start().to_ascii_lowercase().starts_with("rgb(") {
        parse_rgb(input)
    } else {
        Err(ColorParseError::MissingHashPrefix)
    }
}


fn parse_component(component: &str) -> Result<u8, ColorParseError> {
    u8::from_str_radix(component, 16)
        .map_err(|e| ColorParseError::ComponentParseError(component.to_string(), e)) 
}


fn parse_rgb(input: &str) -> Result<Color, ColorParseError> {
    let open = input.find('(').ok_or(ColorParseError::RgbInvalidFormat)?;
    let close = input.rfind(')').ok_or(ColorParseError::RgbInvalidFormat)?;

    if close <= open + 1 { return Err(ColorParseError::RgbInvalidFormat); }

    let inner = &input[open + 1..close];
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

    if parts.len() != 3 { return Err(ColorParseError::RgbInvalidFormat); }

    let mut nums = [0u8; 3];

    for (i, p) in parts.iter().enumerate() {
        let parsed: i32 = p.parse().map_err(|_| ColorParseError::RgbComponentParseError((*p).to_string()))?;
        if parsed < 0 || parsed > 255 {
            return Err(ColorParseError::RgbComponentOutOfRange(parsed));
        }
        nums[i] = parsed as u8;
    }

    Ok(Color { r: nums[0], g: nums[1], b: nums[2] })
}