#pragma once

#include "components/circleManager.hpp"

using namespace SFML;

class GameplayManager {
    public:
        GameplayManager();

        void passRenderWindow(Window *window);

        void render(float delta);
        void onClickListener(unsigned int x, unsigned int y);
    private:
        CircleManager circleManager;
};