#include "sfml.hpp"
#include <SFML/Graphics.hpp>

SFMLRenderer::SFMLRenderer() {

}

void SFMLRenderer::setupWindow(const char* windowTitle, unsigned int windowWidth, unsigned int windowHeight) {
    window = new sf::RenderWindow(sf::VideoMode(200,200), "asd");
}