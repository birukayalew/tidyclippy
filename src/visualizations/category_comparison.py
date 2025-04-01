
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Bar chart comparing issue types between Clang and Clippy. 

df = pd.read_excel("analysis_long.xlsx")

"""
change = ((clang - clippy)/ clang) * 100
"""
# ---- 1. Log-scaled Bar Chart for Combined View ----
def plot_category_comparison_logscale(df):

    # Pivot data for plotting (log scale).
    df["Log Issue Count"] = df["Issue Count"].apply(lambda x: np.log10(x + 1))
    log_pivot_df = df.pivot_table(index="Category", columns="Tool", values="Log Issue Count", aggfunc="sum").fillna(0)

    # Also get raw counts for % calculation.
    raw_pivot_df = df.pivot_table(index="Category", columns="Tool", values="Issue Count", aggfunc="sum").fillna(0)

    ax = log_pivot_df.plot(kind="bar", figsize=(12, 6), log=False)
    plt.title("Issue Categories (Log10 Scale with % Change from Clang to Clippy)")
    plt.ylabel("Log10(Issue Count + 1)")
    plt.xlabel("Category")
    plt.xticks(rotation=45)

    # Annotate with percentage change.
    for idx, category in enumerate(log_pivot_df.index):
        if category in raw_pivot_df.index:
            clang_val = raw_pivot_df.loc[category].get("clang", 0)
            clippy_val = raw_pivot_df.loc[category].get("clippy", 0)

            if clang_val == 0 and clippy_val > 0:
                text = "New"  # New issue in Rust.
                color = "red"
            elif clang_val == 0 and clippy_val == 0:
                text = "0%"
                color = "gray"
            else:
                pct_change = ((clang_val - clippy_val) / clang_val) * 100
                text = f"{pct_change:+.1f}%"
                color = "green" if pct_change > 0 else "red"

            # Place the annotation above the tallest bar.
            y_max = max(log_pivot_df.loc[category])
            ax.text(
                idx,
                y_max + 0.2,
                text,
                ha='center',
                va='bottom',
                fontsize=9,
                color=color,
                fontweight='bold'
            )

    plt.tight_layout()
    plt.show()

plot_category_comparison_logscale(df)
