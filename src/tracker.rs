use chrono::Local;
use std::thread;
use std::time::Duration;

use crate::storage;

pub fn run(poll_sec: u64) {
    println!("Tracking started... Press CTRL+C to stop.");

    loop {
        let now = Local::now().timestamp();

        // Dummy activity to make it portable
        let app = "test_app".to_string();
        let activity = "testing".to_string();

        storage::end_last_session(now);
        storage::start_session(&app, &activity, now);

        thread::sleep(Duration::from_secs(poll_sec));
    }
}
