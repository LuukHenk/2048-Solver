"""Json file - reading and writing"""
from pathlib import Path
from json import dumps, loads

SAVING_PATH = Path(__file__).parent.absolute() / "../../../data/results.json"


class JsonProcessor:
    """Process json data in the file system"""

    def __init__(self, saving_path: Path = SAVING_PATH):
        self.__saving_path = saving_path
        if not self.__saving_path.exists():
            self.__saving_path.touch()

    def save_dict_as_json(self, data: dict):
        """save python dict as json"""

        json_data = dumps(data)
        with open(self.__saving_path, "w") as json_file:
            json_file.write(json_data)

    def load_json_to_dict(self) -> dict:
        """Load python dict from json"""
        with open(self.__saving_path, "r") as json_file:
            json_data = json_file.read()
        return loads(json_data)
