use crate::command::Command;
use crate::AppContext;
use cursive::views::EditView;
use cursive::Cursive;

#[derive(Default)]
pub struct CopyCommand;

impl Command for CopyCommand {
    fn execute(&mut self, app: &mut Cursive) -> bool {
        let editor = app.find_name::<EditView>("Editor").unwrap();
        let mut context = app.take_user_data::<AppContext>().unwrap();

        context.clipboard = editor.get_content().to_string();

        app.set_user_data(context);
        false
    }

    fn undo(&mut self, _: &mut Cursive) {}
}
