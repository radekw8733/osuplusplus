#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include "base.hpp"
SDL_Window *window = NULL;
SDL_Surface *surface = NULL;
SDL_Renderer *renderer = NULL;
using namespace osuReloaded;
bool gameRunning = true;
GAME_STATES gameState;

void closeApp() {
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
}

int main(int argc, char* args[]) {
    SDL_Init(SDL_INIT_EVERYTHING);
    SDL_DisplayMode screenInfo;
    SDL_GetCurrentDisplayMode(0,&screenInfo);
    window = SDL_CreateWindow("osu!++",SDL_WINDOWPOS_UNDEFINED,SDL_WINDOWPOS_UNDEFINED,screenInfo.w,screenInfo.h,SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE);

    SDL_Event e;
    bool switchedGameState = true;

    while (gameRunning) {  // MAIN LOOP
        while (SDL_PollEvent(&e) != 0) {
            if (e.type == SDL_QUIT || e.type == SDL_WINDOWEVENT_CLOSE) {gameRunning = false;}
        }

        switch (gameState) {
            case START_MENU:
                if (switchedGameState) {
                }
                break;
        }

        SDL_Delay(1);
    }
    closeApp();
    return 0;
}