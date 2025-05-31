
MY_PATH="`dirname \"$0\"`"
DATA_PATH="$MY_PATH/data/results.json"

cd $MY_PATH/next_frontend
npm install
npm run build
npm run start

