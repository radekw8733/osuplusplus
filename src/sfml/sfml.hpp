#pragma once

#include "window.hpp"
#include "sprite.hpp"
#include "../ecs.hpp"
#include <SFML/Graphics.hpp>

namespace SFML {

class Sprite; class Window;
class Framework {
    public:
        Framework();
        
        // Child components
        Window* window;

        // Game flow control
        bool isRunning();
        void shutdown();

        std::vector<Node*> objects;
    private:
        enum Gamemode {
            RUNNING,
            STOPPED
        };
        Gamemode gameMode = Gamemode::RUNNING;
};

}