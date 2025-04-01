import sys
from parsers.clang_parser import parse_clang_output
from parsers.clippy_parser import parse_clippy_output
from utils.io import read_lines_from_file


def main():
    if len(sys.argv) < 3:
        print("Usage: python main.py <output_file.txt> <clang|clippy>")
        sys.exit(1)

    file_path = sys.argv[1]
    tool = sys.argv[2].lower()

    lines = read_lines_from_file(file_path)
    if lines is None:
        sys.exit(1)

    if tool == "clang":
        issues, uncategorized = parse_clang_output(lines)
    elif tool == "clippy":
        issues, uncategorized = parse_clippy_output(lines)
    else:
        print("❌ Tool type must be either 'clang' or 'clippy'")
        sys.exit(1)

    for file, issue_dict in issues.items():
        print(f"\n📄 File: {file}")
        for category, msgs in issue_dict.items():
            print(f"  🔹 {category}: {len(msgs)} issues")

    if uncategorized:
        print("\n⚠️  Uncategorized messages:")
        for msg in uncategorized:
            print("   -", msg)

if __name__ == "__main__":
    main()