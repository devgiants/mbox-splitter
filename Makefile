init:
	pip install poetry
	poetry install --no-root
	pre-commit install