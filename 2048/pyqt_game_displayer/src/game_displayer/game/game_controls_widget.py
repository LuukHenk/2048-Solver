
from PySide6.QtWidgets import (
    QHBoxLayout,
    QWidget,
    QPushButton,
)
from PySide6.QtCore import Qt, Signal


class GameControlsWidget(QWidget):
    
    startClicked = Signal()
    
    def __init__(self) -> None:
        super().__init__()
        
    def __create_game_controls(self) -> None:
        layout = QHBoxLayout(self)
        layout.addWidget(self.__create_start_button())
        layout.setAlignment(Qt.AlignCenter)
        
    def __create_start_button(self) -> QPushButton:
        button = QPushButton("Start")
        button.clicked.connect(self.startClicked)
        return button