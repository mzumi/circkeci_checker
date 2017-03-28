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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success_status() {
        let status =  Status::get("success").unwrap();
        assert_eq!("green", status.color());
        assert_eq!("✔︎", status.symbol());
    }

    #[test]
    fn failed_status() {
        let status = Status::get("failed").unwrap();
        assert_eq!("red", status.color());
        assert_eq!("✖︎", status.symbol());
    }

    #[test]
    fn cancel_status() {
        let status = Status::get("cancel").unwrap();
        assert_eq!("yellow", status.color());
        assert_eq!("⚠", status.symbol());
    }

    #[test]
    fn timeout_status() {
        let status = Status::get("timedout").unwrap();
        assert_eq!("gray", status.color());
        assert_eq!(" ⃠", status.symbol());
    }

    #[test]
    fn not_build_status() {
        let status = Status::get("no_tests").unwrap();
        assert_eq!("gray", status.color());
        assert_eq!("•", status.symbol());
    }
}
