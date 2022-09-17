#pragma once

#include "../sfml/sfml.hpp"
using namespace SFML;

class Background : public Sprite {
    public:
        Background(Framework *framework);

        void update(float delta);

    private:
        Framework *framework;
};