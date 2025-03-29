use jj_lib::workspace::{DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory};

fn main() {
    let factory = DefaultWorkspaceLoaderFactory;
    let loader = factory.create(find_workspace_dir(cwd))?;

    println!("Hello, world!");
}
