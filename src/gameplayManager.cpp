#include "gameplayManager.hpp"

GameplayManager::GameplayManager() {

}

void GameplayManager::passRenderWindow(Window *window) {
    circleManager.setRenderWindow(window);
}

void GameplayManager::render(float delta) {
    circleManager.render(delta);
}

void GameplayManager::onClickListener(unsigned int x, unsigned int y) {
    circleManager.onClickListener(x, y);
}