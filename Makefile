
a.out: src/*.cpp
	g++ src/*.cpp src/*.hpp src/glad.c -Wall -lglfw -lGLU -lGL -ldl -Ideps/include

run: a.out
	./a.out

clean: a.out
	rm a.out -fr
