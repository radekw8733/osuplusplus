#include "game.hpp"
#include "sfml.hpp"

Game::Game() {
    renderer = new SFMLRenderer();
}

void Game::initialize() {
    renderer->setupWindow("osu++",200,200);
}

void Game::run() {

}