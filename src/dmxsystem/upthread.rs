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
    // data for microcontroller communication
    settings: TTYSettings,
}

#[derive(Sync, Send, Clone)]
pub struct UpThread{
    thread: JoinHandle<()>,
    stop_updated: Arc<(AtomicBool, Mutex<bool>, Condvar)>
}

impl Updater{
    pub fn set(devs: Vec<Arc<SimpleLight>>, ch: Receiver<Msg>, settings: TTYSettings) -> Updater{
        Updater{devs: devs, settings: settings}
    }
    pub fn start(self) -> UpThread {
        let tern = Arc::new((
            AtomicBool::default(),
            Mutex::new(false),
            Condvar::default()
        ));
        let thr = {
            let tern = tern.clone();
            thread::spawn( move || {
                let mut port = serial::open("/dev/ttyACM0").unwrap(); //add error checking
                port.write_settings(&self.settings);
                loop {
                    {
                        let &(stop, lock, cvar) = &*tern;
                        if stop.load(Ordering::Relaxed) { break; }
                        let mut updated = lock.lock().unwrap();
                        while (!*updated) {
                            cvar.wait(updated).unwrap();
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
        UpThread{thread:thr, stop_updated:tern}
    }
}
impl UpThread{
    pub fn update(&self){
        let &(ref stop, ref lock, ref cvar) = &*stop_updated;
        let mut to_update = lock.lock().unwrap();
        *to_update = true;
        cvar.notify_one();
    }
    pub fn stop(self) {
        let stop = self.(*stop_updated).0;
        stop.store(true, Ordering::Relaxed);
        self.thread.join();
    }
}
