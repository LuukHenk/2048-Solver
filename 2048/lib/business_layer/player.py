"""Player object"""

from pathlib import Path
from typing import List
from copy import deepcopy
from random import choice

from lib.business_layer.python_game_engine.game_data_formats import (
    GameResult,
    MoveResult,
)
from lib.business_layer.python_game_engine.game import Game
from lib.utils.json_file import save_dict_as_json, load_json_to_dict


class Player:
    """Player that can play games, result path should be a directory"""

    def __init__(self, data_layer: Path):
        self.__played_games = []
        self.__results_path = data_layer
        if not self.__results_path.exists():
            self.__results_path.touch()

    @property
    def played_games(self):
        """Shows the played games"""
        return deepcopy(self.__played_games)

    def play_games(self, games_to_play: int, use_of_old_games: bool = False) -> None:
        """Plays a the amount of games as requested.
        Makes use of already played games, if requested
        Args:
            games_to_play (int): the amount of games to be played
            use_of_old_games (bool): If it is accepted to load older games
        """
        if use_of_old_games:
            self.__load_played_games()
        remaining_games = games_to_play - len(self.__played_games)
        self.__play_remaining_games(remaining_games)
        save_dict_as_json(path=self.__results_path, data=self.__played_games)
        print(f"Saved game data to {self.__results_path}")

    def __load_played_games(self) -> List[GameResult]:
        """Load previous games"""
        if not self.__results_path.exists():
            print(f"Could not find {self.__results_path}, skipping game loading")
            return
        self.__played_games = load_json_to_dict(self.__results_path)
        print(f"Loaded {len(self.__played_games)} games")

    def __play_remaining_games(self, games_to_play: int) -> None:
        while len(self.__played_games) < games_to_play:
            print(
                f"Playing remaining games ({len(self.__played_games)}/{games_to_play})"
            )
            self.__play_game()

    def __play_game(self) -> None:
        game = Game()
        game_results = []
        game_results.append(
            MoveResult(
                board=game.board, performed_move=None, score=game.current_score
            ).__dict__
        )
        possible_movements = game.possible_movements()
        while len(possible_movements) > 0:
            move = choice(possible_movements)
            game.perform_movement(move)
            game_results.append(
                MoveResult(
                    board=game.board, performed_move=move, score=game.current_score
                ).__dict__
            )

            possible_movements = game.possible_movements()
        print(f"{game.current_score}, {game.total_moves_performed}")
        self.__played_games.append(game_results)
