DEBUG_BUILD_ARGS=
RELEASE_BUILD_ARGS=-O

.PHONY: debug
debug: build_debug run

.PHONY: release
release: build_release run

.PHONY: build_debug
build_debug:
	rustc ${DEBUG_BUILD_ARGS} -o solution solution.rs

.PHONY: build_release
build_release:
	rustc ${RELEASE_BUILD_ARGS} -o solution solution.rs

.PHONY: run
run:
	./solution

