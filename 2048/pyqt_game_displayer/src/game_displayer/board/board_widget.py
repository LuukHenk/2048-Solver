"""The board widget"""

from typing import List
from PySide6.QtWidgets import (
    QGridLayout,
    QWidget,
)
from PySide6.QtCore import Qt, Signal, Slot
from game_displayer.board.tile_widget import (
    TileWidget, 
    TILE_SIZE,
)

BoardType = List[List[str]]

class BoardWidget(QWidget):
    """The board widget"""
    boardUpdate = Signal(BoardType)
    
    def __init__(self, board_size: int) -> None:
        super().__init__()
        self.__tiles: List[TileWidget] = self.__create_tiles(board_size)
        self.boardUpdate.connect(self.on_update_board)

    @Slot(object)
    def on_update_board(self, board: BoardType) -> None:
        tile_id = 0
        for row in board:
            for column in row:
                self.__tiles[tile_id].style_tile_based_on_text(column)
                tile_id += 1
                
        
    def __create_tiles(self, board_size: int) -> List[TileWidget]:
        tiles: List[TileWidget] = []
        layout = QGridLayout(self)
        for row_id in range(board_size):
            for column_id in range(board_size):
                tile = TileWidget()
                layout.addWidget(tile, row_id, column_id, Qt.AlignCenter)
                tiles.append(tile)
        max_width = layout.columnCount() * TILE_SIZE
        max_height = layout.rowCount() * TILE_SIZE
        self.setFixedSize(max_width, max_height)
        return tiles