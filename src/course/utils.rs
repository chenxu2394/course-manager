use super::models::Course;
use html_escape::encode_text;

pub fn render_courses(c: &[Course]) -> String {
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
                c.id,
                encode_text(&c.title),
                c.credits,
                c.id
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

pub fn render_course(c: &Course) -> String {
    format!(
        r#"
            <div>
            <p>{}</p>
            <p>{} credits</p>
            <p>{}</p>
            </div>
            "#,
        encode_text(&c.title),
        c.credits,
        encode_text(&c.description)
    )
}

pub fn render_add_a_course() -> String {
    (r#"
    <h1>Add a course</h1>
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
    <p></p>
    <input type="submit" value="Submit">
    </form>
    "#)
    .to_owned()
}

pub fn render_root(course_table: String) -> String {
    format!(
        r#"
    <!DOCTYPE html>
    <html>
    <style>
        table, th, td {{
            border:1px solid black;
        }}
    </style>
    <body>
        <h1>List of Courses</h1>
        <a href="/add_course"><button>Add a course</button></a>
        <p></p>
        {}
    </body>
    </html>
    "#,
        course_table
    )
}
