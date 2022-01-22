#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include "base.hpp"
SDL_Window *window;
SDL_Surface *surface;
SDL_Renderer *renderer;
using namespace osuReloaded;
bool gameRunning = true;
GAME_STATES gameState;

void closeApp() {
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
}

int main(int argc, char* args[]) {
    SDL_Init(SDL_INIT_VIDEO);
    SDL_DisplayMode screenInfo;
    SDL_GetCurrentDisplayMode(0,&screenInfo);
    window = SDL_CreateWindow("osu!reloaded",SDL_WINDOWPOS_UNDEFINED,SDL_WINDOWPOS_UNDEFINED,screenInfo.w,screenInfo.h,SDL_WINDOW_FULLSCREEN);
    surface = SDL_GetWindowSurface(window);
    renderer = SDL_CreateRenderer(window,-1,SDL_RENDERER_ACCELERATED);

    SDL_Event e;
    SDL_FillRect(surface,NULL,SDL_MapRGB(surface->format,0xFF,0xFF,0xFF));
    SDL_UpdateWindowSurface(window);
    bool switchedGameState = true;

    while (gameRunning) {  // MAIN LOOP
        while (SDL_PollEvent(&e) != 0) {
            if (e.type == SDL_QUIT) {gameRunning = false;}
        }

        switch (gameState) {
            case START_MENU:
                if (switchedGameState) {
                    switchedGameState = false;
                    SDL_Surface *background = IMG_Load("/usr/share/osureloaded/res/background.jpg");
                    SDL_Rect scaleRect;
                    scaleRect.h = screenInfo.h;
                    scaleRect.w = screenInfo.w;
                    SDL_BlitScaled(background,NULL,surface,&scaleRect);
                    SDL_UpdateWindowSurface(window);
                }
                break;
        }
    }
    closeApp();
    return 0;
}