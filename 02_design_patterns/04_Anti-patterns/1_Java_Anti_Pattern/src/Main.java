class Foo {
	@Override
	public String toString() {
		return "Foo []";
	}

	void m() {
		System.out.println("Foo x() print");
	}
}

public class Main {
	public static void main(String[] args) {
		Bar b = new Bar();
		b.m();
		System.out.println("Hello World. Java");
	}
}

class Bar extends Foo {
}
