import os
import pandas as pd
from parsers.clang_parser import parse_clang_output
from parsers.clippy_parser import parse_clippy_output
from utils.util import read_lines_from_file, extract_program_name, detect_tool_type
from pathlib import Path
from collections import defaultdict

def run_batch_analysis(output_dir="outputs", long_file="analysis_long.xlsx", wide_file="analysis_wide.xlsx"):
    rows = []
    program_tool_data = defaultdict(lambda: defaultdict(dict))  # {program: {tool: {category: count}}}.
    for file in os.listdir(output_dir):
        if not file.endswith(".txt"):
            continue

        file_path = os.path.join(file)
        tool = detect_tool_type(file)
        program = extract_program_name(file)

        if not tool:
            continue

        lines = read_lines_from_file(file_path)
        if lines is None:
            continue
        
        if tool == "clang":
            issues, _ = parse_clang_output(lines)
        elif tool == "clippy":
            issues, _ = parse_clippy_output(lines)
        else:
            continue

        for category, msgs in issues.items():
            program_tool_data[program][tool][category] = len(msgs)

        # Align categories per program.
        aligned_rows = []
        for program, tools in program_tool_data.items():
            all_categories = set()
            for tool_categories in tools.values():
                all_categories.update(tool_categories.keys())

            for tool in ['clang', 'clippy']:
                tool_cats = tools.get(tool, {})
                for category in all_categories:
                    count = tool_cats.get(category, 0)
                    aligned_rows.append({
                        "Program": program,
                        "Tool": tool,
                        "Category": category,
                        "Issue Count": count
                    })

        

        df = pd.DataFrame(aligned_rows)
        df.sort_values(["Program", "Tool", "Category"], inplace=True)

        # Save long format.
        df.to_excel(long_file, index=False)

        # Create wide format.
        wide_df = df.pivot_table(
            index=["Program", "Tool"],
            columns="Category",
            values="Issue Count",
            aggfunc="sum",
            fill_value=0
        )

        # Add a total column.
        wide_df["Total"] = wide_df.sum(axis=1)

        # Reset index so Program and Tool become columns again.
        wide_df = wide_df.reset_index()

        # Save wide format.
        wide_df.to_excel(wide_file, index=False)

        
        

    