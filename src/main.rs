extern crate circleci_checker;
extern crate rustc_serialize;

fn main() {
    println!("CircleCI");
    println!("---");

    let projects = circleci_checker::service::fetch_projects();

    for project in projects.as_array().unwrap() {
        let p = project.as_object().unwrap();

        let username = convert_string(&p.get("username"));
        let reponame = convert_string(&p.get("reponame"));
        println!("{}/{} | href={}", username, reponame, convert_string(&p.get("vcs_url")));

        for (branch_name, branch_info) in p.get("branches").unwrap().as_object().unwrap() {
            print_branch_info(&username, &reponame, branch_name, branch_info);
        }
        println!("---")
    }
}

fn print_branch_info(username: &String, reponame: &String, branch_name: &String, branch_info: &rustc_serialize::json::Json) {
    let info = branch_info.as_object().unwrap();

    let outcome = match info.get("recent_builds") {
        Some(recent_builds) => recent_builds[0].as_object().unwrap().get("outcome").unwrap().as_string().unwrap(),
        None => ""
    };

    let status = circleci_checker::status::get(outcome);
    let url = format!("https://circleci.com/gh/{}/{}/tree/{}", username, reponame, branch_name);
    println!("- {} {} | color={} href={}", status.symbol(), branch_name.replace("%2F", "/"), status.color(), url);
}

fn convert_string(json: &Option<&rustc_serialize::json::Json>) -> String {
    let string = match json {
        &Some(string) => string.as_string(),
        &None => return "".to_string()
    };

    match string {
        Some(s) => s.to_string(),
        None => "".to_string()
    }
}
