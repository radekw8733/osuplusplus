#pragma once

#include <SFML/Graphics.hpp>
#include "sfml.hpp"
#include "../ecs.hpp"

namespace SFML {

class Framework;
class Sprite : public Node {
    public:
        Sprite(Framework *framework, const char* filename);
        void setPosition(unsigned int x, unsigned int y);
        void setScale(float x, float y);
        
        void start();
        void update();
        
        Framework* framework;
        sf::Texture texture;
        sf::Sprite sprite;
};

}