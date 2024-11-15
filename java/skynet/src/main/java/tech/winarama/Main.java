package tech.winarama;

import tech.winarama.machine.EvilRobot;
import tech.winarama.machine.GoodRobot;
import tech.winarama.machine.Robot;

public class Main {
    public static void main(String[] args) {

        Robot robot = new Robot(3);
        GoodRobot goodRobot = new GoodRobot(4);
        EvilRobot evilRobot = new EvilRobot(5);

        System.out.println();
        System.out.println("default_robot\t>>\t\tid: [" + robot.getId() + "] \tzap: [" + robot.zap() + "] \t\t\tbeep: [" + robot.beep() + "] \t\t\tboop: [" + robot.boop() + "]");
        System.out.println("good_robot\t\t>>\t\tid: [" + goodRobot.getId() + "] \tzap: [" + goodRobot.zap() + "] \tbeep: [" + goodRobot.beep() + "] \t\tboop: [" + goodRobot.boop() + "]");
        System.out.println("evil_robot\t\t>>\t\tid: [" + evilRobot.getId() + "] \tzap: [" + evilRobot.zap() + "] \tbeep: [" + evilRobot.beep() + "] \t\tboop: [" + evilRobot.boop() + "]");

        System.out.println();
        System.out.println("good_robot\t>>\t" + goodRobot.help());
        System.out.println("evil_robot\t>>\t" + evilRobot.destroy());
    }
}