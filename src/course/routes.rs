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

#[get("/")]
pub fn root(state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let course_table = render_courses(&g);

    Ok(RawHtml(render_page(course_table)))
}

#[get("/courses")]
pub fn get_courses(state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let content: String = render_page(render_courses(&g));

    Ok(RawHtml(content))
}

#[get("/courses/<id>")]
pub fn get_course(id: u64, state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let course = g.iter().find(|c| c.id == id);

    if let Some(c) = course {
        let content = render_course(c);
        Ok(RawHtml(render_page(content)))
    } else {
        Err(Status::NotFound)
    }
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

#[get("/add_course")]
pub fn add_course() -> Result<RawHtml<String>, Status> {
    let content = render_page(render_course_form("Add a course", "add_course", "", "", 0));
    Ok(RawHtml(content))
}

#[post("/add_course", data = "<course>")]
pub fn add_course_form(
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

#[get("/update_course/<id>")]
pub fn update_course(id: u64, state: &State<AppState>) -> Result<RawHtml<String>, Status> {
    let g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let course = g.iter().find(|c| c.id == id);

    if let Some(c) = course {
        let content = render_course_form(
            "Edit a course",
            format!("update_course/{}", id).as_str(),
            &c.title,
            &c.description,
            c.credits,
        );
        Ok(RawHtml(render_page(content)))
    } else {
        Err(Status::NotFound)
    }
}

#[post("/update_course/<id>", data = "<course>")]
pub fn update_course_form(
    id: u64,
    course: Form<NewCourseForm>,
    state: &State<AppState>,
) -> Result<Redirect, Status> {
    let mut g = state
        .courses
        .lock()
        .map_err(|_| Status::InternalServerError)?;

    let target = g.iter_mut().find(|c| c.id == id);

    if let Some(c) = target {
        c.title = course.title.clone();
        c.description = course.description.clone();
        c.credits = course.credits;
    } else {
        return Err(Status::NotFound);
    }

    Ok(Redirect::to(uri!("/")))
}
