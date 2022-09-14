#pragma once

#include "../sfml/sfml.hpp"
#include <cmath>
#include <iostream>
using namespace SFML;

class HitCircle : public Sprite {
    public:
        HitCircle(Framework *framework);
};

class Circle : public Node {
    public:
        Circle(Framework *framework, unsigned int x, unsigned int y );

        void start();
        void update();
        void onMouseClick(int xMouse, int yMouse);

    private:
        HitCircle *hitCircle;
        Framework *framework;
};