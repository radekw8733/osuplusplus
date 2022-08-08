#pragma once

#include "renderer.hpp"
#include <SFML/Graphics.hpp>

class SFMLRenderer: public Renderer {
    public:
        SFMLRenderer();
        void setupWindow(const char* windowTitle, unsigned int windowWidth, unsigned int windowHeight);

    private:
        sf::RenderWindow *window;
};