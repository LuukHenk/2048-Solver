"""The game board displayer"""

from PySide6 import QtCore, QtWidgets


class Board(QtWidgets.QWidget):  # pylint: disable=R0903
    """The board widge"""

    def __init__(self, board):
        """
        Creates a 2048 board
        Args:
            board (Table[Table[int]]) = The 2048 game board

        Returns (QtWidgets.QWidget) = The game board as qt widget
        """
        super().__init__()
        self.board = QtWidgets.QGridLayout(self)
        self.setStyleSheet("QLabel { background-color : #3C341F;}")
        self.__add_cells(board)

    def __add_cells(self, board):
        for y, row in enumerate(board):  # pylint: disable=invalid-name
            for x, cell in enumerate(row):  # pylint: disable=invalid-name
                cell = self.__create_cell(str(x + y))
                self.board.addWidget(cell, y, x, QtCore.Qt.AlignCenter)

    @staticmethod
    def __create_cell(text: str = ""):
        cell_text = QtWidgets.QLabel(text)
        cell_text.setAlignment(QtCore.Qt.AlignCenter)
        cell_text.setStyleSheet(
            "QLabel { background-color : #808000; color : #AB274F; font-size: 32px;}"
        )
        cell_text.setMargin(50)
        return cell_text
