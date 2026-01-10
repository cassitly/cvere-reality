from ..assembler import CVEREAssembler

from sys import argv
from pathlib import Path

def main():
    """Example entrypoint"""
    assembler = CVEREAssembler()
    filepath = argv[1] if len(argv) > 1 else exit("Error: Missing filepath")
    print(assembler.assemble_to_hex(Path(filepath).read_text()))

if __name__ == "__main__":
    main()