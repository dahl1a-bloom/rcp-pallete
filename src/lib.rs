use std::num::ParseIntError;
use thiserror::Error;

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
/// ## Граматичні правила (для docs.rs)
///
/// Формальна граматика:
///
/// ```text
/// Color        := Hex | Rgb
/// Hex          := "#" (Hex6 | Hex3)
/// Hex6         := H H H H H H                 ; шість шістнадцяткових символів
/// Hex3         := H H H                       ; три шістнадцяткових символи (дублюються)
/// Rgb          := "rgb" "(" Int "," Int "," Int ")"
/// H            := [0-9A-Fa-f]
/// Int          := Digit+                      ; десяткові цілі
/// Digit        := [0-9]
/// ```
///
/// Семантичні обмеження:
///
/// 1. **Hex6**: `#RRGGBB`
///    - Рівно 6 шістнадцяткових символів після `#`.
///    - Кожна пара (`RR`, `GG`, `BB`) парситься як `u8` у діапазоні 0..=255.
///
/// 2. **Hex3**: `#RGB`
///    - Рівно 3 шістнадцяткових символи після `#`.
///    - Кожен символ дублюється (`F` → `FF`), утворюючи валідний Hex6.
///
/// 3. **InvalidLength**:
///    - Будь-який рядок, що починається з `#`, але довжина частини без `#` **≠ 3** і **≠ 6**.
///
/// 4. **InvalidChar / ComponentParseError**:
///    - Будь-який Hex-код, що містить символи поза діапазоном `[0-9A-Fa-f]`.
///
/// 5. **RGB: `rgb(R, G, B)`**:
///    - Пробіли навколо чисел та ком дозволені.
///    - Компоненти `R`, `G`, `B` спочатку парсяться як `i32`.
///    - Якщо компонент не є числом → `RgbComponentParseError`.
///    - Якщо число поза діапазоні 0..=255 → `RgbComponentOutOfRange`.
///    - Некоректна кількість компонентів або відсутні дужки → `RgbInvalidFormat`.
///
/// 6. **MissingHashPrefix**:
///    - Викидається, коли рядок не починається з `#` і не відповідає формі `rgb(...)`.
/// ## Приклади
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
/// // Правило 5: rgb() — валідний приклад
/// assert_eq!(parse_color("rgb(255, 170, 0)").unwrap(), Color { r: 255, g: 170, b: 0 });
/// // Правило 5: rgb() — з пробілами
/// assert_eq!(parse_color(" rgb( 26 , 43 , 60 ) ").unwrap(), Color { r: 26, g: 43, b: 60 });
/// // Правило 5: rgb() — некоректний формат (замало компонентів)
/// assert!(parse_color("rgb(255, 170)").is_err());
/// // Правило 5: rgb() — вихід за діапазон 0..=255
/// assert!(parse_color("rgb(256, 0, 0)").is_err());
/// // Правило 5: rgb() — нечисловий компонент
/// assert!(parse_color("rgb(aa, 0, 0)").is_err());
/// // Негативний приклад (Правило 6: MissingHashPrefix)
/// assert!(parse_color("1A2B3C").is_err());
/// ```
pub fn parse_color(input: &str) -> Result<Color, ColorParseError> {
    if let Some(hex_str) = input.strip_prefix('#') {
        match hex_str.len() {
            3 => {
                let mut chars = hex_str.chars();
                let r_ch = chars.next().unwrap();
                let g_ch = chars.next().unwrap();
                let b_ch = chars.next().unwrap();

                let r_str = format!("{0}{0}", r_ch);
                let g_str = format!("{0}{0}", g_ch);
                let b_str = format!("{0}{0}", b_ch);
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
        }
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

    if close <= open + 1 {
        Err(ColorParseError::RgbInvalidFormat)
    } else {
        let inner = &input[open + 1..close];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

        if parts.len() != 3 {
            Err(ColorParseError::RgbInvalidFormat)
        } else {
            let mut nums = [0u8; 3];

            for (i, p) in parts.iter().enumerate() {
                let parsed: i32 = p
                    .parse()
                    .map_err(|_| ColorParseError::RgbComponentParseError((*p).to_string()))?;
                if !(0..=255).contains(&parsed) {
                    return Err(ColorParseError::RgbComponentOutOfRange(parsed));
                }
                nums[i] = parsed as u8;
            }

            Ok(Color {
                r: nums[0],
                g: nums[1],
                b: nums[2],
            })
        }
    }
}
