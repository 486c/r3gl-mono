use crate::{project::projects::Projects, load_or_default, save, editor::Editor};

pub struct State {
    pub projects: Projects,
    pub editor: Editor,
}

impl State {
    pub fn new() -> Self {
        return Self {
            projects: load_or_default("projects.toml"),
            editor: Editor::new(),
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        save(&self.projects, "projects.toml");
    }
}