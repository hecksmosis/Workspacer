use super::*;

#[test]
pub fn add() {
    let file_config = match FileConfig::build(true) {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };

    let mut workspaces = Workspaces::new(
        file_config
            .workspaces_file
            .expect("Environment variable WORKSPACE_FILE not set")
            .as_str(),
    );
    let workspace = Workspace::new("test".to_string(), "test".to_string(), vec![]);
    workspaces.add(workspace.clone());

    assert_eq!(workspaces.workspaces[0], workspace);
}

#[test]
pub fn remove() {
    let file_config = match FileConfig::build(true) {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };

    let mut workspaces = Workspaces::new(
        file_config
            .workspaces_file
            .expect("Environment variable WORKSPACE_FILE not set")
            .as_str(),
    );
    let workspace = Workspace::new("test".to_string(), "test".to_string(), vec![]);
    workspaces.add(workspace.clone());
    workspaces.remove_from_file(&workspace);

    assert_eq!(workspaces.workspaces.len(), 0);
}

#[test]
pub fn read_from_file() {
    let file_config = match FileConfig::build(true) {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };

    let workspaces = Workspaces::new(
        file_config
            .workspaces_file
            .expect("Environment variable WORKSPACE_FILE not set")
            .as_str(),
    );
    assert_eq!(workspaces.workspaces.len(), 0);
}
