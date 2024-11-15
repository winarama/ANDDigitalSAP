//! Robots are here to take over the world
#![allow(unused_imports)]

/// Traits for robots
pub trait Operate {
    fn get_id(self: &Self) -> Result<u64, String>;
    fn zap(self: &Self) -> Result<String, String>;
    fn beep(self: &Self) -> Result<String, String>;
    fn boop(self: &Self) -> Result<String, String>;
}


/// Struct for Robot
#[derive(Debug)]
pub struct Robot {
    pub id: u64,
}

/// Impl blocks for Robot
impl Robot {
    pub fn new(id: u64) -> Robot {
        Robot {
            id: id,
        }
    }
}

/// Implementation of Operate trait for Robot
impl Operate for Robot {
    fn get_id(self: &Self) -> Result<u64, String> { return Ok(self.id); }
    fn zap(self: &Self) -> Result<String, String> { return Ok(String::from("zap")); }
    fn beep(self: &Self) -> Result<String, String> { return Ok(String::from("beep")); }
    fn boop(self: &Self) -> Result<String, String> { return Ok(String::from("boop")); }
}

/// Implementation of Clone trait for Robot
impl Clone for Robot {
    fn clone(&self) -> Self {
        *self
    }
}

/// Implementation of Copy trait for Robot
impl Copy for Robot { }


/// Struct for GoodRobot
#[derive(Debug)]
pub struct GoodRobot {
    pub id: u64,
}

/// Impl blocks for GoodRobot
impl GoodRobot {
    pub fn new(id: u64) -> GoodRobot {
        GoodRobot {
            id: id,
        }
    }

    pub fn help(self: &Self) -> String {
        return String::from("helping");
    }
}

/// Implementation of Operate trait for GoodRobot
impl Operate for GoodRobot {
    fn get_id(self: &Self) -> Result<u64, String> { return Ok(self.id); }
    fn zap(self: &Self) -> Result<String, String> { return Ok(String::from("good-zap")); }
    fn beep(self: &Self) -> Result<String, String> { return Ok(String::from("good-beep")); }
    fn boop(self: &Self) -> Result<String, String> { return Ok(String::from("good-boop")); }
}

/// Implementation of Clone trait for GoodRobot
impl Clone for GoodRobot {
    fn clone(&self) -> Self {
        *self
    }
}

/// Implementation of Copy trait for GoodRobot
impl Copy for GoodRobot { }


/// Struct for EvilRobot
#[derive(Debug)]
pub struct EvilRobot {
    pub id: u64,
}

/// Impl blocks for EvilRobot
impl EvilRobot {
    pub fn new(id: u64) -> EvilRobot {
        EvilRobot {
            id: id,
        }
    }

    pub fn destroy(self: &Self) -> String {
        return String::from("destroying");
    }
}

/// Implementation of Operate trait for EvilRobot
impl Operate for EvilRobot {
    fn get_id(self: &Self) -> Result<u64, String> { return Ok(self.id); }
    fn zap(self: &Self) -> Result<String, String> { return Ok(String::from("evil-zap")); }
    fn beep(self: &Self) -> Result<String, String> { return Ok(String::from("evil-beep")); }
    fn boop(self: &Self) -> Result<String, String> { return Ok(String::from("evil-boop")); }
}

/// Implementation of Clone trait for EvilRobot
impl Clone for EvilRobot {
    fn clone(&self) -> Self {
        *self
    }
}

/// Implementation of Copy trait for EvilRobot
impl Copy for EvilRobot { }