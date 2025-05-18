import xgboost as xgb
import pandas as pd

# Load & prepare dataset
data = pd.read_csv("network_log.csv")
X = data.drop("congestion_flag", axis=1)
y = data["congestion_flag"]

# Train model
model = xgb.XGBClassifier()
model.fit(X, y)