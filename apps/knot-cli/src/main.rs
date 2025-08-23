mod commands;
mod config;
mod downloader;
mod ignore;
mod linker;
mod project;
mod templates;
mod typescript;

use anyhow::Result;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("knot")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Knot - Monorepo package manager")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new Knot project")
                .arg(
                    Arg::new("name")
                        .help("Project name")
                        .required(false)
                        .index(1),
                )
                .arg(
                    Arg::new("path")
                        .help("Target path for project creation (optional)")
                        .required(false)
                        .index(2),
                )
                .arg(
                    Arg::new("description")
                        .help("Project description")
                        .short('d')
                        .long("description")
                        .value_name("DESC"),
                ),
        )
        .subcommand(
            Command::new("init:package")
                .about("Initialize a new package")
                .arg(
                    Arg::new("name")
                        .help("Package name (optional, will prompt if not provided)")
                        .required(false)
                        .index(1),
                )
                .arg(
                    Arg::new("path")
                        .help("Target path for package creation (optional)")
                        .required(false)
                        .index(2),
                )
                .arg(
                    Arg::new("team")
                        .help("Team name")
                        .short('t')
                        .long("team")
                        .value_name("TEAM"),
                )
                .arg(
                    Arg::new("version")
                        .help("Initial version")
                        .short('v')
                        .long("version")
                        .value_name("VERSION"),
                )
                .arg(
                    Arg::new("template")
                        .help("Package template (typescript, react)")
                        .long("template")
                        .value_name("TEMPLATE"),
                )
                .arg(
                    Arg::new("description")
                        .help("Package description")
                        .short('d')
                        .long("description")
                        .value_name("DESC"),
                )
                .arg(
                    Arg::new("here")
                        .help("Create package in current directory instead of creating new subfolder")
                        .long("here")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("init:app")
                .about("Initialize a new app")
                .arg(Arg::new("name").help("App name (optional, will prompt if not provided)").required(false).index(1))
                .arg(Arg::new("path").help("Target path for app creation (optional)").required(false).index(2))
                .arg(
                    Arg::new("template")
                        .help("App template (react, svelte, nextjs, fastify, express, vanilla)")
                        .long("template")
                        .value_name("TEMPLATE"),
                )
                .arg(
                    Arg::new("description")
                        .help("App description")
                        .short('d')
                        .long("description")
                        .value_name("DESC"),
                )
                .arg(
                    Arg::new("here")
                        .help("Create app in current directory instead of creating new subfolder")
                        .long("here")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("link")
                .alias("l")
                .about("Link packages to apps and setup TypeScript aliases")
                .arg(
                    Arg::new("symlink")
                        .help("Use symlinks instead of copying (default: false)")
                        .long("symlink")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("run")
                .alias("r")
                .about("Run a script from knot.yml, app.yml, or package.yml")
                .arg(
                    Arg::new("script")
                        .help("Script name to run (optional - will show interactive selection if omitted)")
                        .required(false)
                        .index(1),
                ),
        )
        .subcommand(Command::new("status").alias("s").about("Show project status"))
        .subcommand(
            Command::new("auth")
                .alias("a")
                .about("Check authentication status")
        )
        .subcommand(
            Command::new("publish")
                .alias("p")
                .about("Publish a package to Knot Space")
                .arg(
                    Arg::new("team")
                        .help("Team name (optional)")
                        .short('t')
                        .long("team")
                        .value_name("TEAM"),
                )
                .arg(
                    Arg::new("description")
                        .help("Package description")
                        .short('d')
                        .long("description")
                        .value_name("DESC"),
                ),
        )
        .subcommand(
            Command::new("delete")
                .alias("d")
                .about("Delete a package from Knot Space")
                .arg(
                    Arg::new("name")
                        .help("Package name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("version")
                        .help("Package version")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("install")
                .alias("i")
                .alias("add")
                .about("Install a package dependency to the current app")
                .arg(
                    Arg::new("package")
                        .help("Package name to install (local: 'utils', online: '@jwt', team: '@team/package')")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("no-link")
                        .help("Skip automatic linking after installing")
                        .long("no-link")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("version")
                .alias("v")
                .about("Version management commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("patch")
                        .about("Bump patch version (bug fixes)")
                )
                .subcommand(
                    Command::new("minor")
                        .about("Bump minor version (new features)")
                )
                .subcommand(
                    Command::new("major")
                        .about("Bump major version (breaking changes)")
                )
                .subcommand(
                    Command::new("prerelease")
                        .about("Create pre-release version")
                        .arg(
                            Arg::new("preid")
                                .help("Pre-release identifier (alpha, beta, rc)")
                                .long("preid")
                                .value_name("ID")
                        )
                )
                .subcommand(
                    Command::new("set")
                        .about("Set specific version")
                        .arg(
                            Arg::new("version")
                                .help("Version to set (e.g., 1.2.3)")
                                .required(true)
                                .index(1)
                        )
                )
        )
        
        .subcommand(
            Command::new("team")
                .alias("t")
                .about("Team management commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new team")
                        .arg(Arg::new("name").help("Team name (optional - will prompt if not provided)").required(false).index(1))
                        .arg(
                            Arg::new("description")
                                .help("Team description")
                                .short('d')
                                .long("description")
                                .value_name("DESC"),
                        ),
                )
                .subcommand(Command::new("list").about("List all teams"))
                .subcommand(
                    Command::new("info")
                        .about("Show team information")
                        .arg(Arg::new("name").help("Team name (optional - will show selection if not provided)").required(false).index(1)),
                )
                .subcommand(
                    Command::new("add-member")
                        .about("Add member to team")
                        .arg(Arg::new("team").help("Team name").required(false).index(1))
                        .arg(
                            Arg::new("username")
                                .help("Username to add")
                                .required(false)
                                .index(2),
                        )
                        .arg(
                            Arg::new("role")
                                .help("Role (admin or member)")
                                .short('r')
                                .long("role")
                                .value_name("ROLE")
                                .value_parser(["admin", "member"])
                                .default_value("member"),
                        ),
                )
                .subcommand(
                    Command::new("remove-member")
                        .about("Remove member from team")
                        .arg(Arg::new("team").help("Team name").required(false).index(1))
                        .arg(
                            Arg::new("username")
                                .help("Username to remove")
                                .required(false)
                                .index(2),
                        ),
                ),
        )
        .subcommand(
            Command::new("upgrade")
                .alias("u")
                .about("Upgrade Knot CLI to the latest version")
                .arg(
                    Arg::new("force")
                        .help("Force upgrade even if already on latest version")
                        .long("force")
                        .action(clap::ArgAction::SetTrue),
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let path = sub_matches.get_one::<String>("path");
            let description = sub_matches
                .get_one::<String>("description")
                .map(|s| s.as_str());
            commands::init_project(name.map(|s| s.as_str()), path.map(|s| s.as_str()), description)?;
        }
        Some(("init:package", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let path = sub_matches.get_one::<String>("path");
            let team = sub_matches.get_one::<String>("team").map(|s| s.as_str());
            let version = sub_matches.get_one::<String>("version").map(|s| s.as_str());
            let template = sub_matches.get_one::<String>("template").map(|s| s.as_str());
            let description = sub_matches.get_one::<String>("description").map(|s| s.as_str());
            let here = sub_matches.get_flag("here");
            
            commands::init_package(name.map(|s| s.as_str()), team, version, template, description, path.map(|s| s.as_str()), here)?;
        }
        Some(("init:app", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name");
            let path = sub_matches.get_one::<String>("path");
            let template = sub_matches.get_one::<String>("template").map(|s| s.as_str());
            let description = sub_matches
                .get_one::<String>("description")
                .map(|s| s.as_str());
            let here = sub_matches.get_flag("here");

            commands::init_app(name.map(|s| s.as_str()), template, description, path.map(|s| s.as_str()), here)?;
        }
        Some(("link", sub_matches)) => {
            let use_symlinks = sub_matches.get_flag("symlink");
            commands::link_packages(use_symlinks).await?;
        }
        Some(("run", sub_matches)) => {
            if let Some(script_name) = sub_matches.get_one::<String>("script") {
                commands::run_script(script_name).await?;
            } else {
                commands::run_script_interactive().await?;
            }
        }
        Some(("status", _)) => {
            commands::show_status()?;
        }
        Some(("auth", _)) => {
            commands::auth_status().await?;
        }
        Some(("publish", sub_matches)) => {
            let team = sub_matches.get_one::<String>("team").map(|s| s.as_str());
            let description = sub_matches
                .get_one::<String>("description")
                .map(|s| s.as_str());
            commands::publish_package(team, description).await?;
        }
        Some(("delete", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let version = sub_matches.get_one::<String>("version").unwrap();
            commands::delete_package(name, version).await?;
        }
        Some(("install", sub_matches)) | Some(("add", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            let no_link = sub_matches.get_flag("no-link");
            let auto_link = !no_link; // Auto-link by default unless --no-link is specified
            commands::add_package(package, auto_link).await?;
        }
        Some(("version", sub_matches)) => match sub_matches.subcommand() {
            Some(("patch", _)) => {
                commands::version_bump("patch").await?;
            }
            Some(("minor", _)) => {
                commands::version_bump("minor").await?;
            }
            Some(("major", _)) => {
                commands::version_bump("major").await?;
            }
            Some(("prerelease", prerelease_sub)) => {
                let preid = prerelease_sub.get_one::<String>("preid").map(|s| s.as_str());
                commands::version_prerelease(preid).await?;
            }
            Some(("set", set_sub)) => {
                let version = set_sub.get_one::<String>("version").unwrap();
                commands::version_set(version).await?;
            }
            _ => unreachable!(),
        }
        
        Some(("team", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", team_sub)) => {
                let name = team_sub.get_one::<String>("name").map(|s| s.as_str());
                let description = team_sub
                    .get_one::<String>("description")
                    .map(|s| s.as_str());
                commands::create_team(name, description).await?;
            }
            Some(("list", _)) => {
                commands::list_teams().await?;
            }
            Some(("info", team_sub)) => {
                let name = team_sub.get_one::<String>("name").map(|s| s.as_str());
                commands::team_info(name).await?;
            }
            Some(("add-member", team_sub)) => {
                let team = team_sub.get_one::<String>("team");
                let username = team_sub.get_one::<String>("username");
                let role = team_sub.get_one::<String>("role").unwrap();
                commands::add_team_member(
                    team.map(|s| s.as_str()),
                    username.map(|s| s.as_str()),
                    role
                ).await?;
            }
            Some(("remove-member", team_sub)) => {
                let team = team_sub.get_one::<String>("team");
                let username = team_sub.get_one::<String>("username");
                commands::remove_team_member(
                    team.map(|s| s.as_str()),
                    username.map(|s| s.as_str())
                ).await?;
            }
            _ => unreachable!(),
        },
        Some(("upgrade", sub_matches)) => {
            let force = sub_matches.get_flag("force");
            commands::update_cli(force).await?;
        }
        Some((script_name, _)) => {
            // Try to run as a script if it's not a built-in command
            commands::run_script(script_name).await?;
        }
        None => {
            // No subcommand provided, show help
            Command::new("knot")
                .version(env!("CARGO_PKG_VERSION"))
                .about("Knot - Monorepo package manager")
                .print_help()?;
        }
    }

    Ok(())
}
