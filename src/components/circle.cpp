#include "circle.hpp"

HitCircle::HitCircle() : Sprite::Sprite(framework, "res/hitcircle.png") {}

Circle::Circle(unsigned int x, unsigned int y) {
    hitCircle = new HitCircle();
    hitCircle->setPosition(x, y);
    hitCircle->sprite.setColor(Color(255,255,255,0));
}

sf::Vector2u Circle::getPosition() {
    return hitCircle->position;
}