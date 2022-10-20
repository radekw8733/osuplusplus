#pragma once

#include "sfml/sfml.hpp"
using namespace SFML;
#include "ecs.hpp"
#include "components/background.hpp"
#include "components/circle.hpp"
#include "components/fileLoader.hpp"
#include "gameplayManager.hpp"
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

        inline static FileLoader fileLoader;
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
        GameplayManager manager;

        Window::Event* event;

        std::thread eventThread;
};