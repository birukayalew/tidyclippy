import pandas as pd
import matplotlib.pyplot as plt


df = pd.read_excel("analysis_wide.xlsx")
def plot_real_new_categories(df):
    

    # Split data.
    clang = df[df["Tool"] == "clang"].set_index("Program")
    clippy = df[df["Tool"] == "clippy"].set_index("Program")

    # Align columns.
    clang = clang.drop(columns=["Tool", "Total"], errors="ignore")
    clippy = clippy.drop(columns=["Tool", "Total"], errors="ignore")

    all_programs = clippy.index.union(clang.index)
    all_categories = clippy.columns.union(clang.columns)

    data = []
    for prog in all_programs:
        new_cats = 0
        for cat in all_categories:
            clippy_val = clippy.at[prog, cat] if (prog in clippy.index and cat in clippy.columns) else 0
            clang_val = clang.at[prog, cat] if (prog in clang.index and cat in clang.columns) else 0
            if clang_val == 0 and clippy_val > 0:
                new_cats += 1

        data.append({"Program": prog, "New Categories in Rust": new_cats})

    result_df = pd.DataFrame(data)

    # Plot
    result_df.set_index("Program")["New Categories in Rust"].plot(
        kind="bar", figsize=(10, 5), color="green"
    )
    plt.title("New Issue Categories Introduced in Rust (vs Clang)")
    plt.ylabel("Count of New Categories")
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

    result_df.to_excel("new_categories_actual.xlsx", index=False)

plot_real_new_categories(df)
