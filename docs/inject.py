#!/usr/bin/python3
import copy
import json
import sys
from typing import Any, Dict
import re

if __name__ == '__main__':
    if len(sys.argv) > 1: # we check if we received any argument
        if sys.argv[1] == "supports":
            # then we are good to return an exit status code of 0, since the other argument will just be the renderer's name
            sys.exit(0)

    # load both the context and the book representations from stdin
    context, book = json.load(sys.stdin)

    # Inject files based on the following pattern: {{#inject_docstring ../tests/docs/create_client_test.rs}}
    md_regex = re.compile(r'{{\s*#inject_docstring\s*(.*?)\s*}}')
    # Inject docstrings from rust, eg
    # ```rust,no_run
    # use solrstice::Client;
    # ```
    rust_regex = re.compile(r'(```.*?```)', re.DOTALL)

    for section in book['sections']:
        chapter: Dict[str, Any] = section['Chapter']
        content: str = chapter['content']
        for match in md_regex.finditer(content):
            path = match.group(1)
            with open(path, 'r') as f:
                file_content = f.read()
                rust_docstring_match = rust_regex.search(file_content)
                if not rust_docstring_match:
                    raise Exception(f"Could not find rust docstring in {path}\n{file_content}")
                rust_docstring = rust_docstring_match.group(1)
                chapter['content'] = chapter['content'].replace(match.group(0), rust_docstring).replace("///", "")
    # raise Exception(json.dumps(book, indent=2))
    print(json.dumps(book))