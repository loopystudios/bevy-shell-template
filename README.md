# Bevy Shell - Template
An opinionated template for Bevy with CI/CD, Native + WASM launchers, free hosting with GitHub Pages and DockerHub, and GitOps release cutting.

## Multi-platform CI/CD
This shell template used the free CI/CD tools provided by GitHub Actions, which can be accessed by navigating to the "Actions" tab of the repo.  This enables automatically building releases for all 3 major platforms: web, Linux, and Windows for every release you cut.

### Automated dependency management
IDK - Dependencies manager with Renovate

### Automated testing
Pull requests and pushes to the main branch trigger CI automatically.

### Release cutting
Releases package the game for all 3 major platforms to [download](https://github.com/kurbos/bevy-shell-template/releases) and update the WASM web page automatically through GitHub Pages.

## Hosting
### GitHub Pages
From your github repo page, go to Settings > Pages > Select `gh-pages` branch from the dropdown menu and press "Save".

![image](https://user-images.githubusercontent.com/20546772/184507297-e0f7ff46-57e6-4329-9a79-f2d5ceb5d97a.png)

## Launcher
### WASM
The launcher streams assets through an HTTP server, so that the initial WASM bundle is smaller. IDK - Talk about WASM optimizations made to decrease bundle size 
Provide loading screen screenshot
![image](https://user-images.githubusercontent.com/20546772/184507403-1ceec682-6a7b-46c8-965e-ff484cc348e0.png)

### Native
TODO - Provide image of windows, linux (+ mac if possible)
![image](https://user-images.githubusercontent.com/20546772/184508108-97b8c832-901a-422f-9ee9-b658d51d513e.png)

![image](https://user-images.githubusercontent.com/20546772/184508242-89aa5b4d-2a2b-47ac-84d2-31580242b80b.png)
