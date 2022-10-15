#pragma once

#include "../sfml/sfml.hpp"
#include <cmath>
#include <iostream>
using namespace SFML;

class HitCircle : public Sprite {
    public:
        HitCircle();
};

class Circle {
    public:
        Circle(unsigned int x, unsigned int y);

        HitCircle *hitCircle;
        sf::Vector2u getPosition();

        bool active = true;
        unsigned short opacity = 0;
};