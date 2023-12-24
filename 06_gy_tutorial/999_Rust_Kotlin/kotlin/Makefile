KC = kotlinc
JAVA = java

SOURCE_KT = ./src/Main.kt
TEST_SOURCE_KT = /junit-platform-console-standalone-1.9.3.jar
LINK_OBJ =

TARGET = ./out/Main.jar
LDFLAGS_COMMON = -include-runtime

r:
		rm -rf out
		$(KC) $(SOURCE_KT) $(LINK_OBJ) $(LDFLAGS_COMMON) -d $(TARGET)
		$(JAVA) -jar $(TARGET)
t:
		$(KC) $(SOURCE_KT) $(LDFLAGS_COMMON) -d $(TARGET)
		$(JAVA) -jar .$(TEST_SOURCE_KT) --scan-class-path

d:
		wget https://repo1.maven.org/maven2/org/junit/platform/$(TEST_SOURCE_KT)

clean:
		rm -rf out *.jar

init:
		mkdir src
		echo "fun main() {" >> src/Main.kt
		echo "	println(\"Hello, World! Kotlin lang\")" >> src/Main.kt
		echo "}" >> src/Main.kt

init2:
		mkdir src
		echo "fun main(args : Array<String>) {" >> src/Main.kt
		echo "	println(\"Hello, World! Kotlin lang\" >> src/Main.kt
		echo "}" >> src/Main.kt
