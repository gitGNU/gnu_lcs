/* Copyright 2017 Gianmarco Garrisi

This file is part of LCS.

LCS is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

LCS is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with LCS.  If not, see <http://www.gnu.org/licenses/>. */

extern crate lcs;
extern crate gtk;

use std::rc::Rc;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::{ToolButton, SpinButton, Button, Dialog, Window, Box};
use gtk::{Builder, ApplicationWindow, MenuItem};
use gtk::{Label, Entry, ComboBoxText, Orientation, Statusbar, Adjustment};

use lcs::dmxsystem::universe::Universe;

fn main() {

    
    let u = Rc::new(RefCell::new(Universe::new()));
    
    //let app;
    //if let ok(tmp) = Application::new(None::<&str>, gio::APPLICATION_FLAGS_NONE) {
    //    app = tmp;
    //} else {
    //    panic!("Fatal error! Unable to create Gtk Application")
    //}
    
    if gtk::init().is_err(){
        panic!("Fatal error! Unable to initialize Graphic Interface")
    }

    let glade_src = include_str!("GUI.glade");

    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();

    let window: ApplicationWindow = builder.get_object("main").unwrap();

    /* About dialog */
    let tmp_menuitem: MenuItem = builder.get_object("about_menu").unwrap();
    let tmp_dialog: Dialog = builder.get_object("about").unwrap();
    let tmp_d = tmp_dialog.clone();
    tmp_dialog.connect_close(move |_|{
        tmp_d.hide();
    });
    tmp_menuitem.connect_activate(move |_| {
        tmp_dialog.run();
        tmp_dialog.hide();
    });

    /* Add light: I could create the dialog in a separate function and destroy it */
    let tmp_button: ToolButton = builder.get_object("add_light_btn").unwrap();
    let tmp_dialog: Dialog = builder.get_object("add_light_dialog").unwrap();
    let tmp_d:Dialog = tmp_dialog.clone();
    tmp_button.connect_clicked(move |_| {
        tmp_d.run();
        tmp_d.hide();
    });
    let tmp_d = tmp_dialog.clone();
    builder.get_object::<Button>("add_light_cancel").unwrap().connect_clicked(move |_| {
        tmp_d.hide();
    });

    let first_ch_adj: Adjustment = builder.get_object("adjustment1").unwrap();
    let num_of_chs: Adjustment = builder.get_object("adjustment2").unwrap();
    let name: Entry = builder.get_object("light_name").unwrap();
    let u2 = u.clone();
    builder.get_object::<Button>("add_light_ok").unwrap().connect_clicked(move |_| {
        add_light(name.clone(), first_ch_adj.clone(), num_of_chs.clone(), u2.clone());
        tmp_dialog.hide();
    });
    //after the first time the button gets broken!

    let tmp_menuitem: MenuItem = builder.get_object("quit_menuitem").unwrap();
    tmp_menuitem.connect_activate(|_| {
        gtk::main_quit();
    });

    /* "splash" screen */
    let tmp_dialog: Dialog = builder.get_object("welcome").unwrap();
    let tmp_d = tmp_dialog.clone();
    tmp_dialog.connect_close(move |_| {
        tmp_d.hide();
    });
    
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_all();
    tmp_dialog.run();
    tmp_dialog.hide();

    let stat: Statusbar = builder.get_object("stat").unwrap();
    let contid = stat.get_context_id("Ready");
    stat.push(contid, "Ready");
    gtk::main();

    //let l = u.add_light("Par1".to_string(), 1, 4);
    
}

fn add_light(name: Entry, first_channel: Adjustment, number_of_channels: Adjustment, universe: Rc<RefCell<Universe>>){
    let name = name.get_text().unwrap();
    let first_channel = first_channel.get_value() as u16;
    let number_of_channels = number_of_channels.get_value() as u16;
    universe.borrow_mut().add_light(name, first_channel, number_of_channels);
}
