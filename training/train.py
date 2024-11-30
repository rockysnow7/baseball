from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import r2_score

# import optuna
import pandas as pd
import xgboost as xgb


data = pd.read_csv("data/data.csv")

targets = data[["result.home_team_score", "result.away_team_score"]]
features = data.drop(columns=["result.home_team_score", "result.away_team_score"])

X_train, X_test, y_train, y_test = train_test_split(features, targets, test_size=0.2, random_state=42)
print(y_train.describe())
print()

scaler = StandardScaler()
X_train = scaler.fit_transform(X_train)
X_test = scaler.transform(X_test)

dtrain = xgb.DMatrix(X_train, y_train)
dtest = xgb.DMatrix(X_test, y_test)


# def objective(trial: optuna.Trial) -> float:
#     params = {
#         "objective": "reg:squarederror",
#         "learning_rate": trial.suggest_float("learning_rate", 1e-5, 1e-1, log=True),
#         "max_depth": trial.suggest_int("max_depth", 2, 10),
#         "n_estimators": trial.suggest_int("n_estimators", 100, 1000),
#         "subsample": trial.suggest_float("subsample", 0.5, 1.0),
#         "colsample_bytree": trial.suggest_float("colsample_bytree", 0.5, 1.0),
#         "min_child_weight": trial.suggest_int("min_child_weight", 1, 10),
#         "tree_method": "hist",
#     }

#     model = xgb.train(
#         params,
#         dtrain,
#         num_boost_round=1000,
#         early_stopping_rounds=50,
#         evals=[(dtrain, "train"), (dtest, "test")],
#         verbose_eval=100,
#     )

#     r2 = r2_score(y_test, model.predict(dtest))
#     return r2


# study = optuna.create_study(direction="maximize")
# study.optimize(objective, n_trials=100)

# print()
# print(f"best_params: {study.best_params}")
# print(f"best_value: {study.best_value}")

# params = {
#     "objective": "reg:squarederror",
#     "learning_rate": 0.01,
#     "max_depth": 3,
#     "n_estimators": 300,
#     "subsample": 0.6,
#     "colsample_bytree": 0.7,
#     "min_child_weight": 3,
#     "tree_method": "hist",
# }

best_params = {  # from optuna
    "learning_rate": 0.013393217973010949,
    "max_depth": 2,
    "n_estimators": 687,
    "subsample": 0.5903139448662046,
    "colsample_bytree": 0.6045142333929849,
    "min_child_weight": 6,
}

model = xgb.train(
    best_params,
    dtrain,
    num_boost_round=1000,
    early_stopping_rounds=50,
    evals=[(dtrain, "train"), (dtest, "test")],
    verbose_eval=100,
)

best_score = model.best_score
r2 = r2_score(y_test, model.predict(dtest))
print()
print(f"best_score: {best_score}")
print(f"r2: {r2}")