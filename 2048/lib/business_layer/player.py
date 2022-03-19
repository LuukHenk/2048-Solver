"""Player object"""

from pathlib import Path
from typing import List
from copy import deepcopy
from random import choice

from python_game_engine.game_data_formats import GameResult, MoveResult
from python_game_engine.game import Game


class Player:
    """Player that can play games"""

    def __init__(
        self,
        results_path: Path,
    ):
        self.__results_path = results_path
        self.__played_games = []

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

    def __load_played_games(self) -> List[GameResult]:
        """Load previous games"""
        if not self.__results_path.exists():
            print(f"Could not find {self.__results_path}, skipping game loading")
            return
        with open(self.__results_path, "r") as game_results:
            # Read the game results
            self.__played_games = game_results
            print(f"Loaded {len(game_results)} games")

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
            MoveResult(board=game.board, performed_move=None, score=game.current_score)
        )
        possible_movements = game.possible_movements()
        while len(possible_movements) > 0:
            move = choice(possible_movements)
            game.perform_movement(move)
            game_results.append(
                MoveResult(
                    board=game.board, performed_move=move, score=game.current_score
                )
            )

            possible_movements = game.possible_movements()
        print(f"{game.current_score}, {game.total_moves_performed}")
        self.__played_games.append(game_results)


PLAYER = Player("")
PLAYER.play_games(1)
print(PLAYER.played_games)
