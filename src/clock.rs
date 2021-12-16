
use std::time::{Duration};
pub struct Clock {

    
    pub increment: std::time::Duration,
    pub white_time_left: std::time::Duration,
    pub black_time_left: std::time::Duration,
}

impl Clock {
    pub fn white_update(&mut self, time: std::time::Duration){
        
        self.white_time_left = self.white_time_left.saturating_add(self.increment);
        self.white_time_left = self.white_time_left.saturating_sub(time);
        
    }
    pub fn black_update(&mut self, time: std::time::Duration){
        
        self.black_time_left = self.black_time_left.saturating_add(self.increment);
        self.black_time_left = self.black_time_left.saturating_sub(time);
    }

    pub fn white_flag(&self) -> bool{
        self.white_time_left == Duration::ZERO
    }
    pub fn black_flag(&self) -> bool{
        self.black_time_left == Duration::ZERO
    }

    //rewrite to not repeat
    pub fn print_time(&self){
        println!("White {:?}", self.white_time_left);
        println!("Black {:?}", self.black_time_left);
    }
}
