#pragma once

#include "window.hpp"
#include "sprite.hpp"
#include <SFML/Graphics.hpp>

namespace SFML {

class Framework {
    public:
        // Child components
        Window window;

        // Objects creating stuff
        Sprite* createSprite(const char* filename);

        // Game flow control
        bool isRunning();
        void shutdown();

    private:
        enum Gamemode {
            RUNNING,
            STOPPED
        };
        Gamemode gameMode = Gamemode::RUNNING;
};

}