import os.path
import tempfile
from pathlib import Path
from sys import stderr
from typing import List, Dict

import solrstice
import doctest
import mypy.api

def find_mypy_config() -> Path:
    solrstice_path = Path(solrstice.__path__[0])
    solrstice_parent = solrstice_path.parent
    if (solrstice_parent / "mypy.ini").exists():
        return solrstice_parent / "mypy.ini"
    raise FileNotFoundError("Could not find mypy.ini")

def get_doctests_from_solrstice() -> Dict[str, List[doctest.Example]]:
    to_parse = [Path(solrstice.__path__[0])]
    doctest_parser = doctest.DocTestParser()
    doctest_examples = {}
    while to_parse:
        path = Path(to_parse.pop())
        for file in path.iterdir():
            if file.is_dir():
                to_parse.append(file)
            elif file.suffix in [".py", ".pyi"]:
                with open(file) as f:
                    doctests = doctest_parser.get_doctest(f.read(), {}, str(file), str(file), 0)
                    doctest_examples[file.name] = doctests.examples
    return doctest_examples


def test_doctests_with_mypy() -> None:
    mypy_confg = find_mypy_config()
    solrstice_examples = get_doctests_from_solrstice()
    with tempfile.TemporaryDirectory() as d:
        for file, examples in solrstice_examples.items():
            file_path = os.path.join(d, file)
            with open(file_path, "w") as f:
                file_contents: List[str] = []
                # Add spacing so that the code is at the same line in the new file as it was in the original file
                for example in examples:
                    first_example_line = example.lineno
                    # Remove 1 to account for the extra line that will be added
                    while len(file_contents) < first_example_line - 1:
                        file_contents.append("\n")
                    file_contents.append(f'def test_doctest_{example.lineno}() -> None:\n')
                    for line in example.source.split("\n"):
                        file_contents.append(f'    {line}\n')
                    file_contents.append("\n")
                f.write(''.join(file_contents))
        results = list(mypy.api.run(['--config-file', f'{mypy_confg}', d]))
        # Replace file path with the original file name
        results[0] = results[0].replace(d, solrstice.__path__[0]) # type: ignore
        if results[2] != 0:
            print(f'\n{results[0]}', stderr)
            raise AssertionError('Mypy failed. See above for details.')