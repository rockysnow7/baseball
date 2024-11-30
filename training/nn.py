from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import r2_score

import pandas as pd


data = pd.read_csv("data/data.csv")

targets = data[["result.home_team_score", "result.away_team_score"]]
features = data.drop(columns=["result.home_team_score", "result.away_team_score"])

X_train, X_test, y_train, y_test = train_test_split(features, targets, test_size=0.2, random_state=42)
print(y_train.describe())
print()

scaler = StandardScaler()
X_train = scaler.fit_transform(X_train)
X_test = scaler.transform(X_test)