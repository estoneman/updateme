// vim: tabstop=4 shiftwidth=4 softtabstop=4 expandtab autoindent

extern crate chrono;
use chrono::{DateTime, Local};
use clap::Parser;
use json::{JsonValue, JsonError};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const UPDATEME_HOME: &str = "/usr/local/opt/updateme";
const META_FILE: &str = "meta.json";
const MIN_ELAPSED: u64 = 43200;

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "COMMAND", help = "last, meta, update (default)")]
    command: Option<String>,
}

struct UpdatemeMetadata {
    file_path: PathBuf,
}

impl UpdatemeMetadata {
    fn read_file(&self) -> Result<String, io::Error> {
        let display = &self.file_path.display();
        let mut file = match File::open(&self.file_path) {
            Err(why) => panic!("could not open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Err(why) => panic!("could not read {}: {}", display, why),
            Ok(_) => (),
        };

        Ok(file_contents)
    }

    // fn parse_file(&self) -> Result<JsonValue, JsonError> {
    //     todo!();
    // }

    // fn get_value(&self, key: String) -> Option<JsonValue> {
    //     todo!();
    // }
}

// TODO: implement function that reads meta file and returns the JsonValue associated w/ key
//   -> like this: fn read(file_path, key)
// TODO: implement function that formats a given DateTime
//   -> like this: fn dt_format(str_to_format, datetime) variadic?

fn get_meta_path() -> PathBuf {
    let mut meta_path = PathBuf::new();
    meta_path.push(UPDATEME_HOME);
    meta_path.push(META_FILE);

    meta_path
}

fn meta() {
    let metadata = UpdatemeMetadata {
        // file_path: "/usr/local/opt/updateme/meta.json",
        file_path: get_meta_path(),
    };

    println!("{}", metadata.read_file().unwrap());
}

fn last() {
    let meta_path = get_meta_path();

    let display = meta_path.display();

    let mut file = match File::open(&meta_path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(_) => (),
    };

    let update_t_epoch = match json::parse(&file_contents) {
        Err(why) => panic!("could not parse {}: {}", display, why),
        Ok(file_parsed) => match json::parse(&file_parsed["last_update"].to_string()) {
            Err(why) => panic!("could not parse update time: {}", why),
            Ok(JsonValue::Number(update_t_epoch)) => update_t_epoch,
            _ => panic!("last_update field is not of type JsonValue::Number)"),
        },
    };

    let (_, mantissa, _) = update_t_epoch.as_parts();
    let update_t_sys: SystemTime = UNIX_EPOCH + Duration::from_secs(mantissa);
    let datetime: DateTime<Local> = update_t_sys.into();

    println!(
        "Last update was: {}",
        datetime.format("%a, %b %d %H:%M:%S(%Z) %Y")
    );
}

fn update() {
    let mut meta_path = PathBuf::new();
    meta_path.push(UPDATEME_HOME);
    meta_path.push(META_FILE);

    let display = meta_path.display();

    let mut file = match File::open(&meta_path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(_) => (),
    };

    let update_t_epoch = match json::parse(&file_contents) {
        Err(why) => panic!("could not parse {}: {}", display, why),
        Ok(file_parsed) => match json::parse(&file_parsed["last_update"].to_string()) {
            Err(why) => panic!("could not parse update time: {}", why),
            Ok(JsonValue::Number(update_t_epoch)) => update_t_epoch,
            _ => panic!("last_update field is not of type JsonValue::Number)"),
        },
    };

    let (_, mantissa, _) = update_t_epoch.as_parts();
    let update_t_sys: SystemTime = UNIX_EPOCH + Duration::from_secs(mantissa);
    let now = SystemTime::now();
    let since = match now.duration_since(update_t_sys) {
        Ok(since) => since.as_secs(),
        Err(why) => panic!("could not parse system time: {}", why),
    };

    if since < MIN_ELAPSED {
        let datetime: DateTime<Local> = update_t_sys.into();

        println!(
            "Last update was: {}\nplease wait at least {} hours",
            datetime.format("%a, %b %d %H:%M:%S(%Z) %Y"),
            MIN_ELAPSED / 3600,
        );

        return;
    }
}

// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
// https://rust-cli.github.io/book/tutorial/cli-args.html
fn main() {
    let args = Cli::parse();

    match args.command {
        cmd @ Some(_) => match cmd.as_deref() {
            Some("meta") => meta(),
            Some("last") => last(),
            _ => eprintln!("invalid command: {}", cmd.unwrap()),
        },
        None => update(),
    }
}
