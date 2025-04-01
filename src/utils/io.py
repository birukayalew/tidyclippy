from pathlib import Path

def read_lines_from_file(filepath):    
    filepath = Path(__file__).parent.parent / "outputs" / filepath
    try:
        with open(filepath, "r") as f:
            return f.readlines()
    except FileNotFoundError:
        print(f"❌ File not found: {filepath}")
        return None