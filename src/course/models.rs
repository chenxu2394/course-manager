use rocket::FromForm;

#[derive(Clone)]
pub struct Course {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub credits: u8,
}

#[derive(FromForm)]
pub struct NewCourseForm {
    pub title: String,
    pub description: String,
    pub credits: u8,
}
