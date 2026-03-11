use rocket::{
    State,
    form::Form,
    get,
    http::Status,
    post,
    response::{Redirect, content::RawHtml},
    uri,
};

use super::models::*;
use super::states::AppState;
use super::utils::*;

use std::sync::atomic::Ordering;

#[post("/submit", data = "<course>")]
pub fn handle_form(
    course: Form<NewCourseForm>,
    state: &State<AppState>,
) -> Result<Redirect, Status> {
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
pub fn get_courses(state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let content: String = render_courses(&g);

    Ok(RawHtml(content))
}

#[post("/courses/<id>/delete")]
pub fn delete_course(id: u64, state: &State<AppState>) -> Result<Redirect, Status> {
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
pub fn get_course(id: u64, state: &State<AppState>) -> Result<Option<RawHtml<String>>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let course = g.iter().find(|c| c.id == id);

    if let Some(c) = course {
        let content = render_course(c);
        Ok(Some(RawHtml(content)))
    } else {
        Err(Status::NotFound)
    }
}

#[get("/")]
pub fn root(state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let course_table = render_courses(&g);

    Ok(RawHtml(render_root(course_table)))
}

#[get("/add_course")]
pub fn add_course() -> Result<RawHtml<String>, Status> {
    let content = render_add_a_course();
    Ok(RawHtml(content))
}
