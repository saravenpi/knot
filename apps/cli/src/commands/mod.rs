pub mod common;
pub mod deps;
pub mod init;
pub mod package;
pub mod publish;
pub mod run;
pub mod system;
pub mod team;
pub mod variables;
pub mod version;

// Re-export functions from each module
pub use deps::{deps_add, deps_list, deps_resolve, deps_check, deps_tree, deps_outdated, deps_why, deps_sync};
pub use init::{init_project, init_package, init_app};
pub use package::{link_packages, add_package};
pub use publish::{publish_package, delete_package};
pub use run::{run_script, run_script_interactive};
pub use system::{show_status, auth_status, update_cli};
pub use team::{create_team, list_teams, team_info, add_team_member, remove_team_member};
pub use variables::{vars_list, vars_get};
pub use version::{version_bump, version_prerelease, version_set};