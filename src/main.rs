use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use gitlab::{
    api::{self, Query},
    Gitlab, Group, Project,
};

fn main() {
    // let local_path = PathBuf::from("/Users/albertguo/Documents/wiqun/wiqun_code");

    // let token = "glpat-rT4-TEfsiQYW3GWXyepa";
    // let client = Gitlab::new("gitlab.com", token).unwrap();
    let token = "ztZH-Z4FWmW6seNwtzGk";
    let client = Gitlab::new_insecure("gitlab.dev.wiqun.com", token).unwrap();

    // let all_groups = get_all_groups(&client);
    let mut all_groups: Vec<Group> = vec![];
    let groups_name = vec!["tl", "jd"];
    groups_name.iter().for_each(|&group_name| {
        let groupe = api::groups::Group::builder()
            .group(group_name)
            .build()
            .unwrap();
        let group = groupe.query(&client).unwrap();
        let mut subgroups: Vec<Group> = get_subgroup_in_group(&client, &group);
        all_groups.push(group);
        all_groups.append(&mut subgroups);
    });
    all_groups.iter().for_each(|group_ptr| {
        let projects = get_projects_in_group(&client, group_ptr);
        projects.iter().for_each(|project_ptr: &Project| {
            print!("will clone {}", project_ptr.path_with_namespace);
            git_clone_command(
                project_ptr.ssh_url_to_repo.as_str(),
                std::env::current_dir()
                    .unwrap()
                    .join("code")
                    .join(project_ptr.path_with_namespace.as_str())
                    .as_os_str()
                    .to_str()
                    .unwrap(),
            );
        });
    });
}

// fn get_last_path_string(path: &Path) -> &str {
//     match path.file_name() {
//         Some(last) => last.to_str().unwrap(),
//         None => "",
//     }
// }

// fn get_all_groups(client: &Gitlab) -> Vec<Group> {
//     let groups_endpoint = api::groups::Groups::builder()
//         .top_level_only(false)
//         // 注释说是用 full path 但是结果并不是，可能是用 path 排序
//         // 暂时不用排序，从结果看相对有序
//         .order_by(api::groups::GroupOrderBy::Path)
//         .build()
//         .unwrap();
//     let groups: Vec<Group> = api::paged(groups_endpoint, api::Pagination::All)
//         .query(client)
//         .unwrap();

//     return groups;
// }

fn get_subgroup_in_group(client: &Gitlab, group: &Group) -> Vec<Group> {
    let subgroup_endpoint = api::groups::subgroups::GroupSubgroups::builder()
        .group(group.id.value())
        .build()
        .unwrap();
    let groups: Vec<Group> = subgroup_endpoint.query(client).unwrap();
    return groups;
}

fn get_projects_in_group(client: &Gitlab, group: &Group) -> Vec<Project> {
    let group_projects_endpoint = api::groups::projects::GroupProjects::builder()
        .group(group.id.value())
        .build()
        .unwrap();
    let group_projects: Vec<Project> = group_projects_endpoint.query(client).unwrap();

    // println!("group_projects_endpoint: {:?}\n", group_projects_endpoint);
    // println!("group_projects: {:?}\n", group_projects);

    return group_projects;
}

// fn get_all_projects(client: &Gitlab) -> Vec<Project> {
//     let projects_endpoint = api::projects::Projects::builder().build().unwrap();
//     let projects: Vec<Project> = projects_endpoint.query(client).unwrap();

//     // println!("projects_endpoint: {:?}\n", projects_endpoint);
//     // println!("projects: {:?}\n", projects);

//     return projects;
// }

// fn clone_repository(project: Project, local_path: &PathBuf) {
//     let mut repo_clone_success: Vec<Project> = vec![];
//     let mut repo_clone_failed: Vec<Project> = vec![];
//     let result = git_clone_command(project.ssh_url_to_repo.as_str(), &local_path);
//     match result {
//         true => repo_clone_success.push(project),
//         false => repo_clone_failed.push(project),
//     }

//     println!("clone result:");
// }

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
