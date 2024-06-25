import logging
import sys
import optuna

def objective(trial):
    x = trial.suggest_float('x', -10, 10)
    return (x - 2) ** 2

def main() -> int:
    # https://optuna.readthedocs.io/en/stable/tutorial/20_recipes/001_rdb.html
    optuna.logging.get_logger("optuna").addHandler(logging.StreamHandler(sys.stdout))
    study_name = "example-study"
    storage_name = "sqlite:///{}.db".format(study_name)
    study = optuna.create_study(study_name=study_name, storage=storage_name, load_if_exists=True)
    study.optimize(objective, n_trials=100)
    print(study.best_params)  # E.g. {'x': 2.002108042}
    return 0
