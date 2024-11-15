package tech.winarama.machine;

public class GoodRobot implements Operate {

    private int id;

    public GoodRobot(int id) {
        this.id = id;
    }

    @Override
    public int getId() {
        return id;
    }

    @Override
    public String zap() {
        return "good-zap";
    }

    @Override
    public String beep() {
        return "good-beep";
    }

    @Override
    public String boop() {
        return "good-boop";
    }

    public Object help() {
        return "helping";
    }
}
