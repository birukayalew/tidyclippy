import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.read_excel("analysis_long.xlsx")


def plot_issue_reduction_heatmap(df):
    # Pivot the data.
    pivot = df.pivot_table(index=["Program", "Tool"], columns="Category", values="Issue Count", aggfunc="sum", fill_value=0)
    
    # Ensure both tools exist for all rows.
    tools = pivot.index.get_level_values("Tool").unique()
    if "clang" not in tools or "clippy" not in tools:
        print("Both 'clang' and 'clippy' results are required.")
        return

    # Separate out clang and clippy rows.
    clang_df = pivot.loc[pivot.index.get_level_values("Tool") == "clang"]
    clippy_df = pivot.loc[pivot.index.get_level_values("Tool") == "clippy"]

    # Align indexes to just the Program level.
    clang_df.index = clang_df.index.droplevel("Tool")
    clippy_df.index = clippy_df.index.droplevel("Tool")

    # Compute difference: Clang - Clippy.
    diff_df = clang_df.subtract(clippy_df, fill_value=0)

    # Plot.
    plt.figure(figsize=(12, 6))
    sns.heatmap(diff_df, annot=True, cmap="RdYlGn", center=0, fmt=".0f", linewidths=0.5, cbar_kws={'label': 'Issue Reduction (Clang - Clippy)'})
    plt.title("Issue Reduction Heatmap (>0 Improvement, <0 Regression)")
    plt.ylabel("Program")
    plt.xlabel("Issue Category")
    plt.tight_layout()
    plt.show()

plot_issue_reduction_heatmap(df)
