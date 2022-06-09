"""Game data formats"""
from dataclasses import dataclass
from typing import List


TfeBoard = List[List[int]]


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

    board: TfeBoard
    performed_move: Directions
    score: int


GameResult = List[MoveResult]
