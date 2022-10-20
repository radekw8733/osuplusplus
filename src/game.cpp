#include "game.hpp"

Game::Game() {}

bool Game::isRunning() {
    if (gamemode == Gamemode::RUNNING) 
        return true;
    else
        return false;
}

void Game::initialize() {
    setupWindow();
    loadObjects();

    eventThread = std::thread(&Game::processInput, this);
}

void Game::setupWindow() {
    framework.window->setupWindow("osu++",1920,1080);
    framework.window->setFrameRateLimit(60);
    event = framework.window->getEvent();
}

void Game::loadObjects() {
    framework.objects.push_back(std::unique_ptr<Node>(new Background(&framework)));

    for (std::unique_ptr<Node> &object : framework.objects) {
        object->start();
    }
}

void Game::processInput() {
    while (isRunning()) {
        while (framework.window->pollEvent(event)) {
            switch (event->eventType) {
                case Window::Event::EventType::WindowClosed:
                    shutdown();
                    break;
                case Window::Event::EventType::WindowResized:
                    framework.window->setWindowSize(event->windowSize.width, event->windowSize.height);
                    break;
                case Window::Event::EventType::MouseClicked:
                    {
                        int x = framework.window->getMousePosition().x;
                        int y = framework.window->getMousePosition().y;
                        for (std::unique_ptr<Node> &object : framework.objects) {
                            object->onMouseClick(x, y);
                        }
                        manager.onClickListener(x, y);
                    }
                    break;
                default:
                    break;
            }
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
}

void Game::run() {
    float delta = 0.0f;
    std::chrono::system_clock::time_point time0;
    std::chrono::system_clock::time_point time1;
    while (isRunning()) {
        time0 = std::chrono::high_resolution_clock::now();

        framework.window->clearWindow();
        for (std::unique_ptr<Node> &object : framework.objects) {
            object->update(delta);
        }
        manager.render(delta);
        framework.window->display();

        time1 = std::chrono::high_resolution_clock::now();
        delta = std::chrono::duration_cast<std::chrono::milliseconds>(time1 - time0).count() / 1000.0f;
    }
    eventThread.join();
}

void Game::shutdown() {
    framework.shutdown();
    gamemode = Gamemode::STOPPED;
}