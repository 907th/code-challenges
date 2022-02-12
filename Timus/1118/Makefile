.PHONY: build_and_run
build_and_run: build run

BUILD_ARGS=-x c++ -std=c++2a -O2 -Wl,-stack_size,0x4000000

.PHONY: build
build:
	g++ ${BUILD_ARGS} -o solution.exe solution.cpp

.PHONY: run
run:
	./solution.exe
