use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rcp_palette::{parse_color, ColorParseError};
use std::fs;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "CLI для парсингу CSS-кольорів: Hex (#RRGGBB, #RGB), rgb(R, G, B), hsl(H, S%, L%), іменованих.", 
    long_about = None
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Parse { color_str: String },
    File { path: String },
    Author,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { color_str } => {
            println!("--- Парсинг кольору: {} ---", color_str);

            let color = parse_color(color_str)
                .map_err(|e| anyhow::anyhow!(e))
                .context("Не вдалося виконати парсинг кольору!")?;

            println!("Парсинг кольору пройшов успішно!");
            println!("   > Введений колір: {}", color_str);
            println!("   > Color: r: {}, g: {}, b: {}", color.r, color.g, color.b);
        }
        Commands::File { path } => {
            println!("--- Читання та парсинг кольорів з файлу: {} ---", path);

            let content = fs::read_to_string(path)
                .with_context(|| format!("Не вдалося прочитати файл за шляхом: {}", path))?;
            for (i, line) in content.lines().enumerate() {
                let trimmed_line = line.trim();
                if trimmed_line.is_empty() {
                    continue;
                }

                match parse_color(trimmed_line) {
                    Ok(color) => println!(
                        "Рядок {}: ✅ {} -> RGB: r:{}, g:{}, b:{}",
                        i + 1,
                        trimmed_line,
                        color.r,
                        color.g,
                        color.b
                    ),
                    Err(e) => match e {
                        ColorParseError::MissingHashPrefix => eprintln!(
                            "Рядок {}: ❌ {} -> Помилка: Колір має починатися з '#'",
                            i + 1,
                            trimmed_line
                        ),
                        ColorParseError::InvalidLength(_) => eprintln!(
                            "Рядок {}: ❌ {} -> Помилка: Недійсна довжина Hex-коду",
                            i + 1,
                            trimmed_line
                        ),
                        _ => eprintln!("Рядок {}: ❌ {} -> Помилка: {}", i + 1, trimmed_line, e),
                    },
                }
            }
            println!("--- Парсинг файлу завершено ---");
        }
        Commands::Author => {
            println!("--- 🎨 rcp-palette (CSS Color Parser) ---");
            println!("Автор: {}", env!("CARGO_PKG_AUTHORS"));
            println!("Версія: {}", env!("CARGO_PKG_VERSION"));
            println!("Ліцензія: {}", env!("CARGO_PKG_LICENSE"));
            println!("Опис: {}", env!("CARGO_PKG_DESCRIPTION"));
            println!("Репозиторій: {}", env!("CARGO_PKG_REPOSITORY"));
        }
    }

    Ok(())
}
