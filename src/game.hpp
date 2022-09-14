#pragma once

#include "sfml/sfml.hpp"
#include "ecs.hpp"
#include "components/background.hpp"
#include "components/circle.hpp"
#include <vector>
#include <thread>
#include <chrono>

class Game {
    public:
        Game();
        void initialize();
        void run();
        bool isRunning();

    private:
        void loadObjects();
        void setupWindow();
        void processInput();

        SFML::Framework framework;

        SFML::Sprite* background;
        SFML::Window::Event* event;

        std::thread eventThread;
};