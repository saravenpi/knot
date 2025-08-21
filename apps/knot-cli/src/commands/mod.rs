pub mod common;
pub mod init;
pub mod package;
pub mod build;
pub mod publish;
pub mod team;
pub mod version;
pub mod system;

// Re-export specific functions to avoid conflicts
pub use init::{init_project, init_package, init_app};
pub use package::{link_packages, add_package, install_dependencies};
pub use build::{build_apps, run_script};

// For now, re-export everything else from the old commands
pub use crate::commands_old::{
    show_status, show_info, auth_status, publish_package, delete_package,
    create_team, list_teams, team_info, add_team_member, remove_team_member,
    version_bump, version_prerelease, version_set, update_cli
};