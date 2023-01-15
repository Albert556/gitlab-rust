use std::io::{Result, Write};
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Stdio,
};

use gitlab::{
    api::{self, Query},
    Gitlab, Group, Project,
};

fn main() {
    let local_path = PathBuf::from("/Users/albertguo/Documents/wiqun/wiqun_code");

    // let token = "glpat-rT4-TEfsiQYW3GWXyepa";
    // let client = Gitlab::new("gitlab.com", token).unwrap();
    let token = "ztZH-Z4FWmW6seNwtzGk";
    let client = Gitlab::new_insecure("gitlab.dev.wiqun.com", token).unwrap();

    // let groups_name = vec!["tl", "jd"];
}

/// .
///
/// # Panics
///
/// Panics if .
fn get_groups(client: &Gitlab) -> Vec<Group> {
    let groups_endpoint = api::groups::Groups::builder()
        .top_level_only(false)
        // 注释说是用 full path 但是结果并不是，可能是用 path 排序
        // .order_by(api::groups::GroupOrderBy::Path)
        .build()
        .unwrap();
    let mut groups: Vec<Group> = api::paged(groups_endpoint, api::Pagination::All)
        .query(client)
        .unwrap();
    groups.sort_by(|a, b| a.full_path.to_lowercase().cmp(&b.full_path.to_lowercase()));

    return groups;
}

fn get_subgroups_under_group(client: &Gitlab, group: &Group, is_recursion: bool) -> Vec<Group> {
    let subgroup_endpoint = api::groups::subgroups::GroupSubgroups::builder()
        .group(group.id.value())
        .build()
        .unwrap();
    let subgroups: Vec<Group> = subgroup_endpoint.query(client).unwrap();
    if !is_recursion {
        return subgroups;
    }
    // let mut result: Vec<>

    return subgroups;
}

fn get_projects_under_group(client: &Gitlab, group: &Group) -> Vec<Project> {
    let group_projects_endpoint = api::groups::projects::GroupProjects::builder()
        .group(group.id.value())
        .build()
        .unwrap();
    let group_projects: Vec<Project> = group_projects_endpoint.query(client).unwrap();

    return group_projects;
}

fn get_projects(client: &Gitlab) -> Vec<Project> {
    let projects_endpoint = api::projects::Projects::builder().build().unwrap();
    let mut projects: Vec<Project> = projects_endpoint.query(client).unwrap();

    projects.sort_by(|a, b| {
        a.path_with_namespace
            .to_lowercase()
            .cmp(&b.path_with_namespace.to_lowercase())
    });

    return projects;
}

fn clone_repository(project: Project, local_path: &PathBuf) {
    let mut repo_clone_success: Vec<Project> = vec![];
    let mut repo_clone_failed: Vec<Project> = vec![];

    let result = git_clone_command(
        project.ssh_url_to_repo.as_str(),
        local_path.as_os_str().to_str().unwrap(),
    );
    match result {
        true => repo_clone_success.push(project),
        false => repo_clone_failed.push(project),
    }

    if repo_clone_failed.len() != 0 {
        println!("clone faild: {}", repo_clone_failed.len());
        for project in &repo_clone_failed {
            print!("{} ", project.path_with_namespace);
        }
        println!("");
    } else {
        println!("all cloned success!");
    }
}

fn git_clone_command(remote_url: &str, local_path: &str) -> bool {
    let child = std::process::Command::new("git")
        .arg("clone")
        .arg(remote_url)
        .arg(local_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = child.wait_with_output().unwrap();
    print!("\n\n");

    return output.status.success();
}

// fn cli() -> Command<'static> {
//     Command::new("gitlabOp")
//         .about("A gitlab CLI")
//         .subcommand_required(true)
//         .arg_required_else_help(true)
//         .allow_external_subcommands(true)
//         .allow_invalid_utf8_for_external_subcommands(true)
//         .subcommand(
//             Command::new("clone")
//                 .about("Clones repos")
//                 .arg(arg!(<REMOTE> "The remote to clone"))
//                 .arg_required_else_help(true),
//         )
//     // .subcommand(
//     //     Command::new("pull")
//     //         .about("adds things")
//     //         .arg_required_else_help(true)
//     //         .arg(arg!(<PATH> ... "Stuff to add").allow_invalid_utf8(true)),
//     // )
//     // .subcommand(
//     //     Command::new("stash")
//     //         .args_conflicts_with_subcommands(true)
//     //         .args(push_args())
//     //         .subcommand(Command::new("push").args(push_args()))
//     //         .subcommand(Command::new("pop").arg(arg!([STASH])))
//     //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
//     // )
// }
