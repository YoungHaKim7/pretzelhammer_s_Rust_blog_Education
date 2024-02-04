r:
		rm -rf out
		mkdir out
		javac -cp src src/Main.java -d out
		java -cp out Main

b:
		javac -cp src src/Main.java -d out
		java -cp out Main

d:
		wget https://repo1.maven.org/maven2/org/junit/platform/junit-platform-console-standalone/1.9.3/junit-platform-console-standalone-1.9.3.jar

t:
		java -jar ./junit-platform-console-standalone-1.9.3.jar --class-path out --scan-class-path

clean:
		rm -rf *.jar ./out

init:
		mkdir src
		echo "public class Main {" >> src/Main.java
		echo "	public static void main(String[] args) {" >> src/Main.java
		echo "		System.out.println(\42Hello World. Java\42);" >> src/Main.java
		echo "	}" >> src/Main.java
		echo "}" >> src/Main.java
