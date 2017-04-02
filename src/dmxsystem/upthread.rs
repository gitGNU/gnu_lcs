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

use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
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
    // data for microcontroller communication
    settings: TTYSettings,
}

pub struct UpThread{
    join_handle: JoinHandle<()>,
    arc: Arc<UpThreadInternals>
}

pub struct UpThreadInternals {
    stop: AtomicBool,
    lock: Mutex<bool>,
    cvar: Condvar
}

impl Updater{
    pub fn set(devs: Vec<Arc<SimpleLight>>, settings: TTYSettings) -> Updater{
        Updater{devs: devs, settings: settings}
    }
    pub fn start(self) -> UpThread {
        let arc = Arc::new(
            UpThreadInternals{
                stop: AtomicBool::default(),
                lock: Mutex::new(false),
                cvar: Condvar::default()
            }
        );
        let thr =
        {
            let controls = arc.clone();
            thread::spawn( move || {
                let mut port = serial::open("/dev/ttyACM0").unwrap(); //add error checking
                port.write_settings(&self.settings);
                loop {
                    {
                        let ref stop = controls.stop;
                        let ref lock = controls.lock;
                        let ref cvar = controls.cvar;
                        if stop.load(Ordering::Relaxed) { break; }
                        let mut updated = lock.lock().unwrap();
                        while !*updated {
                            updated = cvar.wait(updated).unwrap();
                        }
                        *updated = false;
                    }
                    for dev in self.devs.iter()
                        .filter(|d| d.is_changed()) {
                            dev.set_updated();
                            for ChVal(ch, val) in dev.changed_ch_vals(){
                                //send couple to microcontroller
                                write!(&mut port, "{}c{}v", ch, val);
                            }
                        }
                }
            })
        };
        UpThread{join_handle:thr, arc:arc}
    }
}

impl UpThread{
    pub fn update(&self){
        self.arc.update();
    }
    pub fn get_arc(&self) -> Arc<UpThreadInternals>{
        self.arc.clone()
    }
    pub fn stop(self){
        if let Ok(ut) = Arc::try_unwrap(self.arc){
            ut.stop();
            self.join_handle.join();
        } else {
            //log
        }
    }
}

impl UpThreadInternals{
    pub fn update(&self){
        let mut to_update = self.lock.lock().unwrap();
        *to_update = true;
        self.cvar.notify_one();
    }
    fn stop(self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}
