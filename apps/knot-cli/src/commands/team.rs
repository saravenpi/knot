use anyhow::Result;
use inquire::{Text, Select};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::IsTerminal;

// API structures
#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct UserProfile {
    id: String,
    username: String,
    email: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Serialize, Deserialize)]
struct CreateTeamRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Team {
    id: String,
    name: String,
    description: Option<String>,
    #[serde(rename = "ownerId")]
    owner_id: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    owner: UserProfile,
    _count: TeamCounts,
    members: Vec<TeamMember>,
}

#[derive(Serialize, Deserialize)]
struct TeamCounts {
    members: u32,
    packages: u32,
}

#[derive(Serialize, Deserialize)]
struct TeamMember {
    id: String,
    #[serde(rename = "teamId")]
    team_id: String,
    #[serde(rename = "userId")]
    user_id: String,
    role: String,
    #[serde(rename = "joinedAt")]
    joined_at: String,
    user: UserProfile,
}

#[derive(Serialize, Deserialize)]
struct AddTeamMemberRequest {
    username: String,
    role: String, // Should be "owner", "admin", or "member"
}

// Helper functions
fn get_knot_space_url() -> String {
    env::var("KNOT_SPACE_URL").unwrap_or_else(|_| "https://knot-space-production.up.railway.app".to_string())
}

fn get_auth_token() -> Result<Option<String>> {
    match env::var("KNOT_TOKEN") {
        Ok(token) if !token.trim().is_empty() => Ok(Some(token)),
        _ => Ok(None),
    }
}

fn require_auth_token() -> Result<String> {
    get_auth_token()?
        .ok_or_else(|| anyhow::anyhow!("Authentication required. Set KNOT_TOKEN environment variable."))
}

fn format_api_error(status: reqwest::StatusCode, response_text: &str) -> String {
    // Try to parse as JSON first to extract the actual error message
    let error_message = if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(response_text) {
        json_value
            .get("error")
            .or_else(|| json_value.get("message"))
            .and_then(|v| v.as_str())
            .unwrap_or(response_text)
            .to_string()
    } else {
        response_text.to_string()
    };

    // Provide user-friendly context based on status code
    match status.as_u16() {
        400 => format!("❌ {}", error_message),
        401 => format!("🔐 Authentication failed: {}", error_message),
        403 => format!("🚫 Permission denied: {}", error_message),
        404 => format!("🔍 Not found: {}", error_message),
        409 => format!("⚠️  Conflict: {}", error_message),
        500 => format!("💥 Server error: {}", error_message),
        502 | 503 | 504 => format!("🌐 Service temporarily unavailable: {}", error_message),
        _ => format!("❌ Error ({}): {}", status.as_u16(), error_message),
    }
}

// Check if running in interactive environment
fn is_interactive() -> bool {
    std::io::stdin().is_terminal()
}

// Helper function for interactive input with beautiful UI
fn prompt_for_input(prompt: &str, default: Option<&str>) -> Result<String> {
    // Fallback to default if not interactive
    if !is_interactive() {
        if let Some(default_val) = default {
            return Ok(default_val.to_string());
        } else {
            anyhow::bail!("Interactive input required but running in non-interactive environment. Please provide values via command line arguments.");
        }
    }

    let mut text_prompt = Text::new(prompt);

    if let Some(default_val) = default {
        text_prompt = text_prompt.with_default(default_val);
    }

    let is_required = default.is_none();
    text_prompt = text_prompt.with_validator(move |input: &str| {
        if input.trim().is_empty() && is_required {
            Ok(inquire::validator::Validation::Invalid("This field is required".into()))
        } else if !input.trim().is_empty() && !input.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == ' ') {
            Ok(inquire::validator::Validation::Invalid("Please use only letters, numbers, hyphens, underscores, and spaces".into()))
        } else {
            Ok(inquire::validator::Validation::Valid)
        }
    });

    Ok(text_prompt.prompt()?)
}

// Helper function to fetch teams and allow interactive selection
async fn select_team_interactively(prompt_message: &str) -> Result<String> {
    if !is_interactive() {
        anyhow::bail!("Interactive mode required but running in non-interactive environment");
    }

    // Fetch available teams
    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams", base_url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch teams list");
    }

    let response_text = response.text().await?;
    let api_response: ApiResponse<Vec<Team>> = match serde_json::from_str(&response_text) {
        Ok(resp) => resp,
        Err(_) => anyhow::bail!("Failed to parse teams response"),
    };

    if !api_response.success {
        anyhow::bail!("Server error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string()));
    }

    let teams = api_response.data.unwrap_or_default();

    if teams.is_empty() {
        anyhow::bail!("No teams found. Create a team first with 'knot team create'");
    }

    // Create selection options
    let team_options: Vec<String> = teams
        .iter()
        .map(|team| {
            if let Some(desc) = &team.description {
                format!("{} - {}", team.name, desc)
            } else {
                team.name.clone()
            }
        })
        .collect();

    println!("🔍 Available teams:");
    let selection = Select::new(prompt_message, team_options.clone())
        .with_help_message("Use arrow keys to navigate, Enter to select, Esc to cancel")
        .prompt();

    match selection {
        Ok(selected_text) => {
            // Find the team name from the selected option
            let selected_index = team_options
                .iter()
                .position(|opt| opt == &selected_text)
                .unwrap_or(0);
            Ok(teams[selected_index].name.clone())
        }
        Err(_) => {
            anyhow::bail!("Team selection cancelled");
        }
    }
}

// Helper function to get username with validation
fn prompt_for_username(prompt_message: &str) -> Result<String> {
    let text_prompt = Text::new(prompt_message)
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(inquire::validator::Validation::Invalid("Username is required".into()))
            } else if !input.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
                Ok(inquire::validator::Validation::Invalid("Username can only contain letters, numbers, hyphens, and underscores".into()))
            } else {
                Ok(inquire::validator::Validation::Valid)
            }
        });

    Ok(text_prompt.prompt()?)
}

// Public commands
pub async fn create_team(name: Option<&str>, description: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;

    // Interactive prompts for missing arguments
    let team_name = match name {
        Some(n) => n.to_string(),
        None => {
            if is_interactive() {
                println!("🎉 Creating a new team!");
                println!();
                prompt_for_input("💼 Team name", None)?
            } else {
                anyhow::bail!("Team name is required. Use: knot team create <name>");
            }
        }
    };

    let team_description = match description {
        Some(d) => Some(d.to_string()),
        None => {
            if is_interactive() {
                match prompt_for_input("📝 Team description (optional)", Some("")) {
                    Ok(desc) if !desc.trim().is_empty() => Some(desc),
                    _ => None,
                }
            } else {
                None
            }
        }
    };

    let request = CreateTeamRequest {
        name: team_name,
        description: team_description,
    };

    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams", base_url);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        match serde_json::from_str::<ApiResponse<Team>>(&response_text) {
            Ok(api_response) => {
                if api_response.success {
                    if let Some(team) = api_response.data {
                        println!("👥 Created team: {}", team.name);
                        if let Some(desc) = team.description {
                            println!("   Description: {}", desc);
                        }
                    } else {
                        anyhow::bail!("Server response was successful but contained no data.");
                    }
                } else {
                    anyhow::bail!("Server reported an error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
            Err(_) => {
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to create team: {}", error_message);
                        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to create team: {}", message);
                        }
                        else {
                            anyhow::bail!("Failed to parse the server response. The server sent an unexpected JSON object:\n{}", response_text);
                        }
                    }
                    Err(_) => {
                        anyhow::bail!("Failed to parse the server response. The server sent the following unexpected response:\n{}", response_text);
                    }
                }
            }
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Failed to create team: {}", formatted_error);
    }

    Ok(())
}

pub async fn list_teams() -> Result<()> {
    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams", base_url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        match serde_json::from_str::<ApiResponse<Vec<Team>>>(&response_text) {
            Ok(api_response) => {
                if api_response.success {
                    if let Some(teams) = api_response.data {
                        if teams.is_empty() {
                            println!("No teams found.");
                        } else {
                            println!("👥 Teams:");
                            for team in teams {
                                println!(
                                    "  • {} - {}",
                                    team.name,
                                    team.description.unwrap_or("No description".to_string())
                                );
                            }
                        }
                    } else {
                        anyhow::bail!("Server response was successful but contained no data.");
                    }
                } else {
                    anyhow::bail!("Server reported an error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
            Err(_) => {
                // Attempt to parse as a generic JSON value to provide a better error message
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to list teams: {}", error_message);
                        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to list teams: {}", message);
                        } else {
                            anyhow::bail!("Failed to parse the server response. The server sent an unexpected JSON object:\n{}", response_text);
                        }
                    }
                    Err(_) => {
                        anyhow::bail!("Failed to parse the server response. The server sent the following unexpected response:\n{}", response_text);
                    }
                }
            }
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not retrieve teams: {}", formatted_error);
    }

    Ok(())
}

pub async fn team_info(name: Option<&str>) -> Result<()> {
    // Get team name either from argument or interactive selection
    let team_name = match name {
        Some(n) => n.to_string(),
        None => {
            if is_interactive() {
                println!("🔍 Select a team to view information:");
                println!();
                select_team_interactively("Select team").await?
            } else {
                anyhow::bail!("Team name is required. Use: knot team info <name>");
            }
        }
    };

    let base_url = get_knot_space_url();
    let url = format!("{}/api/teams/{}", base_url, team_name);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        match serde_json::from_str::<ApiResponse<Team>>(&response_text) {
            Ok(api_response) => {
                if api_response.success {
                    if let Some(team) = api_response.data {
                        println!("👥 Team: {}", team.name);
                        if let Some(desc) = team.description {
                            println!("   Description: {}", desc);
                        }
                        println!("   Created: {}", team.created_at);
                        println!("   Members:");
                        for member in team.members {
                            println!("     • {} ({})", member.user.username, member.role);
                        }
                    } else {
                        anyhow::bail!("Server response was successful but contained no data.");
                    }
                } else {
                    anyhow::bail!("Server reported an error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
            Err(_) => {
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(json) => {
                        if let Some(error_message) = json.get("error").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to get team info: {}", error_message);
                        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                            anyhow::bail!("Failed to get team info: {}", message);
                        } else {
                            anyhow::bail!("Failed to parse the server response. The server sent an unexpected JSON object:\n{}", response_text);
                        }
                    }
                    Err(_) => {
                        anyhow::bail!("Failed to parse the server response. The server sent the following unexpected response:\n{}", response_text);
                    }
                }
            }
        }
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not retrieve team information: {}", formatted_error);
    }

    Ok(())
}

pub async fn add_team_member(team: Option<&str>, username: Option<&str>, role: &str) -> Result<()> {
    let token = require_auth_token()?;

    // Interactive prompts for missing arguments
    let team_name = match team {
        Some(t) => t.to_string(),
        None => {
            if is_interactive() {
                println!("➕ Adding member to team");
                println!();
                select_team_interactively("Select team to add member to").await?
            } else {
                anyhow::bail!("Team name is required. Use: knot team add-member <team> <username>");
            }
        }
    };

    let username_str = match username {
        Some(u) => u.to_string(),
        None => {
            if is_interactive() {
                prompt_for_username("👤 Username to add to team")?
            } else {
                anyhow::bail!("Username is required. Use: knot team add-member <team> <username>");
            }
        }
    };

    // Interactive role selection if in interactive mode and default role is being used
    let selected_role = if is_interactive() && team.is_none() && username.is_none() {
        let role_options = vec!["member".to_string(), "admin".to_string()];
        let role_descriptions = vec![
            "member - Can view team packages and participate in team activities",
            "admin - Can manage team members and settings"
        ];
        
        println!();
        println!("🎭 Select role for the new team member:");
        let selection = Select::new("Select role", role_descriptions.clone())
            .with_help_message("Use arrow keys to navigate, Enter to select")
            .prompt();

        match selection {
            Ok(selected_desc) => {
                let selected_index = role_descriptions
                    .iter()
                    .position(|desc| desc == &selected_desc)
                    .unwrap_or(0);
                role_options[selected_index].clone()
            }
            Err(_) => {
                println!("❌ Role selection cancelled, using default role: member");
                "member".to_string()
            }
        }
    } else {
        role.to_string()
    };

    let request = AddTeamMemberRequest {
        username: username_str.clone(),
        role: selected_role,
    };

    let base_url = get_knot_space_url();
    // URL encode the team name to handle special characters
    let encoded_team = urlencoding::encode(&team_name);
    let url = format!("{}/api/teams/{}/members", base_url, encoded_team);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Added {} to team {} as {}", username_str, team_name, role);
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not add team member: {}", formatted_error);
    }

    Ok(())
}

pub async fn remove_team_member(team: Option<&str>, username: Option<&str>) -> Result<()> {
    let token = require_auth_token()?;

    // Interactive prompts for missing arguments
    let team_name = match team {
        Some(t) => t.to_string(),
        None => {
            if is_interactive() {
                println!("➖ Removing member from team");
                println!();
                select_team_interactively("Select team to remove member from").await?
            } else {
                anyhow::bail!("Team name is required. Use: knot team remove-member <team> <username>");
            }
        }
    };

    let username_str = match username {
        Some(u) => u.to_string(),
        None => {
            if is_interactive() {
                // If we're in interactive mode and no username provided, 
                // we could potentially fetch team members and let user select
                // For now, just prompt for username
                prompt_for_username("👤 Username to remove from team")?
            } else {
                anyhow::bail!("Username is required. Use: knot team remove-member <team> <username>");
            }
        }
    };

    let base_url = get_knot_space_url();
    // URL encode the team name and username to handle special characters
    let encoded_team = urlencoding::encode(&team_name);
    let encoded_username = urlencoding::encode(&username_str);
    let url = format!("{}/api/teams/{}/members/{}", base_url, encoded_team, encoded_username);

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Removed {} from team {}", username_str, team_name);
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        let formatted_error = format_api_error(status, &text);
        anyhow::bail!("Could not remove team member: {}", formatted_error);
    }

    Ok(())
}