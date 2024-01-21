use crate::gui::{Button, Dialog};

pub struct NewButton;

impl Button for NewButton {
    fn render(&self) {
        println!("render a new kind of button");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click the new button");
    }
}

pub struct NewDialog;

impl Dialog for NewDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(NewButton)
    }
}