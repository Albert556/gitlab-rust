use std::borrow::{Borrow, BorrowMut};
use std::io::{BufRead, BufReader, Result, Write};
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
    // let local_path = PathBuf::from("/Users/albertguo/Documents/wiqun/wiqun_code");
    let local_path = PathBuf::from(r"");


    // let token = "";
    // let client = Gitlab::new("gitlab.com", token).unwrap();
    let token = "";
    let client = Gitlab::new_insecure("gitlab.dev.wiqun.com", token).unwrap();

    let groups_name = vec!["tl", "jd", "op", "dp", "ml"];
    let groups = name_to_group(&client, &groups_name);
    groups.into_iter().for_each(|group| {
        clone_projects_under_group(&client, &group, &local_path, true);
    });
}

fn name_to_group(client: &Gitlab, groups_name: &Vec<&str>) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    let mut group_builder = api::groups::Group::builder();
    for &group_name in groups_name {
        let group_endpoint = group_builder.group(group_name).build().unwrap();
        let group = group_endpoint.query(client).unwrap();
        groups.push(group);
    }
    groups
}

fn clone_projects_under_group(
    client: &Gitlab,
    group: &Group,
    local_path: &PathBuf,
    is_recursion: bool,
) {
    let projects = get_projects_under_group(client, &group);
    projects.iter().for_each(|project| {
        let _ = clone_repository(&project, local_path);
    });
    if is_recursion {
        let subgroups = get_subgroups_under_group(client, &group, false);
        subgroups.iter().for_each(|subgroup| {
            clone_projects_under_group(client, subgroup, local_path, true);
        });
    }
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

fn clone_repository(project: &Project, local_path: &PathBuf) -> bool {
    let mut project_dir_path = local_path.clone();
    let pn = &project.path_with_namespace;
    let ss = pn.split("/");
    ss.for_each(|s| {
        project_dir_path.push(s);
    });

    let result = git_clone_command(
        project.ssh_url_to_repo.as_str(),
        project_dir_path.as_os_str().to_str().unwrap(),
        "dev"
    );
    result
}

fn git_clone_command(url: &str, local_path: &str, branch: &str) -> bool {
    let child = std::process::Command::new("git")
        .arg("clone")
        .arg("-b")
        .arg(branch)
        .arg(url)
        .arg(local_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();
    print!("\n");

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
