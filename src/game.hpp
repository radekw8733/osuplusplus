#pragma once

#include "renderer.hpp"

class Game {
    public:
        Game();
        void initialize();
        void run();

    private:
        Renderer *renderer;
};