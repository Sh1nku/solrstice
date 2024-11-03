import re
import tempfile
from pathlib import Path
from sys import stderr
from typing import List, Tuple

import mypy.api

import solrstice

from .helpers import find_mypy_config


def find_readme_path() -> Path:
    solrstice_path = Path(solrstice.__path__[0])  # type: ignore
    solrstice_parent = solrstice_path.parent
    if (solrstice_parent / "README.md").exists():
        return solrstice_parent / "README.md"
    raise FileNotFoundError("Could not find README.md")


def extract_readme_examples() -> List[Tuple[int, str]]:
    """Extracts code examples from the README.md file with the first line of the example"""
    readme_path = find_readme_path()
    with open(readme_path) as f:
        readme_content = f.read()
    examples = []
    extraction_regex = r"```python\n(.*?)\n```"
    for match in re.finditer(extraction_regex, readme_content, re.DOTALL):
        start_position = match.start()
        line_number = readme_content.count("\n", 0, start_position) + 1
        examples.append((line_number, match.group(1)))
    return examples


def test_readme_with_mypy() -> None:
    mypy_config = find_mypy_config()
    readme_examples = extract_readme_examples()
    file_contents: List[str] = []
    with tempfile.NamedTemporaryFile(mode="w", suffix=".py") as f:
        for example in readme_examples:
            while len(file_contents) < example[0] - 1:
                file_contents.append("\n")

            file_contents.append(f"def test_readme_example_{example[0]}() -> None:\n")
            for line in example[1].split("\n"):
                file_contents.append(f"    {line}\n")
        f.write("".join(file_contents))
        f.flush()
        results = list(mypy.api.run(["--config-file", f"{mypy_config}", f.name]))
        # Replace file path with the original file name
        results[0] = results[0].replace(f.name, str(find_readme_path()))  # type: ignore
        if results[2] != 0:
            print(f"\n{results[0]}", stderr)
            raise AssertionError("Mypy failed. See above for details.")
