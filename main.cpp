#include "SDL.h"

int main() {
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window *window = SDL_CreateWindow("osu!reloaded",SDL_WINDOWPOS_UNDEFINED,SDL_WINDOWPOS_UNDEFINED,480,480,0);
    SDL_Delay(5000);
    SDL_Quit();
    return 0;
}