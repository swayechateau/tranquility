.PHONY: \
	build run test lint fmt clean check-license test-coverage image-run \
	docs-build docs-serve \
	ci-check dev-setup setup update

build:
	./scripts/tquil-build.sh

run:
	./scripts/tquil-run.sh $(ARGS)

test:
	./scripts/tquil-test.sh

test-coverage:
	./scripts/tquil-test-coverage.sh

lint:
	./scripts/tquil-lint.sh

fmt:
	./scripts/tquil-fmt.sh

clean:
	./scripts/tquil-clean.sh

check-license:
	./scripts/tquil-license.sh

image-run:
	./scripts/tquil-image-run.sh $(ARGS)

docs-build:
	./scripts/docs-build.sh

docs-serve:
	./scripts/docs-serve.sh

ci-check:
	./scripts/ci-check.sh

setup:
	./scripts/setup.sh

update:
	./scripts/update.sh