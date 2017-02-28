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

use std::time::Duration;
use std::u8;
use std::sync::{Arc, Mutex};

///A couple channel, value
pub struct ChVal(pub u16, pub u8);

#[derive(Copy, Clone)]
enum Dir{
    Forward,
    Backward
}

///A base channel. Can be decorated linking it from other structs.
pub struct Channel{
    ch_num: u16,
    value: u8,
    needs_update: bool,
}

///Adds a snapping functionality to a channel. It is designed for
///channels that only cares about specific ranges of values 
pub struct Snapping<'a>{
    ch: &'a Channel,
    possible_values:Vec<ChVal>,
}

///Adds functionality to channel. It is designed for those channels
///that allow sliding in some ranges with different meaning of ranges
pub struct Segmented<'a>{
    ch: &'a Channel,
    
}

impl Channel {
    pub fn new(ch_num:u16) -> Self {
        Channel{
            ch_num: ch_num,
            value: 0,
            needs_update: false
        }
    }
    pub fn is_changed(&self) -> bool{
        self.needs_update
    }
    pub fn reset_changed(&mut self){
        self.needs_update = false;
    }
    pub fn get_ch_val(&self) -> ChVal{
        ChVal(self.ch_num, self.value)
    }
    pub fn set_value(&mut self, value: u8) {
        self.value=value;
        self.needs_update = true;
    }
}

///Adds fading functionality to channel. It is designed for channels
///that need fading, like dimmers, colors, movements.
#[derive(Clone)]
pub struct Fader{
    ch: Arc<Mutex<Channel>>,
    target: u8,
    step: u8,
    direction: Dir
}
impl Fader {

    ///Creates a new fader.
    pub fn new(ch: Arc<Mutex<Channel>>) -> Fader{
        Fader{ch:ch.clone(), target: 0u8, step:1u8, direction: Dir::Forward}
    }
    pub fn set_value(&self, value: u8) {
        self.ch.lock().unwrap().set_value(value);
    }

    ///Prepares for a fading from current value to maximum.
    ///Returns maximum - current value
    pub fn fade_in(&mut self) -> u8{
        self.target = u8::max_value();
        self.step = 1;
        self.direction = Dir::Forward;
        self.target-self.ch.lock().unwrap().value
    }
    ///Prepares for a fading from current value to minimum.
    ///Returns current value - minimum
    pub fn fade_out(&mut self) -> u8{
        self.target = 0u8;
        self.step = 1;
        self.direction = Dir::Backward;
        self.ch.lock().unwrap().value - self.target
    }
    ///Prepares for a fading to a specified value.
    ///Returns abs(current value - target value)
    pub fn fade_to_value(&mut self, value:u8) -> u8{
        self.target = value;
        self.step = 1;
        if self.target < self.ch.lock().unwrap().value {
            self.direction = Dir::Backward;
            self.ch.lock().unwrap().value - self.target
        } else {
            self.direction = Dir::Forward;
            self.target - self.ch.lock().unwrap().value
        }
    }

    pub fn set_step(&mut self, step: u8){
        self.step = step;
    }

    ///Does a step fading to the target. Returns false when the target is reached.
    ///Use it as a condition in a while loop: it will finish when the fading is
    ///completed.
    pub fn fade_step(&mut self) -> bool{
        //can be used as a contition for while loops
        match self.direction {
            Dir::Forward  => self.ch.lock().unwrap().set_value(self.ch.lock().unwrap().value-self.step),
            Dir::Backward => self.ch.lock().unwrap().set_value(self.ch.lock().unwrap().value+self.step)
        }
        if self.ch.lock().unwrap().value == self.target {
            return false;
        }
        true
    }
}
