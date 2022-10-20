#include "circleManager.hpp"

CircleManager::CircleManager() {
    
}

void CircleManager::setRenderWindow(SFML::Window *window) {
    windowToRender.reset(window);
}

void CircleManager::newCircle(unsigned int x, unsigned int y) {
    Circle* circle = new Circle(x, y);
    circles.push_back(std::unique_ptr<Circle>(circle));
}

void CircleManager::render(float delta) {
    for (std::unique_ptr<Circle> &circle : circles) {
        if (circle.get() != nullptr) {
            if (circle->active && circle->opacity < 255) {
                circle->hitCircle->sprite.setColor(Color(255,255,255,circle->opacity += 5 + delta));
            }
            else if (!circle->active && circle->opacity > 0 && circle->opacity <= 255) {
                circle->hitCircle->sprite.setColor(Color(255,255,255,circle->opacity -= 5 + delta));
                if (circle->opacity > 255) { // opacity flips to other end of int
                    circles.erase( std::remove(circles.begin(), circles.end(), circle) );
                    continue;
                }
            }
            windowToRender->drawSprite(circle->hitCircle);
        }
    }
}

void CircleManager::onClickListener(unsigned int xMouse, unsigned int yMouse) {
    for (std::unique_ptr<Circle> &circle : circles) {
        if (circle->active) {
            unsigned int radius = circle->hitCircle->sprite.getTexture()->getSize().x / 2;
            unsigned int x = circle->hitCircle->position.x + radius;
            unsigned int y = circle->hitCircle->position.y + radius;
            unsigned int xDiff = abs(x - xMouse);
            unsigned int yDiff = abs(y - yMouse);
            unsigned int delta = sqrt((xDiff * xDiff) + (yDiff * yDiff)); // calculate length between mouse click and circle center

            if (delta < radius) {
                circle->active = false;
                circles.insert(circles.begin(), std::unique_ptr<Circle>(new Circle(rand() % 200, rand() % 200)));
                return;
            }
        }
    }
}