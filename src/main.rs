use std::rc::Rc;

use chrono::NaiveDate;
use gio::Settings;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, ListBox, Orientation, SpinButton, Adjustment, HeaderBar};

// use gio::prelude::*;
// use gio::Settings;

use impfungen::Impfung;

const APP_ID: &str = "dem.hauke.Impfungen";


fn main() {
    
    let settings = Settings::new(APP_ID);


    let app = Application::new(
        Some(APP_ID),
        Default::default(),
    );

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(application: &Application){

    let mut list_of_impfungen: Vec<Impfung> = vec![Impfung::new("test".to_string(), NaiveDate::from_ymd_opt(2005, 05, 19).expect("REASON"), 6)];


    let vbox = gtk::Box::new(Orientation::Vertical, 5);


    let entry_hbox = gtk::Box::new(Orientation::Horizontal, 3);


    let name_entry = Entry::builder().placeholder_text("Name der Impfung...").build();

    let date_entry = Entry::builder()
    .placeholder_text("YYYY-MM-DD")
    .build();

    let adjustment = Adjustment::new(0.0, 0.0, 20.0, 1.0, 10.0, 0.0);
    let spin = SpinButton::new(Some(&adjustment), 1.0, 0);

    let add_button = Button::with_label("Impfung hinzufügen");


    let listbox = ListBox::new();


    let name_entry_clone = name_entry.clone();
    let date_entry_clone = date_entry.clone();
    let spin_clone = spin.clone();
    let listbox_clone = listbox.clone();



    add_button.connect_clicked(move |_| {

        let name_text = name_entry_clone.text().to_string();
        let date_text = date_entry_clone.text().to_string();
        let expiration_time = spin_clone.value_as_int();

        if format!("{name_text}{date_text}").is_empty(){
            return;
        }

        //let row = ListBoxRowExt

        let hbox = gtk::Box::new(Orientation::Horizontal, 3);


        let name = gtk::Label::new(Some(&name_text));
        let date = gtk::Label::new(Some(&date_text));
        let time = gtk::Label::new(Some(&expiration_time.to_string()));

        hbox.append(&name);
        hbox.append(&date);
        hbox.append(&time);

        listbox_clone.append(&hbox);

        

        name_entry_clone.set_text("");
        date_entry_clone.set_text("");
        adjustment.set_value(0.0);

    }
    );

    vbox.append(&entry_hbox);
    vbox.append(&listbox);

    entry_hbox.append(&name_entry);
    entry_hbox.append(&date_entry);
    entry_hbox.append(&spin);
    entry_hbox.append(&add_button);


    let header = HeaderBar::new();

    let reload_btn = Button::from_icon_name("view-refresh");


    header.pack_end(&reload_btn);


    //let list_of_impfungen_copy = list_of_impfungen.clone();
    reload_btn.connect_clicked(move |_| {

        reset(&list_of_impfungen, &listbox);
        
    });


    

    let window = ApplicationWindow::builder()
        .application(application)
        .titlebar(&header)
        .title("Impfungen")
        .default_width(600)
        .default_height(300)
        .build();

    window.set_child(Some(&vbox));
    window.present();

    reload_btn.emit_clicked(); // Damit reset ausgeführt wird

}

fn reset(list_of_impfungen: &Vec<Impfung>, listbox: &ListBox) {

    listbox.remove_all();

    for impfung in list_of_impfungen{
        append_listbox_row(listbox, impfung);
    }
}

fn append_listbox_row(listbox: &ListBox, impfung: &Impfung){


    let hbox = gtk::Box::new(Orientation::Horizontal, 3);


    let name = gtk::Label::new(Some(&impfung.name));
    // let date = gtk::Label::new(Some(&impfung.d));
    // let time = gtk::Label::new(Some(&expiration_time.to_string()));

    hbox.append(&name);
    // hbox.append(&date);
    // hbox.append(&time);

    listbox.append(&hbox);

    //listbox_clone.append(&hbox);
}

