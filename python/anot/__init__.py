import sys
from ._anot import Annotation, extract_annotations, format_annotations, run_cli

__all__ = ["Annotation", "extract_annotations", "format_annotations", "run_cli"]


def main():
    sys.exit(run_cli(sys.argv))
