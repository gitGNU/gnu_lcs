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

//facade class for the DMX system
//should offer an elegant interface

//TODO: use Arc!
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, RwLock, Arc};
use std::thread;
use std::thread::JoinHandle;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Result};
use std::option::Option;
use std::time::Duration;

use serial::posix::TTYSettings;

use dmxsystem::devs::*;
use dmxsystem::upthread::{Updater, Msg};

pub struct Universe {
    lights:  BTreeMap<String, Arc<RwLock<SimpleLight>>>,
    rgbs:    HashMap<String, RGBLight>,
    dimmers: HashMap<String, Dimmer>,
    updater: Option<(JoinHandle<()>, Sender<Msg>)>,
}

pub struct Transition(JoinHandle<()>, Sender<Msg>); 

impl Universe{
    
    fn new() -> Universe {
        Universe{
            lights:  BTreeMap::new(),
            rgbs:    HashMap::new(),
            dimmers: HashMap::new(),
            updater: None,
        }
    }
    fn load<P: AsRef<Path>>(path: P) -> Result<Universe> {
        let u = Self::new();
        let mut file = try!(File::open(path));
        let mut s = String::new();
        try!(file.read_to_string(&mut s));
        for l in s.lines(){
            
        }
        Ok(u)
    }
  // fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
  //     let mut file = try!(File::create(path));
  //     
  // }

    fn start(&mut self, settings: TTYSettings){
        let (s, r) = mpsc::channel();
        self.updater = Some((Updater::set(self.lights.values().cloned().collect::<Vec<_>>(), r, settings).start(), s));
    }
    fn stop(&mut self){
        if let Some(c) = self.updater.take() {
            c.1.send(Msg::Stop).unwrap();
            c.0.join();
        }
    }

    fn add_light(&mut self, name: String, first_ch: u16){
        self.lights.insert(name.clone(), Arc::new(RwLock::new(SimpleLight::new(name, first_ch))));
    }
    fn add_dimmer(&mut self, name: String, dimmer_ch: u16){
        self.dimmers.insert(name.clone(), Dimmer::new(self.lights.get(&name).unwrap().clone(), dimmer_ch)); //add error checking
    }
    fn add_rgb(&mut self, name: String, red_ch: u16, green_ch: u16, blue_ch:u16){
        self.rgbs.insert(name.clone(), RGBLight::new(self.lights.get(&name).unwrap().clone(), red_ch, green_ch, blue_ch)); //add error checking
    }
    fn fade_in_one(&mut self, name: String, t: Duration) -> Option<Transition> {
        if let Some(ref c) = self.updater {
            let s = c.1.clone();
            if let Some(ref mut d) = self.dimmers.get(&name) {
                let mut d = d.clone();
                let t = d.fade_in(t);
                let (tx, rx) = mpsc::channel();
                return Some(Transition(thread::spawn( move || {
                    while d.fade_step(){
                        s.send(Msg::Go).unwrap();
                        thread::sleep(t);
                        if let Ok(Msg::Stop) = rx.try_recv() {
                            break;
                        }
                    }
                }), tx))
            }
        }
        None
    }
}

impl Drop for Universe {
    fn drop(&mut self){
        if let Some(a) = self.updater.take() {
            a.1.send(Msg::Stop).unwrap();
            a.0.join();
        }
    }
}
//usually people suggest a scoped threadpool for something where that's not available, but the way this looks i'm not sure the best way to integrate it

impl Transition{
    fn stop(self){
        self.1.send(Msg::Stop).unwrap();
        self.0.join();
    }
}
