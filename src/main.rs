use rocket::{build, fs::FileServer, launch, routes};

use std::sync::{Mutex, atomic::AtomicU64};

mod course;
use course::models::*;
use course::routes::*;
use course::states::AppState;

#[launch]
fn rocket() -> _ {
    let init = AppState {
        courses: Mutex::new(vec![
            Course {
                id: 1,
                title: "Rust Programming".to_owned(),
                description: "Learn Rust at your own pace".to_owned(),
                credits: 4,
            },
            Course {
                id: 2,
                title: "Linux Basics".to_owned(),
                description: "The foundation every computer science student should have".to_owned(),
                credits: 3,
            },
        ]),
        next_id: AtomicU64::new(3),
    };

    build()
        .manage(init)
        .mount(
            "/",
            routes![
                root,
                add_course_form,
                get_courses,
                get_course,
                delete_course,
                add_course,
                update_course,
                update_course_form
            ],
        )
        .mount("/static", FileServer::from("static"))
}
