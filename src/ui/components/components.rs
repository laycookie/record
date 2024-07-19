use std::{path::Path, rc::Rc};

use gtk4::{prelude::*, Button, Image, Label, Orientation, Stack};

pub fn user_button(child: &gtk4::Box, stack: Rc<Stack>) {
    let boxa = gtk4::Box::new(Orientation::Horizontal, 5);
    boxa.set_vexpand(true);
    let username = Label::new(Some("Username"));

    // TODO: Check if the contact has pfp
    let image_path = Path::new("public/assets/PlaceHolderPfp.jpg");
    let avatar = Image::from_file(image_path);

    boxa.append(&avatar);
    boxa.append(&username);

    let chat = Button::new();
    chat.connect_clicked(move |_| {
        stack.set_visible_child_name("chat");
    });
    chat.set_child(Some(&boxa));
    child.append(&chat);
}
