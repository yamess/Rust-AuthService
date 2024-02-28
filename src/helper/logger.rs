use std::path::Path;

use chrono::Utc;
use fern::colors::{Color, ColoredLevelConfig};

async fn get_log_file_path(folder: &str) -> String {
    let folder = match Path::new::<str>(folder).is_dir() {
        true => folder,
        false => "/tmp",
    };
    let file_path = format!(
        "{folder}/{file_prefix}.log",
        folder = folder,
        file_prefix = Utc::now().format("%Y-%m-%d")
    );
    file_path
}

pub async fn initialize_logger(log_folder: &str) -> Result<(), fern::InitError> {
    let file_path = get_log_file_path(log_folder).await;
    log::info!("Testing log file name: {}", &file_path);

    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightBlack)
        .debug(Color::Blue)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::BrightRed);

    let file_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{datetime} - {level} - Thread {thread} - {target}: {line}]: \
                {message}",
                datetime = Utc::now().format("%Y-%m-%d %H:%M:%S"),
                level = record.level(),
                thread = std::thread::current()
                    .name()
                    .unwrap_or("unnamed")
                    .to_uppercase(),
                target = record.target(),
                line = record.line().unwrap(),
                message = message
            ))
        })
        .chain(fern::log_file(file_path).unwrap());

    let console_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{datetime} - {level} - Thread {thread} - {target}: {line}]: \
                {message}",
                datetime = Utc::now().format("%Y-%m-%d %H:%M:%S"),
                level = colors.color(record.level()),
                thread = std::thread::current()
                    .name()
                    .unwrap_or("unnamed")
                    .to_uppercase(),
                target = record.target(),
                line = record.line().unwrap(),
                message = message
            ))
        })
        .chain(std::io::stdout());

    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(console_dispatcher)
        .chain(file_dispatcher)
        .apply()?;
    Ok(())
}
