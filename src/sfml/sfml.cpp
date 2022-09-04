#include "sfml.hpp"

namespace SFML {

Framework::Framework() {
    window = new Window;
}

Sprite* Framework::createSprite(const char* filename) {
    return new Sprite(this, filename);
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
    window->close();
    gameMode = Gamemode::STOPPED;
}

}