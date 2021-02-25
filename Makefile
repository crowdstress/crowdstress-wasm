VERSION = $(shell awk -F ' = ' '$$1 ~ /version/ { gsub("\"", "-", $$2); printf("%s",$$2) }' Cargo.toml | cut -d- -f2)

.PHONY: major
major:
	cargo bump major
	make commit

.PHONY: minor
minor:
	cargo bump minor
	make commit

.PHONY: patch
patch:
	cargo bump patch
	make commit

.PHONY: commit
commit:
	git commit -am $(VERSION)
	git tag v$(VERSION)

