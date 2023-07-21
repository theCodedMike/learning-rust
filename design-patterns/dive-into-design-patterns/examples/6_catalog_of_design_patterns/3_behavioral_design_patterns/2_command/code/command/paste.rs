use crate::command::Command;
use crate::AppContext;
use cursive::views::EditView;
use cursive::Cursive;

#[derive(Default)]
pub struct PasteCommand {
    backup: String,
}

impl Command for PasteCommand {
    fn execute(&mut self, app: &mut Cursive) -> bool {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context: &mut AppContext| {
            self.backup = editor.get_content().to_string();
            editor.set_content(context.clipboard.clone());
        });

        true
    }

    fn undo(&mut self, app: &mut Cursive) {
        let mut editor = app.find_name::<EditView>("Editor").unwrap();
        editor.set_content(&self.backup);
    }
}
