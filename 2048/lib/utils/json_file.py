"""Json file - reading and writing"""
from pathlib import Path
from json import dumps, loads


def save_dict_as_json(path: Path, data: dict):
    """save python dict as json"""
    json_data = dumps(data)
    with open(path, "w") as json_file:
        json_file.write(json_data)


def load_json_to_dict(path: Path) -> dict:
    """Load python dict from json"""
    with open(path, "r") as json_file:
        json_data = json_file.read()
    return loads(json_data)
