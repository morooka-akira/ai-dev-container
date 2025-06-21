pub struct WorkspaceManager;

impl WorkspaceManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_workspace(&self, task_name: &str) -> Result<(), String> {
        println!("Creating workspace for: {}", task_name);
        Ok(())
    }

    pub fn list_workspaces(&self) -> Result<Vec<String>, String> {
        println!("Listing workspaces");
        Ok(vec![])
    }
}
