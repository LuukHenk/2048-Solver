"""Game displayer"""

from typing import List
from time import sleep

from PySide6.QtCore import QObject, Signal  # pylint: disable=E0611

from lib.utils.json_file import JsonProcessor
from lib.business_layer.python_game_engine.game_data_formats import (
    GameResult,
    MoveResult,
)


class GameDisplayer(QObject):  # pylint: disable=R0903
    """Loops over the game results and signals each move"""

    updated_move = Signal(MoveResult)

    def display_games(self) -> None:
        """Displays the played games"""
        games = self.__load_games()
        for game in games:
            self.__display_game(game)

    @staticmethod
    def __load_games() -> List[GameResult]:
        """Load games from datalayer"""
        json_processor = JsonProcessor()
        return json_processor.load_json_to_dict()

    def __display_game(self, game: GameResult, sleep_time: int = 1) -> None:
        """Displays a single game"""
        for move in game:
            self.updated_move.emit(move)
            sleep(sleep_time)
