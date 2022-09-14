#pragma once

#include "window.hpp"
#include "sprite.hpp"
#include "../ecs.hpp"
#include <memory>
#include <SFML/Graphics.hpp>

namespace SFML {

class Sprite; class Window;
class Framework {
    public:
        Framework();
        
        // Child components
        Window* window;
        std::vector<std::unique_ptr<Node>> objects;

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