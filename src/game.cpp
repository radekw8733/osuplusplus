#include "game.hpp"
using namespace SFML;

Game::Game() {}

bool Game::isRunning() {
    return framework.isRunning();
}

void Game::initialize() {
    setupWindow();
    loadObjects();

    eventThread = std::thread(&Game::processInput, this);
}

void Game::setupWindow() {
    framework.window->setupWindow("osu++",1920,1080);
    framework.window->setFrameRateLimit(60);
}

void Game::loadObjects() {
    background = new Background(&framework);
    Circle *circle = new Circle(&framework, 50, 100);

    framework.objects.push_back(std::unique_ptr<Node>(background));
    framework.objects.push_back(std::unique_ptr<Node>(circle));
    for (std::unique_ptr<Node> &object : framework.objects) {
        object->start();
    }
}

void Game::processInput() {
    event = framework.window->getEvent();
    while (isRunning()) {
        while (framework.window->pollEvent(event)) {
            switch (event->eventType) {
                case Window::Event::EventType::WindowClosed:
                    framework.shutdown();
                    break;
                case Window::Event::EventType::WindowResized:
                    framework.window->setWindowSize(event->windowSize.width, event->windowSize.height);
                    break;
                case Window::Event::EventType::MouseClicked:
                    for (std::unique_ptr<Node> &object : framework.objects) {
                        int xMouse = framework.window->getMousePosition().x;
                        int yMouse = framework.window->getMousePosition().y;
                        object->onMouseClick(xMouse, yMouse);
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
    framework.window->clearWindow();
    for (std::unique_ptr<Node> &object : framework.objects) {
        object->update();
    }
    framework.window->display();
}