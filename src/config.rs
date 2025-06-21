pub struct WorkspaceConfig {
    pub base_dir: String,
    pub branch_prefix: String,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            base_dir: "../workspaces".to_string(),
            branch_prefix: "work/".to_string(),
        }
    }
}

pub fn load_config() -> WorkspaceConfig {
    println!("Loading config (using defaults for now)");
    WorkspaceConfig::default()
}
