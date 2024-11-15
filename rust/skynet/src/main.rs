//! Robots are here to take over the world
#![allow(unused_imports)]
mod machine;

use machine::robots::Robot;
use machine::robots::GoodRobot;
use machine::robots::EvilRobot;
use machine::robots::Operate;

fn main() {
    let robot = Robot::new(3);
    let good_robot = GoodRobot::new(4);
    let evil_robot = EvilRobot::new(5);

    println!();
    println!("default_robot\t>>\tid: [{}] \tzap: [{}] \t\tbeep: [{}] \t\tboop: [{}]", robot.get_id().unwrap(), robot.zap().unwrap(), robot.beep().unwrap(), robot.boop().unwrap());
    println!("good_robot\t>>\tid: [{}] \tzap: [{}] \tbeep: [{}] \tboop: [{}]", good_robot.get_id().unwrap(), good_robot.zap().unwrap(), good_robot.beep().unwrap(), good_robot.boop().unwrap());
    println!("evil_robot\t>>\tid: [{}] \tzap: [{}] \tbeep: [{}] \tboop: [{}]", evil_robot.get_id().unwrap(), evil_robot.zap().unwrap(), evil_robot.beep().unwrap(), evil_robot.boop().unwrap());

    println!();
    println!("good_robot\t>>\t{}", good_robot.help());
    println!("evil_robot\t>>\t{}", evil_robot.destroy());
}

/*
fn is_evil(robot: T) -> String {

}

#[cfg(test)]
mod tests {

    #[test]
    fn is_evil() {
        let result = super::add(2, 2);
        assert_eq!(result, "destroying");
    }
}

*/