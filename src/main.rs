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

use gtk::prelude::*;
use gtk::{ToolButton, SpinButton, Button, Dialog, Window, Box};
use gtk::{Builder, ApplicationWindow, MenuItem, AboutDialog};
use gtk::{Label, Entry, ComboBoxText, Orientation, Statusbar};

use lcs::dmxsystem::universe::Universe;

fn main() {

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

    let builder = Builder::new_from_string(glade_src);
    let window = builder.get_object::<ApplicationWindow>("main").unwrap();
    let tmp_menuitem = builder.get_object::<MenuItem>("about_menu").unwrap();
    let tmp_dialog = builder.get_object::<Dialog>("about").unwrap();
    let tmp_d = tmp_dialog.clone();
    tmp_dialog.connect_close(move |_|{
        tmp_d.hide();
    });
    let tmp_button: ToolButton = builder.get_object("add_light_btn").unwrap();
    
    tmp_menuitem.connect_activate(move |_| {
        tmp_dialog.show();
    });
    
    let tmp_dialog = builder.get_object::<Dialog>("add_light_dialog").unwrap();
    let tmp_d:Dialog = tmp_dialog.clone();
    tmp_button.connect_clicked(move |_| {
        tmp_d.show();
    });
    let tmp_d = tmp_dialog.clone();
    builder.get_object::<Button>("add_light_cancel").unwrap().connect_clicked(move |_| {
        tmp_d.hide();
    });
    
    builder.get_object::<Button>("add_light_ok").unwrap().connect_clicked(move |_| {
        //add_light();
        tmp_dialog.hide();
    });
    //after the first time the button gets broken!

    let tmp_menuitem: MenuItem = builder.get_object("quit_menuitem").unwrap();
    tmp_menuitem.connect_activate(|_| {
        gtk::main_quit();
    });

    let tmp_dialog = builder.get_object::<Dialog>("welcome").unwrap();
    tmp_dialog.show();
    let tmp_d = tmp_dialog.clone();
    tmp_dialog.connect_close(move |_| {
        tmp_d.destroy();
    });
    
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_all();

    let stat = builder.get_object::<Statusbar>("stat").unwrap();
    let contid = stat.get_context_id("Ready");
    stat.push(contid, "Ready");
    gtk::main();

    //let mut u = Universe::new();

    //let l = u.add_light("Par1".to_string(), 1, 4);
    
}
