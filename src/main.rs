extern crate circleci_checker;
extern crate rustc_serialize;

use std::error::Error;

fn main() {
    match print_projects() {
        Ok(()) => return ,
        Err(ref error) => println!("{}", error.description()),
    }
}

fn print_projects() -> Result<(), circleci_checker::Error>{
    let projects = circleci_checker::service::fetch_projects()?;

    println!("CircleCI");
    println!("---");

    for project in projects {
        println!("{}/{} | href={}", project.username, project.reponame, project.vcs_url);
        for (branch_name, branch) in project.branches {
            let url = format!("https://circleci.com/gh/{}/{}/tree/{}", project.username, project.reponame, branch_name);
            let recent_build = match branch.recent_builds {
                Some(ref builds) if builds.len() > 0 => &builds[0],
                _ => continue
            };

            let status = circleci_checker::Status::get(recent_build.outcome.as_str()).expect("ビルド状態が未定義です");
            println!("- {} {} | color={} href={}", status.symbol(), branch_name.replace("%2F", "/"), status.color(), url);
        }
        println!("---")
    }
    Ok(())
}
