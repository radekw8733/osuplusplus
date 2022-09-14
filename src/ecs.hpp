#pragma once

class Node {
    public:
        virtual void start() = 0;
        virtual void update() = 0;
        virtual void onMouseClick(int xMouse, int yMouse) {};
};