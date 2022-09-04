#pragma once

#include <SFML/Graphics.hpp>
#include "sprite.hpp"

namespace SFML {

class Sprite;
class Window {
    public:
        // Control functions
        void setupWindow(const char* windowTitle, unsigned int width, unsigned int height);
        void clearWindow();
        void display();
        void close();
        sf::Vector2u getWindowSize();
        void setWindowSize(unsigned int width, unsigned int height);

        // Drawing stuff
        void drawSprite(Sprite* sprite);

        // Events
        struct Event {
            enum EventType {
                WindowClosed,
                WindowResized
            };
            struct WindowResizedEvent {
                unsigned int width;
                unsigned int height;
            };

            EventType eventType;
            union 
            {
                WindowResizedEvent windowSize;
            };
        };
        Event* getEvent();
        bool pollEvent(Event *event);
    private:
        sf::RenderWindow *renderWindow;
};

}