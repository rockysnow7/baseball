# baseball

this is a project to predict baseball game results.
my goal is to predict the total number of runs scored by each team in any given game.

## data

all data is gathered from the mlb api.
currently, i am working with a dataset of team-level data of most games from 2016-2021 (inclusive).

## usage

* **data gathering**: `cargo run <season>` will gather data on all games by all teams in the given season. the data will be saved in `data/<season>/<home team id>`. the data of each game will be saved in a separate file within that directory, named `<date>.json`, where `<date>` is the date of the game. this will also save the progress of the data gathering in `data/progress.json`, so that the program can be stopped and restarted without losing progress. this file should not be edited manually.
* **data processing**: `python training/convert_data.py` will gather all the data from the `data` directory and save it in a single file, `data/data.csv`, which will be used for training the model.
* **model training**: `python training/train.py` will train the model on the data in `data/data.csv`.

## results so far

so far, i have trained an XGBoost model on the 2016-2021 data, with hyperparameters tuned using optuna.
this model gets RMSE=~3.0, R^2=~0.1, and MAE=2.37 on the test set. this is not great, so i am going to try some other models to see if i can get better results.

## notes

* having experimented with various amounts of data, it seems that a larger dataset does not correlate with better results. this implies either that the features of the dataset are not very predictive of the target, or that the model is not able to learn the patterns in the data. i sincerely hope it is the latter, as data gathering is soooo hard.