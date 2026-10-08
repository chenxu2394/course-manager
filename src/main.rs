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

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::{
        http::{ContentType, Status},
        local::blocking::Client,
    };

    fn client() -> Client {
        Client::tracked(rocket()).expect("valid Rocket instance")
    }

    #[test]
    fn root_lists_seed_courses() {
        let client = client();

        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response
            .into_string()
            .expect("response should contain HTML");

        assert!(body.contains("Rust Programming"));
        assert!(body.contains("Linux Basics"));
    }

    #[test]
    fn nonexistent_course_returns_not_found() {
        let client = client();

        let response = client.get("/courses/999").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn can_add_course() {
        let client = client();

        let response = client
            .post("/add_course")
            .header(ContentType::Form)
            .body("title=Distributed+Systems&description=Learn+distributed+systems&credits=5")
            .dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        let response = client.get("/courses/3").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response
            .into_string()
            .expect("response should contain HTML");

        assert!(body.contains("Distributed Systems"));
        assert!(body.contains("5 credits"));
        assert!(body.contains("Learn distributed systems"));
    }

    #[test]
    fn can_update_course() {
        let client = client();

        let response = client
            .post("/update_course/1")
            .header(ContentType::Form)
            .body("title=Advanced+Rust&description=Ownership+and+concurrency&credits=6")
            .dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        let response = client.get("/courses/1").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response
            .into_string()
            .expect("response should contain HTML");

        assert!(body.contains("Advanced Rust"));
        assert!(body.contains("6 credits"));
        assert!(body.contains("Ownership and concurrency"));
        assert!(!body.contains("Rust Programming"));
    }

    #[test]
    fn can_delete_course() {
        let client = client();

        let response = client.post("/courses/1/delete").dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        let response = client.get("/courses/1").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn invalid_credits_are_rejected() {
        let client = client();

        let response = client
            .post("/add_course")
            .header(ContentType::Form)
            .body("title=Invalid+Course&description=Credits+do+not+fit+in+u8&credits=300")
            .dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);

        let response = client.get("/courses/3").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn course_content_is_html_escaped() {
        let client = client();

        let response = client
            .post("/add_course")
            .header(ContentType::Form)
            .body(
                "title=%3Cscript%3Ealert%281%29%3C%2Fscript%3E&description=%3Cb%3Eunsafe%3C%2Fb%3E&credits=5",
            )
            .dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        let response = client.get("/courses/3").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let body = response
            .into_string()
            .expect("response should contain HTML");

        assert!(body.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(body.contains("&lt;b&gt;unsafe&lt;/b&gt;"));
        assert!(!body.contains("<script>"));
        assert!(!body.contains("<b>unsafe</b>"));
    }
}
