.PHONY: push
push:
	@read -p "What have you done today? " WHATDONE; \
	git add . && git commit -m "$$WHATDONE" && git push
