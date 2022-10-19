#pragma once

#include "sfml/sfml.hpp"
using namespace SFML;
#include "ecs.hpp"
#include "components/background.hpp"
#include "components/circle.hpp"
#include "components/circleManager.hpp"
#include <vector>
#include <thread>
#include <chrono>

class Game {
    public:
        Game();
        void initialize();
        void run();
        void shutdown();
        bool isRunning();

    private:
        void loadObjects();
        void setupWindow();
        void processInput();

        enum Gamemode {
            RUNNING,
            STOPPED
        };
        Gamemode gamemode = Gamemode::RUNNING;

        Framework framework;
        CircleManager manager;

        Window::Event* event;

        std::thread eventThread;
};