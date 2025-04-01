import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

wide = pd.read_excel("analysis_wide.xlsx")
# Total Issues per Program. 
# Pivot to have Programs as rows and categories as columns.
pivot = wide.pivot(index='Program', columns='Tool', values='Total').fillna(0)

# Heatmap.
sns.heatmap(pivot, annot=True, fmt=".0f", cmap="YlOrRd")
plt.title("Total Issues per Program (Clang vs Clippy)")
plt.ylabel("Program")
plt.xlabel("Tool")
plt.show()
