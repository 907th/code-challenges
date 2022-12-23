SRC=solution.rs
EXE=solution.exe

.PHONY: debug
debug:
	rustc -o ${EXE} ${SRC}
	./${EXE}

.PHONY: release
release:
	rustc -O -o ${EXE} ${SRC}
	./${EXE}

.PHONY: test
test:
	rustc --test -o ${EXE} ${SRC}
	./${EXE} --nocapture

