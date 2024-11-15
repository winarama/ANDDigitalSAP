package tech.winarama.machine;

public class EvilRobot implements Operate {

    private int id;

    public EvilRobot(int id) {
        this.id = id;
    }

    @Override
    public int getId() {
        return id;
    }

    @Override
    public String zap() {
        return "evil-zap";
    }

    @Override
    public String beep() {
        return "evil-beep";
    }

    @Override
    public String boop() {
        return "evil-boop";
    }

    public Object destroy() {
        return "destroying";
    }
}
