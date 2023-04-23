use super::*;

pub fn clear(path: &str) {
    let _ = std::fs::remove_file(path);
}

#[test]
pub fn add() {
    let file_config = match FileConfig::build() {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };

    let file_path = file_config.test_workspaces_file;

    let mut workspaces = Workspaces::new(&file_path.as_str());
    let workspace = Workspace::new(
        "test".to_string(),
        "test".to_string().into(),
        "cmd".to_string(),
        vec![],
    );
    workspaces.add(workspace.clone());

    assert_eq!(workspaces.workspaces[0], workspace);
    clear(file_path.as_str());
}

#[test]
pub fn remove() {
    let file_config = match FileConfig::build() {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };

    let file_path = file_config.test_workspaces_file;

    let mut workspaces = Workspaces::new(file_path.as_str());
    let workspace = Workspace::new(
        "test".to_string(),
        "test".to_string().into(),
        "cmd".to_string(),
        vec![],
    );
    workspaces.add(workspace.clone());
    workspaces.remove_from_file(&workspace);

    assert_eq!(workspaces.workspaces.len(), 0);
    clear(file_path.as_str());
}

#[test]
pub fn read_from_file() {
    let file_config = match FileConfig::build() {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };

    let file_path = file_config.test_workspaces_file;

    let workspaces = Workspaces::new(&file_path.as_str());
    assert_eq!(workspaces.workspaces.len(), 0);
    clear(file_path.as_str());
}
