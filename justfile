set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

build:
    ./scripts/tquil-build.sh

run *args:
    ./scripts/tquil-run.sh {{args}}

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

image-run *args:
    ./scripts/tquil-image-run.sh {{args}}

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