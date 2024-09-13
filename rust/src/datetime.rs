use chrono::{Local, Utc};

// Function to get the formatted UTC date and time
pub fn get_formatted_time() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

// Function to get the local date and time
pub fn get_local_time() -> String {
    let local_now = Local::now();
    local_now.format("%Y-%m-%d %H:%M:%S").to_string()
}

// General function to display both
pub fn get_dt() -> String {
    let formatted_time = get_formatted_time();
    let local_time = get_local_time();

    format!("UTC: {}\nLocal: {}", formatted_time, local_time)
}
