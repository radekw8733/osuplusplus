#include "sprite.hpp"
#include <cmrc/cmrc.hpp>

CMRC_DECLARE(resources);

namespace SFML {

Sprite::Sprite(Framework* framework, const char* filename) {
    this->framework = framework;
    
    cmrc::file file = cmrc::resources::get_filesystem().open("res/background.jpg");
    texture.loadFromMemory(file.begin(), file.size());
    sprite.setTexture(texture);
}

void Sprite::setPosition(unsigned int x, unsigned int y) {

}

void Sprite::setScale(float x, float y) {
    sprite.setScale(x, y);
}

void Sprite::start() {
}

void Sprite::update() {
    framework->window->drawSprite(this);
}

}