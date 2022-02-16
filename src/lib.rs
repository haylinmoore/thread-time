extern crate libc;
use std::io::{Result, Error};
use std::time::Duration;

use libc::{clock_gettime, timespec};
use libc::{pthread_self, pthread_getcpuclockid, c_int};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ThreadTime {
    pub last: Duration,
    pub clk_id: c_int,
}

impl ThreadTime {
    pub fn get_clk() -> Result<c_int> {
        let clk_id: c_int;

        unsafe { 
            let mut v = 3;
            let clock_id = &mut v as *mut i32;
            let res = pthread_getcpuclockid(pthread_self(), clock_id);
            if res != 0 {
                return Err(Error::last_os_error());
            }
            clk_id = *clock_id;
        }

        Ok(clk_id)
    }
    
    pub fn query_clk(clk_id: c_int) -> Result<Duration> {
        let mut time = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        if unsafe { clock_gettime(clk_id, &mut time) } == -1
        {
            return Err(Error::last_os_error());
        }
        Ok(Duration::new(time.tv_sec as u64, time.tv_nsec as u32))
    }

    pub fn query(&self) -> Result<Duration> {
        return Self::query_clk(self.clk_id)
    }

    pub fn new() -> Self {
        Self::try_new().unwrap()
    }

    pub fn try_new() -> Result<Self> {
        let clk_id = Self::get_clk();
        if let Err(e) = clk_id {
            return Err(e);
        }
        let clk_id = clk_id.unwrap();
        let last = Self::query_clk(clk_id);
        if let Err(e) = last {
            return Err(e);
        } 

        return Ok(Self {
            last: last.unwrap(),
            clk_id: clk_id
        });
    }

    pub fn elapsed(&self) -> Duration {
        self.query().unwrap() - self.last
    }

    pub fn now(&mut self) {
        self.last = self.query().unwrap();
    }

}
