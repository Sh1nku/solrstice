#!/bin/sh -e
set -x
paths="tests solrstice"

autoflake --remove-all-unused-imports --recursive --remove-unused-variables --in-place $paths --exclude=__init__.py
isort $paths --profile black
black $paths