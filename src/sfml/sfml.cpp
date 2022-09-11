#include "sfml.hpp"

namespace SFML {

Framework::Framework() {
    window = new Window;
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