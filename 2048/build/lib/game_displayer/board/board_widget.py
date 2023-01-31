"""The board widget"""

from typing import List, Final
from PySide6.QtWidgets import (
    QGridLayout,
    QWidget,
    QHBoxLayout,
    QLabel,
)
from PySide6.QtCore import Qt, Slot
from game_displayer.board.tile_widget import TileWidget


class BoardWidget(QWidget):
    """The board widget"""

    BoardType = List[List[str]]

    def __init__(self, board: BoardType) -> None:
        super().__init__()
        self.__layout = self.__create_layout(board)

    def __create_layout(self, board: BoardType) -> QGridLayout:
        layout = QGridLayout(self)
        for row_id, row in enumerate(board):
            for column_id, column in enumerate(row):
                layout.addWidget(TileWidget(), row_id, column_id, Qt.AlignCenter)
