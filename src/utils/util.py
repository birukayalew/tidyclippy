from pathlib import Path

def read_lines_from_file(filepath):    
    filepath = Path(__file__).parent.parent / "outputs" / filepath
    try:
        with open(filepath, "r") as f:
            return f.readlines()
    except FileNotFoundError:
        print(f"❌ File not found: {filepath}")
        return None

def extract_program_name(file_path):
    return file_path.split("-")[0] 

def detect_tool_type(file_name):
    if "clippy" in file_name:
        return "clippy"
    elif "clang" in file_name:
        return "clang"
    return None
