import xgboost as xgb
import pandas as pd

# Load dataset
data = pd.read_csv("network_log.csv")
X = data.drop("congestion_flag", axis=1)
y = data["congestion_flag"]

# Train model
model = xgb.XGBClassifier()
model.fit(X, y)

# Predict congestion likelihood
predictions = model.predict(X)
print(predictions)