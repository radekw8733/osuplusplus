#include "circle.hpp"

HitCircle::HitCircle(Framework* framework) : Sprite::Sprite(framework, "res/hitcircle.png") {}

Circle::Circle(Framework *framework, unsigned int x, unsigned int y) {
    hitCircle = new HitCircle(framework);
    hitCircle->setPosition(x, y);

    this->framework = framework;
}

void Circle::start() {
    hitCircle->start();
}

void Circle::update() {
    hitCircle->update();
}

void Circle::onMouseClick(int xMouse, int yMouse) {
    unsigned int radius = hitCircle->sprite.getTexture()->getSize().x / 2 - 10;
    unsigned int x = hitCircle->position.x + radius;
    unsigned int y = hitCircle->position.y + radius;
    unsigned int xDiff = abs(x - xMouse);
    unsigned int yDiff = abs(y - yMouse);
    unsigned int delta = sqrt((xDiff * xDiff) + (yDiff * yDiff)); // calculate length between mouse click and circle center

    if (delta < radius) {
        framework->objects.push_back(std::unique_ptr<Node>(new Circle(framework, rand() % 1000, rand() % 1000))); // create new circle
        std::vector<std::unique_ptr<Node>>::iterator position = 
            std::find_if(framework->objects.begin(), framework->objects.end(), 
                [this](std::unique_ptr<Node> &node) {
                    if (node.get() == this) return true; else return false; // check if address matches this circle
                } 
            );
        framework->objects.erase(position);
    }
}