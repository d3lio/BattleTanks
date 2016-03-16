DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

GAME_DIR="$DIR/../game"
ENGINE_DIR="$DIR/../engine"

cd $ENGINE_DIR && cargo test
cd $GAME_DIR && cargo test
