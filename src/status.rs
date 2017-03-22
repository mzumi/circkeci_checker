pub enum Status {
    Success,
    Failed,
    Cancel,
    Timedout,
    NotBuild,
}

impl Status {
    pub fn color(&self) -> &str {
        match *self {
            Status::Success => "green",
            Status::Failed => "red",
            Status::Cancel => "yellow",
            Status::Timedout => "gray",
            Status::NotBuild =>  "gray",
        }
    }

    pub fn symbol(&self) -> &str {
        match *self {
            Status::Success => "✔︎",
            Status::Failed => "✖︎",
            Status::Cancel => "⚠",
            Status::Timedout => " ⃠",
            Status::NotBuild =>  "•",
        }
    }

    pub fn get(status: &str) -> Option<Status> {
        match status {
            "success" => Some(Status::Success {}),
            "failed" => Some(Status::Failed {}),
            "cancel" => Some(Status::Cancel {}),
            "timedout" => Some(Status::Timedout {}),
            "no_tests" => Some(Status::NotBuild {}),
            _ => None
        }
    }
}
