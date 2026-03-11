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
                        <div class="actions">
                            <div class="actions">
                                <a class="action-button" href="/update_course/{}">Edit</a>
                                <form class="inline-form" action="/courses/{}/delete" method="post">
                                    <button class="action-button" type="submit">Delete</button>
                                </form>
                            </div>
                        </div>
                    </td>
                </tr>
        "#,
                c.id,
                encode_text(&c.title),
                c.credits,
                c.id,
                c.id
            )
        })
        .collect();

    format!(
        r#"
        <h1>List of Courses</h1>
        <a href="/add_course"><button>Add a course</button></a>
        <p></p>
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
                <h1>Course Detail</h1>
                <a href="/">Home</a>
                <a href="/update_course/{}">Edit</a>
                <p>{}: {} credits</p>
                <p>{}</p>
            </div>
            "#,
        c.id,
        encode_text(&c.title),
        c.credits,
        encode_text(&c.description)
    )
}

pub fn render_course_form(
    page_title: &str,
    form_action: &str,
    current_title: &str,
    current_description: &str,
    current_credits: u8,
) -> String {
    format!(
        r#"
        <h1>{}</h1>
        <a href="/">Home</a>
        <form action="/{}" method="post">
        <label for="title">Course Title</label>
        <br>
        <input type="text" id="title" name="title" value="{}" required>
        <br>
        <label for="credits">Credits</label>
        <br>
        <input type="number" id="credits" name="credits" value="{}" required>
        <br>
        <label for="description">Description</label>
        <br>
        <input type="text" id="description" name="description" value="{}" required>
        <div class="actions" id="submit">
            <input type="submit" value="Submit">
            <a href="/">Cancel</a>
        </div>
        </form>
    "#,
        page_title, form_action, current_title, current_credits, current_description
    )
}

pub fn render_page(body: String) -> String {
    format!(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <link rel="stylesheet" href="/static/style.css">
    </head>
    <body>
        {}
    </body>
    </html>
    "#,
        body
    )
}
