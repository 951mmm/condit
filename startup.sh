cd back
cargo build -F production --release
cd ..
cd $PWD/front
yarn build:dev
cd ..
port=$(cat $PWD/back/.env | awk '{FS="="} $1=="SERVER_PORT" {print $2}')
cd $PWD/back/target/release
echo "server start listening at $(echo -e "\033[46mhttp://localhost:$port\033[0m")"
./back
