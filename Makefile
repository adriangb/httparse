PHONY: init build test

.init:
	rm -rf .venv
	python -m venv .venv
	./.venv/bin/pip install -r requirements-dev.txt -r requirements-bench.txt
	./.venv/bin/pre-commit install
	touch .init

.clean:
	rm -rf .init

init: .clean .init

build-develop: .init
	. ./.venv/bin/activate && maturin develop --release --strip

test: build-develop
	./.venv/bin/python test_httparse.py

lint: build-develop
	./.venv/bin/pre-commit run --all-files
