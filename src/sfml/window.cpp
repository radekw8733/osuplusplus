#include "window.hpp"

namespace SFML {

void Window::setupWindow(const char* windowTitle, unsigned int width, unsigned int height) {
    renderWindow = new sf::RenderWindow(sf::VideoMode(width,height), windowTitle);
}

void Window::clearWindow() {
    renderWindow->clear();
}

void Window::display() {
    renderWindow->display();
}

void Window::close() {
    renderWindow->close();
}

sf::Vector2u Window::getWindowSize() {
    return renderWindow->getSize();
}

void Window::setWindowSize(unsigned int width, unsigned int height) {
    sf::FloatRect area(0.0f, 0.0f, width, height);
    renderWindow->setView(sf::View(area));
}

void Window::drawSprite(Sprite* sprite) {
    sf::Vector2u size = sprite->texture.getSize();
    sprite->setScale(
        (float) getWindowSize().x / (float) size.x,
        (float) getWindowSize().y / (float) size.y);
    renderWindow->draw(sprite->sprite);
}


Window::Event* Window::getEvent() {
    return new Window::Event;
}

bool Window::pollEvent(Event* event) {
    sf::Event sfmlEvent;
    if (renderWindow->pollEvent(sfmlEvent)) {
        switch (sfmlEvent.type)
        {
        case sf::Event::Resized:
            event->eventType = Window::Event::EventType::WindowResized;
            event->windowSize = {sfmlEvent.size.width, sfmlEvent.size.height};
            return true;
            break;
        case sf::Event::Closed:
            event->eventType = Window::Event::EventType::WindowClosed;
            return true;
            break;
        case sf::Event::MouseButtonPressed:
            event->eventType = Window::Event::EventType::MouseClicked;
            return true;
            break;
        default:
            return false;
            break;
        }
    }
    else {
        return false;
    }
}

}