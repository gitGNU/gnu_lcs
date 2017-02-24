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

use std::iter::*;
use std::slice::Iter;
use std::ops::Deref;
use std::u8;
use std::ops::Index;
use std::time::Duration;
use std::sync::{Arc, RwLock, Mutex};

use dmxsystem::channel::*;

pub struct SimpleLight{
    name: String,
    first_ch:u16,
    needs_update: bool,
    channels: Vec<Arc<Mutex<Channel>>>
}

impl<'a> SimpleLight{

    pub fn new(name: String, first_ch: u16) -> SimpleLight {
        SimpleLight{
            name: name,
            first_ch: first_ch,
            needs_update: false,
            channels: vec!()
        }
    }

    pub fn set(&mut self, couple: ChVal){
        self.channels[(couple.0-self.first_ch) as usize].lock().unwrap().set_value(couple.1);
        self.needs_update = true;
    }

    pub fn get_ch(&self, i:u16) -> Arc<Mutex<Channel>>{
        self.channels.index(i as usize).clone()
    }
    
    pub fn is_changed(&self) -> bool {
        self.needs_update
    }
    
    pub fn changed_ch_vals(&'a self) -> FilterMap<Iter<'a, Arc<Mutex<Channel>>>, fn(&Arc<Mutex<Channel>>) -> Option<ChVal>> {
        self.channels.iter()
            .filter_map(ch_val as _)
    }
    pub fn updated(&'a mut self){
        self.needs_update = false;
    }
}

fn ch_val(r: &Arc<Mutex<Channel>>) -> Option<ChVal>{
    let ch = r.lock().unwrap();
    if ch.is_changed(){
        Some(ch.get_ch_val())
    } else {
        None
    }
}

pub struct RGBLight {
    l: Arc<RwLock<SimpleLight>>,
    r: Fader,
    g: Fader,
    b: Fader
}

impl RGBLight {
    pub fn new(l: Arc<RwLock<SimpleLight>>, r:u16, g:u16, b:u16) -> RGBLight{
        RGBLight{
            l:l.clone(),
            r:Fader::new(l.read().unwrap().get_ch(r)),
            g:Fader::new(l.read().unwrap().get_ch(g)),
            b:Fader::new(l.read().unwrap().get_ch(b))
        }
    }
    pub fn set_color(self, r:u8, g:u8, b:u8){
        self.r.set_value(r);
        self.g.set_value(g);
        self.b.set_value(b);
        self.l.write().unwrap().needs_update = true;
    }
}

#[derive(Clone)]
pub struct Dimmer {
    l: Arc<RwLock<SimpleLight>>,
    d: Fader
}

impl Dimmer{
    pub fn new(l: Arc<RwLock<SimpleLight>>, d: u16) -> Dimmer{
        Dimmer{
            l: l.clone(),
            d: Fader::new(l.read().unwrap().get_ch(d))
        }
    }

    pub fn fade_in(&mut self, d: Duration) -> Duration{
        let a = self.d.fade_in();
        if  a != 0 {
            d / a as u32
        }
        else {
            Duration::new(0,0)
        }
    }

    pub fn fade_step(&mut self) -> bool{
        self.d.fade_step()
    }
}
