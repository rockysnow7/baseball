from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import r2_score, mean_absolute_error
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, Dropout
from tensorflow.keras.optimizers import Adam
from tensorflow.keras.callbacks import EarlyStopping

import pandas as pd


data = pd.read_csv("data/data.csv")

targets = data[["result.home_team_score", "result.away_team_score"]]
features = data.drop(columns=["result.home_team_score", "result.away_team_score"])

X_train, X_test, y_train, y_test = train_test_split(features, targets, test_size=0.2, random_state=42)
print(f"{X_train.shape=}")
print(f"{y_train.shape=}")

scaler = StandardScaler()
X_train = scaler.fit_transform(X_train)
X_test = scaler.transform(X_test)

model = Sequential([
    Dense(128, activation="relu", input_shape=(X_train.shape[1],)),
    Dropout(0.2),
    Dense(64, activation="relu"),
    Dropout(0.2),
    Dense(32, activation="relu"),
    Dense(2, activation="linear"),
])

optimizer = Adam(learning_rate=0.001)
model.compile(optimizer=optimizer, loss="mse", metrics=["mae", "r2_score"])

history = model.fit(
    X_train,
    y_train,
    validation_data=(X_test, y_test),
    epochs=200,
    batch_size=32,
    callbacks=[EarlyStopping(patience=10)],
)