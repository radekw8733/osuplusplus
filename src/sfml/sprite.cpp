#include "sprite.hpp"
#include "../game.hpp"
#include <cmrc/cmrc.hpp>

namespace SFML {

Sprite::Sprite(Framework* framework, const char* filename) {
    this->framework = framework;
    
    cmrc::file file = Game::fileLoader.loadResFile(filename);
    texture.loadFromMemory(file.begin(), file.size());
    sprite.setTexture(texture);
}

void Sprite::setPosition(unsigned int x, unsigned int y) {
    sprite.setPosition(x,y);
    position = sf::Vector2u(x,y);
}

void Sprite::setScale(float x, float y) {
    sprite.setScale(x, y);
}

void Sprite::onMouseClick() {}

void Sprite::start() {}

void Sprite::update(float delta) {
    framework->window->drawSprite(this);
}

}