
from pathlib import Path
from json import loads


        
def load_json_to_dict(file_path: Path) -> dict:
    """Load python dict from json"""
    with open(file_path, "r") as json_file:
        json_data = json_file.read()
    return loads(json_data)