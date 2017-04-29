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
extern crate gtk_sys;
extern crate gdk;

use std::rc::Rc;
use std::cell::{Cell, RefCell};

use gtk::prelude::*;
use gtk::{ToolButton, Button, Dialog, Box};
use gtk::{Builder, ApplicationWindow, MenuItem, Grid, ButtonBox};
use gtk::{Entry, Label, ComboBoxText, Orientation, Statusbar, Adjustment};

use gdk::enums::key;

use lcs::dmxsystem::universe::Universe;

/* the following constants contains the glade sources as strings. Change the argument of include_str! if the path of the files changes */
const GLADE_SRC: &'static str=include_str!("GUI.glade");
const ADD_LIGHT_DIALOG_SRC: &'static str=include_str!("add_light_dialog.glade");

fn main() {

    
    let u = Rc::new(RefCell::new(Universe::new()));
    let fullscreen = Rc::new(Cell::new(false));
    //let app;
    //if let ok(tmp) = Application::new(None::<&str>, gio::APPLICATION_FLAGS_NONE) {
    //    app = tmp;
    //} else {
    //    panic!("Fatal error! Unable to create Gtk Application")
    //}
    
    if gtk::init().is_err(){
        panic!("Fatal error! Unable to initialize Graphic Interface")
    }

    let builder = Builder::new();
    builder.add_from_string(GLADE_SRC).unwrap();

    let window: ApplicationWindow = builder.get_object("main").unwrap();
    /* Status bar*/
    let stat: Statusbar = builder.get_object("stat").unwrap();

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
    let u2 = u.clone();
    let w2 = window.clone();
    let s2 = stat.clone();
    tmp_button.connect_clicked(move |_| {
        add_light(u2.clone(), &w2, s2.clone());
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
    let w = window.clone();
    window.connect_key_press_event(move |_,key| {
        if key.get_keyval() == key::F11 {
            if fullscreen.get() {
                fullscreen.set(false);
                w.unfullscreen();
            } else {
                fullscreen.set(true);
                w.fullscreen();
            }
        }
        Inhibit(false)
    });
    window.show_all();
    tmp_dialog.run();
    tmp_dialog.hide();

    let contid = stat.get_context_id("Ready");
    stat.push(contid, "Ready");
    gtk::main();

    //let l = u.add_light("Par1".to_string(), 1, 4);
    
}

fn add_light(universe: Rc<RefCell<Universe>>, main_window: &ApplicationWindow, status_bar: Statusbar){

    let builder = Builder::new();
    builder.add_from_string(ADD_LIGHT_DIALOG_SRC).unwrap();

    let add_dialog: Dialog = builder.get_object("add_light_dialog").unwrap();

    //add transient parent
    add_dialog.set_transient_for(Some(main_window));
    
    let tmp_d = add_dialog.clone();
    add_dialog.connect_close(move |_| {
        tmp_d.destroy();
    });
    let tmp_d = add_dialog.clone();
    builder.get_object::<Button>("add_light_cancel").unwrap().connect_clicked(move |_| {
        tmp_d.destroy();
    });
    let tmp_d = add_dialog.clone();
    add_dialog.connect_delete_event(move |_,_|{
        tmp_d.destroy();
        Inhibit(false)
    });

    let tmp_d = add_dialog.clone();
    let first_ch_adj: Adjustment = builder.get_object("adjustment1").unwrap();
    let num_of_chs: Adjustment = builder.get_object("adjustment2").unwrap();
    let name: Entry = builder.get_object("light_name").unwrap();
    builder.get_object::<Button>("add_light_ok").unwrap().connect_clicked(move |_| {
        let name = name.get_text().unwrap();
        let first_channel = first_ch_adj.get_value() as u16;
        let number_of_channels = num_of_chs.get_value() as u16;
        //hide window
        //tmp_d.hide();
        //clear window
        if let Some(ref child) = tmp_d.get_child(){
            tmp_d.remove(child);
            child.destroy();
        }
        //draw next phase
        let mut decorations = Vec::with_capacity(number_of_channels as usize);
        let g = Grid::new();
        let b = Box::new(Orientation::Vertical, 10);
        let butt_box = ButtonBox::new(Orientation::Horizontal);
        let ok_button = Button::new_from_stock(gtk_sys::GTK_STOCK_OK);
        let cancel_button = Button::new_from_stock(gtk_sys::GTK_STOCK_CANCEL);
        {
            let tmp_d = tmp_d.clone();
            cancel_button.connect_clicked(move |_| {tmp_d.destroy()});
        }
        butt_box.add(&cancel_button);
        butt_box.add(&ok_button);
        butt_box.set_layout(gtk::ButtonBoxStyle::End);
        for i in 0..number_of_channels as i32{
            let ch_name = Label::new(Some(format!("Channel {}", i+1).as_str()));
            g.attach(&ch_name,    0, i, 1, 1);
            let decoration = ComboBoxText::new();
            decoration.append_text("Dimmer coarse");
            decoration.append_text("Dimmer fine");
            decoration.append_text("Red");
            decoration.append_text("Green");
            decoration.append_text("Blue");
            g.attach(&decoration, 1, i, 1, 1);
            decorations.push(decoration);
        }
        let decorations = RefCell::new(decorations);
        {
            let tmp_d = tmp_d.clone();
            //let name = name.clone();
            ok_button.connect_clicked(move |_| {
                for (i, decoration) in decorations.into_inner().into_iter().enumerate() {
                    if let Some(string) = decoration.get_active_text() {
                        match string.as_str() {
                            "Dimmer coarse" => universe.borrow_mut().add_dimmer(name, i as u16),
                            &_ => {}
                        }
                    }
                }
                tmp_d.destroy()
            });
        }
        let vp = gtk::Viewport::new(None, None);
        vp.add(&g);
        let sw = gtk::ScrolledWindow::new(None, None);
        b.add(&vp);
        b.add(&butt_box);
        sw.add(&b);
        tmp_d.add(&sw);
        //show window
        tmp_d.show_all();
        tmp_d.run();
        let l = universe.borrow_mut().add_light(name, first_channel, number_of_channels);
        let contid = status_bar.get_context_id("Add light");
        status_bar.push(contid, format!("Light \"{}\" added", l.get_name()).as_str());
    });

    add_dialog.run();
}
