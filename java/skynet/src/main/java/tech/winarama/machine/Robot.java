package tech.winarama.machine;

public class Robot implements Operate {

    private int id;

    public Robot(int id) {
        this.id = id;
    }

    @Override
    public int getId() {
        return id;
    }

    @Override
    public String zap() {
        return "zap";
    }

    @Override
    public String beep() {
        return "beep";
    }

    @Override
    public String boop() {
        return "boop";
    }
}
