#pragma once

#include <vector>
#include <mutex>
#include "circle.hpp"

class CircleManager { 
    public:
        CircleManager();

        void newCircle(unsigned int x, unsigned int y);
        void render(float delta, SFML::Window *windowToRender);
        void onClickListener(unsigned int xMouse, unsigned int yMouse);
    
    private:
        std::vector<std::unique_ptr<Circle>> circles;
        std::mutex circlesLock;
};