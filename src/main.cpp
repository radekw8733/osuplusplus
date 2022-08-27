#include "game.hpp"

int main(int argc, char* args[]) {
    Game osuGame;
    osuGame.initialize();
    while (osuGame.isRunning()) {
        osuGame.run();
    }
    return 0;
}