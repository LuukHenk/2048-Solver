"""The board widget"""

from typing import List
from PySide6.QtWidgets import (
    QGridLayout,
    QWidget,
)
from PySide6.QtCore import Qt
from game_displayer.board.tile_widget import (
    TileWidget, 
    TILE_SIZE,
)

BoardType = List[List[str]]

class BoardWidget(QWidget):
    """The board widget"""

    def __init__(self, board: BoardType) -> None:
        super().__init__()
        self.__tiles: List[TileWidget] = self.__create_tiles(board)

    def update_board(self, board: BoardType) -> None:
        self.__validate_board(board)
        tile_id = 0
        for row in board:
            for column in row:
                self.__tiles[tile_id].style_tile_based_on_text(column)
                tile_id += 1
                
        
    def __create_tiles(self, board: BoardType) -> List[TileWidget]:
        tiles: List[TileWidget] = []
        layout = QGridLayout(self)
        for row_id, row in enumerate(board):
            for column_id, column in enumerate(row):
                tile = TileWidget()
                layout.addWidget(tile, row_id, column_id, Qt.AlignCenter)
                tile.style_tile_based_on_text(column)
                tiles.append(tile)
        max_width = layout.columnCount() * TILE_SIZE
        max_height = layout.rowCount() * TILE_SIZE
        self.setFixedSize(max_width, max_height)
        return tiles

    
    def __validate_board(self, board: BoardType) -> None:
        board_size = sum(len(row) for row in board)
        if board_size != len(self.__tiles):
            raise ValueError("New board mismatches initial board")
    