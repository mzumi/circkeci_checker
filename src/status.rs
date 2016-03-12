struct Success {}
struct Failed {}
struct Cancel {}
struct Timedout {}
struct NotBuild {}

pub trait Status {
    fn color(&self) -> &str;
    fn symbol(&self) -> &str;
}

impl Status for Success {
    fn color(&self) -> &str { "green" }
    fn symbol(&self) -> &str { "✔︎" }
}

impl Status for Failed {
    fn color(&self) -> &str { "red" }
    fn symbol(&self) -> &str { "✖︎" }
}

impl Status for Cancel {
    fn color(&self) -> &str { "yellow" }
    fn symbol(&self) -> &str { "⚠" }
}

impl Status for Timedout {
    fn color(&self) -> &str { "gray" }
    fn symbol(&self) -> &str { " ⃠" }
}

impl Status for NotBuild {
    fn color(&self) -> &str { "gray" }
    fn symbol(&self) -> &str { "•" }
}

pub fn get(status: &str) -> Box<Status> {
    match status {
        "success" => Box::new(Success {}),
        "failed" => Box::new(Failed {}),
        "cancel" => Box::new(Cancel {}),
        "timedout" => Box::new(Timedout {}),
        _ => Box::new(NotBuild {})
    }
}
