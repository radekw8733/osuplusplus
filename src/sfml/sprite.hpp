#pragma once

#include <SFML/Graphics.hpp>

namespace SFML {

class Sprite {
    public:
        Sprite(const char* filename);
        void setPosition(unsigned int x, unsigned int y);
        void setScale(float x, float y);
        
        sf::Texture texture;
        sf::Sprite sprite;
};

}