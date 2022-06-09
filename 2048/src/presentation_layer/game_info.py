""" Displays additional game information such as latest movement and score"""

from PySide6.QtWidgets import (  # pylint: disable=E0611
    QWidget,
    QLabel,
    QHBoxLayout,
)
from PySide6.QtCore import Slot  # pylint: disable=E0611

from src.data_layer.game_data_formats import MoveResult
from src.utils.qt_tools import get_widget_from_layout


class GameInfo(QWidget):  # pylint: disable=R0903
    """Displays additional game information such as latest movement and score"""

    LATEST_MOVEMENT_WIDGET_NAME = "latest_movement"
    SCORE_WIDGET_NAME = "score"

    def __init__(self):
        """Displays additional game information such as latest movement and score"""
        super().__init__()
        self.game_info = QHBoxLayout(self)
        # self.game_info.addWidget(QLabel("WOOOOOOO"))
        self.__add_last_movement_info()
        self.__add_score_info()
        # display last movement
        # Display current score
        # ((display highscore / average score))
        # ((display winrate))

    @Slot(MoveResult)
    def update_info(self, move_result: MoveResult) -> None:
        """Update the game information based on the latest move results
        Arg - move_result (MoveResult): The results of the latest movement
        """
        latest_movement = get_widget_from_layout(
            self.game_info, self.LATEST_MOVEMENT_WIDGET_NAME
        )
        latest_movement.setText(move_result["performed_move"])
        score = get_widget_from_layout(self.game_info, self.SCORE_WIDGET_NAME)
        score.setText(str(move_result["score"]))

    def __add_last_movement_info(self):
        self.game_info.addWidget(QLabel("<b>Latest movement: </b>"))
        latest_movement_widget = QLabel()
        latest_movement_widget.setObjectName(self.LATEST_MOVEMENT_WIDGET_NAME)
        latest_movement_widget.setMargin(10)
        self.game_info.addWidget(latest_movement_widget)

    def __add_score_info(self):
        self.game_info.addWidget(QLabel("<b>Score: </b>"))
        score_widget = QLabel()
        score_widget.setObjectName(self.SCORE_WIDGET_NAME)
        self.game_info.addWidget(score_widget)


# if __name__ == "__main__":
#     import sys
#     from PySide6 import QtWidgets

#     app = QtWidgets.QApplication(sys.argv)
#     window = GameInfo()
#     window.show()

#     sys.exit(app.exec())
