use rocket::{
    State, build,
    form::{Form, FromForm},
    get,
    http::Status,
    launch, post,
    response::{Redirect, content::RawHtml},
    routes, uri,
};

use std::sync::{
    Mutex,
    atomic::{AtomicU64, Ordering},
};

#[derive(Clone)]
struct Course {
    id: u64,
    title: String,
    description: String,
    credits: u8,
}

struct AppState {
    courses: Mutex<Vec<Course>>,
    next_id: AtomicU64,
}

#[derive(FromForm)]
struct NewCourseForm {
    title: String,
    description: String,
    credits: u8,
}

fn render_courses(c: &[Course]) -> String {
    let content: String = c
        .iter()
        .map(|c| {
            format!(
                r#"
                <tr>
                    <td><a href="/courses/{}">{}</a></td>
                    <td>{}</td>
                    <td>
                        <form action="/courses/{}/delete" method="post">
                        <input type="submit" value="Delete">
                        </form>
                    </td>
                </tr>
        "#,
                c.id, c.title, c.credits, c.id
            )
        })
        .collect();

    format!(
        r#"
        <div>
            <table>
            <tr>
                <th>Title</th>
                <th>Credits</th>
                <th>Actions</th>
{}
            </table>
        </div>
    "#,
        content
    )
}

#[post("/submit", data = "<course>")]
fn handle_form(course: Form<NewCourseForm>, state: &State<AppState>) -> Result<Redirect, Status> {
    let new_id = state.next_id.fetch_add(1, Ordering::Relaxed);

    let new_course = Course {
        id: new_id,
        title: course.title.clone(),
        description: course.description.clone(),
        credits: course.credits,
    };

    let mut g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    (*g).push(new_course);

    Ok(Redirect::to(uri!("/")))
}

#[get("/courses")]
fn get_courses(state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let content: String = render_courses(&g);

    Ok(RawHtml(content))
}

#[post("/courses/<id>/delete")]
fn delete_course(id: u64, state: &State<AppState>) -> Result<Redirect, Status> {
    let mut g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let index = g.iter().position(|c| c.id == id);

    if let Some(i) = index {
        let _ = (*g).remove(i);
        Ok(Redirect::to(uri!("/")))
    } else {
        Err(Status::NotFound)
    }
}

#[get("/courses/<id>")]
fn get_course(id: u64, state: &State<AppState>) -> Result<Option<RawHtml<String>>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let course = g.iter().find(|c| c.id == id);

    if let Some(c) = course {
        let content = format!(
            r#"
            <div>
            <p>{}</p>
            <p>{} credits</p>
            <p>{}</p>
            </div>
            "#,
            c.title, c.credits, c.description
        );
        Ok(Some(RawHtml(content)))
    } else {
        Err(Status::NotFound)
    }
}

#[get("/")]
fn root(state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let content = r#"
    <h1>Add Course</h1>
    <form action="/submit" method="post">
    <label for="title">Course Title</label>
    <br>
    <input type="text" id="title" name="title" required>
    <br>
    <label for="credits">Credits</label>
    <br>
    <input type="number" id="credits" name="credits" required>
    <br>
    <label for="description">Description</label>
    <br>
    <input type="text" id="description" name="description" required>
    <br>
    <input type="submit" value="Submit">
    </form>
    "#;

    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let course_table = render_courses(&g);

    Ok(RawHtml(format!(
        r#"
    <!DOCTYPE html>
    <html>
    <style>
        table, th, td {{
            border:1px solid black;
        }}
    </style>
    <body>
        {}
        <p></p>
        {}
    </body>
    </html>
    "#,
        content, course_table
    )))
}

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

    build().manage(init).mount(
        "/",
        routes![root, handle_form, get_courses, get_course, delete_course],
    )
}
