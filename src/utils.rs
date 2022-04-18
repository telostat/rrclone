use chrono::Utc;

pub fn log(msg: String) -> () {
    eprintln!("RRCLONE>> [{}] {}", isonow(), msg);
}

pub fn isonow() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
