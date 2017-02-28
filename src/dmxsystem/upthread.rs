/*
Copyright 2017 Gianmarco Garrisi

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

use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::thread;
use std::io::Write;

use serial;
use serial::prelude::*;
use serial::posix::TTYSettings;
use serial::SerialDevice;

use dmxsystem::devs::*;
use dmxsystem::channel::ChVal;

pub struct Updater{
    devs: Vec<Arc<SimpleLight>>,
    ch: Receiver<Msg>,
    // data for microcontroller communication
    settings: TTYSettings
}

pub enum Msg {
    Go,
    Stop
}

impl Updater{
    pub fn set(devs: Vec<Arc<SimpleLight>>, ch: Receiver<Msg>, settings: TTYSettings) -> Updater{
        Updater{devs: devs, ch:ch, settings: settings}
    }
    pub fn start(self) -> JoinHandle<()> {
        thread::spawn( move || {
                       let mut port = serial::open("/dev/ttyACM0").unwrap(); //add error checking
                       port.write_settings(&self.settings);
                       loop {
                           match self.ch.recv().unwrap() {
                               Msg::Go =>
                                   for mut dev in self.devs.iter()
                                   .filter(|d| d.is_changed()) {
                                       dev.set_updated();
                                       for ChVal(ch, val) in dev.changed_ch_vals(){
                                           //send couple to microcontroller
                                           write!(&mut port, "{}c{}v", ch, val);
                                       }
                                   },
                               Msg::Stop => break
                           }
                       }
        })
    }
}
