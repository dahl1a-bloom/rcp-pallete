use anyhow::Result;
use rcp_palette::parse_color;

#[test]
fn hex6_parsing_is_ok() -> Result<()> {
    let c = parse_color("#1A2B3C")?;
    assert_eq!((c.r, c.g, c.b), (26, 43, 60));
    Ok(())
}

#[test]
fn hex3_parsing_is_ok() -> Result<()> {
    let c = parse_color("#FA0")?;
    assert_eq!((c.r, c.g, c.b), (255, 170, 0));
    Ok(())
}

#[test]
fn invalid_length_is_error() -> Result<()> {
    let err = parse_color("#1234").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Недійсна довжина") || msg.contains("Invalid"));
    Ok(())
}

#[test]
fn invalid_char_is_error() -> Result<()> {
    let err = parse_color("#1A2B3G").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Недійсний") || msg.contains("invalid"));
    Ok(())
}

#[test]
fn missing_hash_prefix_is_error() -> Result<()> {
    let err = parse_color("1A2B3C").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("починатися") || msg.contains("begin with"));
    Ok(())
}

#[test]
fn rgb_parsing_is_ok() -> Result<()> {
    let c = parse_color("rgb(255, 170, 0)")?;
    assert_eq!((c.r, c.g, c.b), (255, 170, 0));
    Ok(())
}

#[test]
fn rgb_with_spaces_parsing_is_ok() -> Result<()> {
    let c = parse_color(" rgb( 26 , 43 , 60 ) ")?;
    assert_eq!((c.r, c.g, c.b), (26, 43, 60));
    Ok(())
}

#[test]
fn rgb_invalid_format_is_error() -> Result<()> {
    let err = parse_color("rgb(255, 170)").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("rgb") || msg.contains("format"));
    Ok(())
}

#[test]
fn rgb_out_of_range_is_error() -> Result<()> {
    let err = parse_color("rgb(256, 0, 0)").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("0..=255") || msg.contains("range"));
    Ok(())
}

#[test]
fn rgb_non_numeric_component_is_error() -> Result<()> {
    let err = parse_color("rgb(aa, 0, 0)").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("числовий") || msg.contains("numeric"));
    Ok(())
}
