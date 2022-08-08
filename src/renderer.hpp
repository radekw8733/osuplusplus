#pragma once

class Renderer {
    public:
        virtual void setupWindow(const char* windowTitle, unsigned int windowWidth, unsigned int windowHeight) = 0;
};