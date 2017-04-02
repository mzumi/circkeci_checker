use std::collections::HashMap;

#[derive(Debug, RustcDecodable)]
pub struct Project {
    pub username: String,
    pub reponame: String,
    pub vcs_url: String,
    pub branches: HashMap<String, Branch>,
}

#[derive(Debug, RustcDecodable)]
pub struct Branch {
    // pub pusher_logins: Vec<String>,
    // pub last_non_success: BuildStatus,
    // pub last_success: BuildStatus,
    pub recent_builds: Option<Vec<BuildStatus>>,
}

#[derive(Debug, RustcDecodable)]
pub struct BuildStatus {
    pub pushed_at: String,
    pub vcs_revision: String,
    pub build_num: i32,
    pub outcome: String,
}
