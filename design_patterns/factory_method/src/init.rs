use crate::gui::Dialog;
// use crate::html_gui::HtmlDialog;
use crate::windows_gui::WindowsDialog;
use crate::new_gui::NewDialog;

pub fn initialize() -> &'static dyn Dialog {
    // The dialog type is selected depending on the environment settings or configuration.
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating the HTML GUI --");
        // &HtmlDialog

        println!("creating the new GUI");
        &NewDialog
    }
}