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
use gtk::{ToolButton, SpinButton, MessageType, Dialog, Window, Box};
use gtk::{Builder, ApplicationWindow, MenuItem, AboutDialog};
use gtk::{Label, Entry, ComboBoxText, Orientation};

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

    let builder = Builder::new_from_file("GUI.glade");
    let window = builder.get_object::<ApplicationWindow>("main").unwrap();
    let mut tmp_menuitem = builder.get_object::<MenuItem>("about_menu").unwrap();
    let mut tmp_dialog = builder.get_object::<Dialog>("about").unwrap();
    let mut tmp_button: ToolButton = builder.get_object("add_light_btn").unwrap();
        
    tmp_menuitem.connect_activate(move |_| {
        tmp_dialog.show();
    });
    
    tmp_dialog = builder.get_object("add_light_dialog").unwrap();
    tmp_button.connect_clicked(move |_| {
        tmp_dialog.show();
    });
    //after the first time the button gets broken!
    let bx = builder.get_object::<gtk::Box>("add_light_box").unwrap();
    //let proto = builder.get_object::<gtk::Box>("prototype").unwrap();
    builder.get_object::<SpinButton>("num_ch").unwrap().connect_value_changed(move |b| {
        let value = b.get_value_as_int();
        let i = bx.get_children().len() - 3;
        if value < i as i32 {
            //remove last element
        } else {
            //add element
            let strng = format!("Channel {}", i+1);
            let lbl = Label::new(Some(strng.as_str()));
            let txt = Entry::new();
            let cmb = ComboBoxText::new();
            cmb.append(None, "Dimmer");
            cmb.append(None, "Red");
            cmb.append(None, "Green");
            cmb.append(None, "Blue");
            cmb.append(None, "None");
            let mut x = Box::new(Orientation::Horizontal, 0);
            x.pack_end(&lbl, false, false, 0);
            x.pack_end(&txt, false, false, 0);
            x.pack_end(&cmb, false, false, 0);

            bx.pack_end(&x, false, false, 0);
        }
    });

    tmp_menuitem = builder.get_object("quit_menuitem").unwrap();
    tmp_menuitem.connect_activate(|_| {
        gtk::main_quit();
    });

    tmp_dialog = builder.get_object::<Dialog>("welcome").unwrap();
    tmp_dialog.show();
    tmp_dialog.connect_close(move |_| {
        //tmp_dialog.destroy(); //error: cannot move `tmp_dialog` into closure because it is borrowed [E0504]
    });
    
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_all();

    gtk::main();

    //let mut u = Universe::new();

    //let l = u.add_light("Par1".to_string(), 1, 4);
    
}
