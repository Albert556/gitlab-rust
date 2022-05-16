use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use clap::{arg, Command};
use gitlab::api::{self, groups, projects, Query};
use gitlab::Gitlab;
use serde::Deserialize;

// The return type of a `Project`. Note that GitLab may contain more information, but you can
// define your structure to only fetch what is needed.
#[derive(Debug, Deserialize)]
struct Project {
    id: u64,
    default_branch: Option<String>,
    ssh_url_to_repo: String,
    http_url_to_repo: String,
    web_url: String,
    name: String,
    name_with_namespace: String,
    path: String,
    path_with_namespace: String,
}

#[derive(Debug, Deserialize)]
struct Group {
    id: u64,
    name: String,
    path: String,
    visibility: String,
    web_url: String,
    full_name: String,
    full_path: String,
    parent_id: Option<u64>,
}

fn cli() -> Command<'static> {
    Command::new("gitlabOp")
        .about("A gitlab CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("clone")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
    // .subcommand(
    //     Command::new("pull")
    //         .about("adds things")
    //         .arg_required_else_help(true)
    //         .arg(arg!(<PATH> ... "Stuff to add").allow_invalid_utf8(true)),
    // )
    // .subcommand(
    //     Command::new("stash")
    //         .args_conflicts_with_subcommands(true)
    //         .args(push_args())
    //         .subcommand(Command::new("push").args(push_args()))
    //         .subcommand(Command::new("pop").arg(arg!([STASH])))
    //         .subcommand(Command::new("apply").arg(arg!([STASH]))),
    // )
}

fn main() {
    let local_path = PathBuf::from("/Users/albertguo/Documents/wiqun/wiqun_code");

    // let token = "glpat-rT4-TEfsiQYW3GWXyepa";
    // let client = Gitlab::new("gitlab.com", token).unwrap();
    let token = "ztZH-Z4FWmW6seNwtzGk";
    let client = Gitlab::new_insecure("gitlab.dev.wiqun.com", token).unwrap();

    if !local_path.exists() {
        println!("{:?} is not exists", local_path.as_os_str());
        return;
    }
    if token == "" {
        println!("token is not exists");
    }

    clone_repository(&client, &local_path);
}

fn clone_repository(client: &Gitlab, local_path: &PathBuf) {
    let mut repo_clone_fail: Vec<Project> = Vec::new();
    let mut repo_clone_success: Vec<Project> = Vec::new();

    let groups = get_all_groups(&client);
    for group in &groups {
        let projects = get_projects_in_group(client, group);

        for project in projects {
            let mut lp = local_path.clone();
            let name_with_namespace = project.name_with_namespace.clone();
            let paths = name_with_namespace.split("/");
            for p in paths {
                let pp = p.trim();
                lp.push(pp);
            }
            // println!(
            //     "Clone \"{}@{}\" to {:?}",
            //     project.name_with_namespace.replace(" / ", "/"),
            //     project.default_branch.as_ref().unwrap().as_str(),
            //     lp.as_os_str()
            // );
            if lp.exists() {
                println!("\"{}\" already exists, skip\n", project.name);
                repo_clone_success.push(project);
                continue;
            }
            let result = git_clone_command(project.ssh_url_to_repo.as_str(), &lp);
            match result {
                true => repo_clone_success.push(project),
                false => repo_clone_fail.push(project),
            }
        }
    }

    println!("\nclone result:");
    println!("success: {}", repo_clone_success.len());
    println!("failed: {}", repo_clone_fail.len());
}

fn get_all_groups(client: &Gitlab) -> Vec<Group> {
    let groups_endpoint = groups::Groups::builder()
        .top_level_only(true)
        .build()
        .unwrap();
    // let groups: Vec<Group> = groups_endpoint.query(client).unwrap();

    let groups = api::paged(groups_endpoint, api::Pagination::Limit(200))
        .query(client)
        .unwrap();
    // println!("groups_endpoint: {:?}\n", groups_endpoint);
    // println!("groups: {:?}\n", groups);

    return groups;
}

fn get_projects_in_group(client: &Gitlab, group: &Group) -> Vec<Project> {
    let group_projects_endpoint = groups::projects::GroupProjects::builder()
        .group(group.id)
        .build()
        .unwrap();
    let group_projects: Vec<Project> = group_projects_endpoint.query(client).unwrap();

    // println!("group_projects_endpoint: {:?}\n", group_projects_endpoint);
    // println!("group_projects: {:?}\n", group_projects);

    return group_projects;
}

fn get_all_projects(client: &Gitlab) -> Vec<Project> {
    let mut projects_builder = projects::Projects::builder();
    projects_builder.owned(true);
    let projects_endpoint = projects_builder.build().unwrap();
    let projects: Vec<Project> = projects_endpoint.query(client).unwrap();

    // println!("projects_endpoint: {:?}\n", projects_endpoint);
    // println!("projects: {:?}\n", projects);

    return projects;
}

fn git_clone_command(git_remote: &str, local_path: &Path) -> bool {
    let child = std::process::Command::new("git")
        .arg("clone")
        .arg(git_remote)
        .arg(local_path.as_os_str())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = child.wait_with_output().unwrap();

    return output.status.success();
}
