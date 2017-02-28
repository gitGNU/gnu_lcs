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

use std::iter::*;
use std::slice::Iter;
use std::ops::Index;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::vec::Vec;

use dmxsystem::channel::*;

pub struct SimpleLight{
    name: String,
    first_ch:u16,
    needs_update: AtomicBool,
    channels: Vec<Arc<Mutex<Channel>>>
}

impl<'a> SimpleLight{

    pub fn new(name: String, first_ch: u16, number_of_chs: u16) -> SimpleLight {
        let mut tmp_vec = Vec::with_capacity(number_of_chs as _);
        for i in (first_ch..).take(number_of_chs as _) {
            tmp_vec.push(Arc::new(Mutex::new(Channel::new(i))));
        }
        SimpleLight{
            name: name,
            first_ch: first_ch,
            needs_update: AtomicBool::default(),
            channels: tmp_vec
        }
    }

    pub fn set(&self, couple: ChVal){
        self.channels[(couple.0-self.first_ch) as usize].lock().unwrap().set_value(couple.1);
        self.needs_update.store(true, Ordering::Relaxed);
    }

    pub fn get_ch(&self, i:u16) -> Arc<Mutex<Channel>>{
        self.channels.index(i as usize).clone()
    }
    
    pub fn is_changed(&self) -> bool {
        self.needs_update.load(Ordering::Relaxed)
    }
    
    pub fn changed_ch_vals(&'a self) -> FilterMap<Iter<'a, Arc<Mutex<Channel>>>, fn(&Arc<Mutex<Channel>>) -> Option<ChVal>> {
        self.channels.iter()
            .filter_map(ch_val as _)
    }
    pub fn set_updated(&'a self){
        self.needs_update.store(false, Ordering::Relaxed);
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
    l: Arc<SimpleLight>,
    r: Fader,
    g: Fader,
    b: Fader
}

impl RGBLight {
    pub fn new(l: Arc<SimpleLight>, r:u16, g:u16, b:u16) -> RGBLight{
        RGBLight{
            l:l.clone(),
            r:Fader::new(l.get_ch(r)),
            g:Fader::new(l.get_ch(g)),
            b:Fader::new(l.get_ch(b))
        }
    }
    pub fn set_color(self, r:u8, g:u8, b:u8){
        self.r.set_value(r);
        self.g.set_value(g);
        self.b.set_value(b);
        self.l.needs_update.store(true, Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct Dimmer {
    l: Arc<SimpleLight>,
    d: Fader
}

impl Dimmer{
    pub fn new(l: Arc<SimpleLight>, d: u16) -> Dimmer{
        Dimmer{
            l: l.clone(),
            d: Fader::new(l.get_ch(d))
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
    
    pub fn fade_out(&mut self, d:Duration) -> Duration{
        let a = self.d.fade_out();
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

    pub fn black_out(&mut self){
        self.d.set_value(0);
    }

}
