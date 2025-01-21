from pathlib import Path

import solrstice


def find_mypy_config() -> Path:
    solrstice_path = Path(solrstice.__path__[0])  # type: ignore
    solrstice_parent = solrstice_path.parent
    if (solrstice_parent / "mypy.ini").exists():
        return solrstice_parent / "mypy.ini"
    raise FileNotFoundError("Could not find mypy.ini")
