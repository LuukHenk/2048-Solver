
from PySide6 import QtCore, QtWidgets, QtGui

class Board(QtWidgets.QWidget):
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
        for y, row in enumerate(board):
            for x, cell in enumerate(row):
                cell = self.__create_cell(str(x+y))
                self.board.addWidget(cell, y, x, QtCore.Qt.AlignCenter)

    def __create_cell(self, text:str=""):
        cell_text = QtWidgets.QLabel(text)
        cell_text.setAlignment(QtCore.Qt.AlignCenter)
        cell_text.setStyleSheet("QLabel { background-color : #808000; color : #AB274F; font-size: 32px;}")
        cell_text.setMargin(50)
        return cell_text
