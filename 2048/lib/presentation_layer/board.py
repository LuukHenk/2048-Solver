"""The game board displayer"""

from PySide6 import QtCore, QtWidgets

from lib.business_layer.python_game_engine.game_data_formats import MoveResult
from lib.presentation_layer.colors import COLORSET_2


class Board(QtWidgets.QWidget):  # pylint: disable=R0903
    """The board widge"""

    def __init__(self, board_size: int = 4) -> None:
        """
        Creates a 2048 board
        Args:
            board (Table[Table[int]]) = The 2048 game board

        Returns (QtWidgets.QWidget) = The game board as qt widget
        """
        super().__init__()
        self.board = QtWidgets.QGridLayout(self)
        self.setStyleSheet("QLabel { background-color : #3C341F;}")
        self.__add_cells(board_size)

    def __add_cells(self, board_size):
        size_range = range(board_size)
        for y in size_range:  # pylint: disable=invalid-name
            for x in size_range:  # pylint: disable=invalid-name
                cell = self.__create_cell("0")
                self.board.addWidget(cell, y, x, QtCore.Qt.AlignCenter)

    @staticmethod
    def __create_cell(text: str = "") -> None:
        cell_text = QtWidgets.QLabel(text)
        cell_text.setAlignment(QtCore.Qt.AlignCenter)
        background_color = COLORSET_2[int(text)]
        cell_text.setStyleSheet(
            "QLabel { background-color : "
            + background_color
            + "; color : #000; font-size: 32px;}"
        )
        cell_text.setMargin(50)
        return cell_text

    @QtCore.Slot(MoveResult)
    def update_cells(self, move_result: MoveResult) -> None:
        for y, row in enumerate(move_result["board"]):
            for x, value in enumerate(row):
                cell = self.board.itemAtPosition(y, x).widget()
                cell.setText(str(value))
                background_color = COLORSET_2[value]
                cell.setStyleSheet(
                    "QLabel { background-color : "
                    + background_color
                    + "; color : #000; font-size: 32px;}"
                )
