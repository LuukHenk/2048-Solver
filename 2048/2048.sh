MY_PATH="`dirname \"$0\"`"
DATA_PATH="$MY_PATH/data/results.json"

pip install $MY_PATH/pyqt_game_displayer/ && python3 $MY_PATH/pyqt_game_displayer/displayer.py --path $DATA_PATH


