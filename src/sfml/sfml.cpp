#include "sfml.hpp"
#include <SFML/Graphics.hpp>

namespace SFML {

Sprite* Framework::createSprite(const char* filename) {
    return new Sprite(filename);
}

bool Framework::isRunning() {
    if (gameMode == Gamemode::RUNNING) {
        return true;
    }
    else {
        return false;
    }
}

void Framework::shutdown() {
    window.close();
    gameMode = Gamemode::STOPPED;
}

}