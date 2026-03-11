use super::models::Course;

use std::sync::{Mutex, atomic::AtomicU64};
pub struct AppState {
    pub courses: Mutex<Vec<Course>>,
    pub next_id: AtomicU64,
}
