"""Game data formats"""
from dataclasses import dataclass
from typing import List

DisplayBoard = List[List[int]]
BoardModel = List[int]


@dataclass
class Directions:
    """ENUM for movements"""

    left: str = "left"
    right: str = "right"
    down: str = "down"
    up: str = "up"  # pylint: disable=C0103


@dataclass
class MoveResult:
    """The results of a single game ordered by index"""

    board: DisplayBoard
    performed_move: Directions
    score: int


@dataclass
class GameResult:
    """The final game result"""

    moves: List[MoveResult]
    score: int
