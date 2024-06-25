import logging
import sys
import optuna # type: ignore
import boto3 # type: ignore
import json
from multiprocessing import Pool

def invoke_lambda_function(function_name, payload):
    client = boto3.client('lambda')
    response = client.invoke(
        FunctionName=function_name,
        InvocationType='RequestResponse',
        Payload=json.dumps(payload).encode('utf-8')
    )
    return json.loads(response['Payload'].read().decode('utf-8'))


def invoke_example(x):
    function_name = 'InfraStack-ExampleA925490C-bE0GVfxwVczM'
    payload = {"x": x}
    result = invoke_lambda_function(function_name, payload)
    return result['result']

def objective(trial):
    x = trial.suggest_float('x', -10, 10)
    # return (x - 2) ** 2
    return invoke_example(x)

study_name = "example-study-with-lambda-multiprocessing"
storage_name = "sqlite:///{}.db".format(study_name)

def optimize(_args):
    study = optuna.create_study(study_name=study_name, storage=storage_name, load_if_exists=True)
    study.optimize(objective, n_trials=10)
    print(study.best_params)  # E.g. {'x': 2.002108042}

def main() -> int:
    # https://optuna.readthedocs.io/en/stable/tutorial/20_recipes/001_rdb.html
    optuna.logging.get_logger("optuna").addHandler(logging.StreamHandler(sys.stdout))

    # https://stackoverflow.com/questions/1408356
    with Pool(4) as p:
        p.map_async(optimize, [None for _ in range(10)]).get(99999)
    return 0
