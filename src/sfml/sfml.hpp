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
        unsigned int objectsToBeDeleted = 0;

        // Game flow control
        void shutdown();
};

class Color : public sf::Color {
    public:
        Color(unsigned int red, unsigned int green, unsigned int blue, unsigned int alpha = 255) : sf::Color(red,green,blue,alpha) {};
};

}