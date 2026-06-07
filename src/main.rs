use clap::Parser;
use serde::Deserialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "nine-stock")]
struct Args {
    #[arg(long)]
    session: String,
    #[arg(long)]
    code: String,
    #[arg(long)]
    ktype: String,
    #[arg(long)]
    data: String,
    #[arg(long)]
    debug: bool,
}

#[derive(Deserialize)]
struct Config {
    #[serde(default = "default_true")]
    opencb: bool,
    #[serde(default = "default_channel")]
    opencb_channel_id: String,
}

fn default_true() -> bool {
    true
}

fn default_channel() -> String {
    "1513197866598928494".to_string()
}

fn config_path() -> PathBuf {
    let mut p = dirs::home_dir().expect("cannot determine home dir");
    p.push(".config/nine-stock/config.toml");
    p
}

fn load_config() -> Config {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or(Config {
            opencb: default_true(),
            opencb_channel_id: default_channel(),
        })
    } else {
        let dir = path.parent().unwrap();
        let _ = fs::create_dir_all(dir);
        let default = format!(
            "opencb = true\nopencb-channel-id = \"{}\"\n",
            default_channel()
        );
        let _ = fs::write(&path, default);
        Config {
            opencb: default_true(),
            opencb_channel_id: default_channel(),
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.debug {
        eprintln!(
            "nine-poe --session \"{}\" --model \"NS2026-{}\" --prompt \"{}\"",
            args.session, args.ktype, args.data
        );
    }

    let output = Command::new("nine-poe")
        .arg("--session")
        .arg(&args.session)
        .arg("--model")
        .arg(format!("NS2026-{}", args.ktype))
        .arg("--prompt")
        .arg(&args.data)
        .output()
        .expect("failed to execute nine-poe");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let response = if !stdout.trim().is_empty() {
        stdout
    } else if !stderr.trim().is_empty() {
        stderr
    } else {
        return;
    };

    let mut full_response = response;
    full_response.push_str("\n#LINEBREAK#\n");

    let home = dirs::home_dir().expect("cannot determine home dir");
    let file_dir = home.join(format!(
        ".opens/nine-stock/analysis/{}/{}/",
        args.code, args.session
    ));
    let _ = fs::create_dir_all(&file_dir);
    let file_path = file_dir.join(format!("{}.txt", args.ktype));
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .expect("failed to open file for writing");
    file.write_all(full_response.as_bytes())
        .expect("failed to write to file");

    print!("{full_response}");

    let config = load_config();
    if !config.opencb || config.opencb_channel_id.is_empty() {
        return;
    }

    if args.debug {
        eprintln!(
            "opencb send \"{}\" --rc \"{}\"",
            full_response.trim(),
            config.opencb_channel_id
        );
    }

    let _ = Command::new("opencb")
        .arg("send")
        .arg(&full_response)
        .arg("--rc")
        .arg(&config.opencb_channel_id)
        .output();
}
