# Bevy Shell - Template
An opinionated template for Bevy with CI/CD, Native + WASM launchers, free hosting with GitHub Pages and DockerHub, and GitOps release cutting.

## Multi-platform CI/CD
TODO - Talk about how there are github actions for building on all platforms + wasm with GitHub Actions
### Automated dependency management
TODO - Dependencies manager with Renovate
### Automated testing
TODO - Pull requests and pushes to the main branch trigger CI automatically
### Release cutting
TODO - Releases package the game for all 3 major platforms to download (link to releases) and update WASM version automatically through GitHub Pages

## Hosting
### GitHub Pages (free hosting)
TODO show how to setup GitHub Pages (just need to cut a release, go to settings > Pages > enable from the branch `gh-pages`)

## Launcher
### WASM
TODO - Streams assets through HTTP server so initial WASM bundle is smaller. Talk about WASM optimizations made to decrease bundle size Provide loading screen screenshot
### Native
TODO - Provide image of windows, linux (+ mac if possible)
