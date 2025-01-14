from pathlib import Path
from typing import List

__all__ = ["Annotation", "extract_annotations", "format_annotations", "run_cli"]

class Location:
    file: Path
    line: int
    inline: bool
    def __init__(self, file: Path, line: int, inline: bool) -> None: ...

class Annotation:
    kind: str
    content: str
    location: Location
    def __init__(self, kind: str, content: str) -> None: ...

def extract_annotations(content: str, file_type: str) -> List[Annotation]: ...
def format_annotations(annotations: List[Annotation], format: str) -> str: ...
def run_cli(args: list[str]): ...
