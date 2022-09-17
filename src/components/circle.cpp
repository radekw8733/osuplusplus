#include "circle.hpp"

HitCircle::HitCircle(Framework* framework) : Sprite::Sprite(framework, "res/hitcircle.png") {}

Circle::Circle(Framework *framework, unsigned int x, unsigned int y) {
    hitCircle = new HitCircle(framework);
    hitCircle->setPosition(x, y);
    hitCircle->sprite.setColor(Color(255,255,255,0));

    this->framework = framework;
}

void Circle::start() {
    hitCircle->start();
}

void Circle::update(float delta) {
    hitCircle->update(delta);
    if (active && opacity < 255) {
        hitCircle->sprite.setColor(Color(255,255,255,opacity));
        opacity += 5 + delta;
    }
    else if (!active && opacity > 0 && opacity <= 255) { // check if opacity is fading out
        hitCircle->sprite.setColor(Color(255,255,255,opacity));
        opacity -= 5 + delta;
    }
    else if (!active && opacity > 255) { // check if opacity int sign flipped variable
        framework->objectsToBeDeleted++;
        // framework->objects.erase(
        //     std::remove_if(framework->objects.begin(), framework->objects.end(), 
        //         [this](std::unique_ptr<Node> &node) {
        //             return node.get() == this; // check if address matches this circle
        //         }
        //     )
        // );
    }
}

void Circle::onMouseClick(int xMouse, int yMouse) {
    if (active) {
        unsigned int radius = hitCircle->sprite.getTexture()->getSize().x / 2;
        unsigned int x = hitCircle->position.x + radius;
        unsigned int y = hitCircle->position.y + radius;
        unsigned int xDiff = abs(x - xMouse);
        unsigned int yDiff = abs(y - yMouse);
        unsigned int delta = sqrt((xDiff * xDiff) + (yDiff * yDiff)); // calculate length between mouse click and circle center

        if (delta < radius - 20) {
            framework->objects.insert(framework->objects.begin() + 1, std::unique_ptr<Node>(new Circle(framework, rand() % 100, rand() % 100))); // create new circle
            active = false;
        }
    }
    
}

sf::Vector2u Circle::getPosition() {
    return hitCircle->position;
}