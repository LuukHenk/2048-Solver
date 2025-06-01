
MY_PATH="`dirname \"$0\"`"

cd $MY_PATH/next_frontend
npm install
npm run build
npm run start

