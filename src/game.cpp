#include "game.hpp"
using namespace SFML;

Game::Game() {}

bool Game::isRunning() {
    return framework.isRunning();
}

void Game::initialize() {
    framework.window->setupWindow("osu++",1920,1080);
    loadObjects();
    event = framework.window->getEvent();
}

void Game::loadObjects() {
    background = new Background(&framework);

    objects.push_back((Node*) background);
    for (Node* object : objects) {
        object->start();
    }
    framework.objects = objects;
}

void Game::run() {
    if (framework.window->pollEvent(event)) {
        switch (event->eventType) {
            case Window::Event::EventType::WindowClosed:
                framework.shutdown();
                break;
            case Window::Event::EventType::WindowResized:
                framework.window->setWindowSize(event->windowSize.width, event->windowSize.height);
                break;
            case Window::Event::EventType::MouseClicked:
                for (Node* object : objects) {
                    object->onMouseClick();
                }
                break;
            default:
                break;
        }
    }
    framework.window->clearWindow();
    for (Node* object : objects) {
        object->update();
    }
    framework.window->display();
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
}