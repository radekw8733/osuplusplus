#pragma once

#include "sfml/sfml.hpp"

class Game {
    public:
        Game();
        void initialize();
        void run();
        bool isRunning();

    private:
        SFML::Framework framework;

        SFML::Sprite* background;
        SFML::Window::Event* event;
};