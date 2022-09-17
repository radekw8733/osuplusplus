#include "background.hpp"
#include <iostream>
using namespace SFML;

Background::Background(Framework *framework) : Sprite::Sprite(framework, "res/background.jpg") {
    this->framework = framework;
}

void Background::update(float delta) {
    sf::Vector2u size = sprite.getTexture()->getSize();
    sprite.setScale(
        (float) framework->window->getWindowSize().x / (float) size.x,
        (float) framework->window->getWindowSize().y / (float) size.y);
    Sprite::update(delta);
}