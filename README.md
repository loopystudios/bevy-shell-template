
<div align="center">

# 🕊️ Bevy Shell - Template
*An opinionated, monolithic template for Bevy with cross-platform CI/CD, native + WASM launchers, and managed cross-platform deployment.*

<img src="https://user-images.githubusercontent.com/20546772/184515793-9f7dea0d-ff21-45ba-9869-49804094e237.png" width="756px" height="600px"/>

</div>

# ✨ Features
- Single, monolithic repository for a cross-platform Bevy App
- Automated CI on pushes and merge requests to `main`
- Cross-platform delivery of your Bevy app through GitHub releases
  - Windows, Linux, MacOS, Web ([Demo](https://kurbos.github.io/bevy-shell-template/))
- Options to deploy Web to DockerHub (self-host) or GitHub Pages (free)
- Settings for automated dependency management through Renovate
- Best practices in GitOps and IaC

# ✅ Quickstart
- [Use](https://github.com/kurbos/bevy-shell-template/generate) or [Fork](https://github.com/kurbos/bevy-shell-template/fork) this template
- [Setup RenovateBot](https://github.com/marketplace/renovate) with permissions to your repository to automatically detect dependency updates
- Setup your WASM build with one of:
  - [GitHub Pages](#github-pages) *(I'm new to hosting, no equipment)*
  - [Docker](#docker) *(I know what I'm doing)*
- **Optional**: Modify your Bevy App, which is self-contained in [`src/`](src/)
  - You can preview your WASM version with `trunk serve` ([Trunk](https://trunkrs.dev) required)
  - You can preview the Native version with `cargo run`
- [Cut a release](#release-cutting) to cross-deploy your game across WASM, Windows, Mac, and Linux

---

# 💁🏻 Information
Below is detailed information for the features of this template

## 📦 Multi-platform CI/CD
This template uses GitOps to deploy a multi-platform release. GitHub Actions powers [testing pipelines](https://github.com/kurbos/bevy-shell-template/actions) and [release deployment](https://github.com/kurbos/bevy-shell-template/releases) to ensure a solid foundation for production release infrastructure.

### Automated testing
Pushing to the main branch automatically triggers CI pipelines:
- Unit Testing - Ubuntu (latest)
- Build - Ubuntu (latest)
- Build - Windows (latest)
- Build - MacOS (latest)
- Build - WebAssembly

🔸 You can ensure all pull requests must pass CI testing before merging through [GitHub's branch protection rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/managing-a-branch-protection-rule).

### Automated dependency management
[Renovate](https://github.com/marketplace/renovate) is a free open source bot on GitHub for your repositories to automatically create pull requests for dependency updates on your projects. This template's [Renovate settings file](.github/renovate.json) allows for automatic merging of minor and patch updates, if all CI tests pass. You can change these settings yourself.

🔸 More information is available at [renovatebot.com](https://renovatebot.com/)

### Release cutting
Creating a release on your template will trigger the release pipeline, which packages [download bundles]((https://github.com/kurbos/bevy-shell-template/releases/latest)) for all 3 major platforms. Additionally, the pipeline will create a branch `gh-pages` with the WASM bundle to serve by GitHub Pages ([demo](https://kurbos.github.io/bevy-shell-template)), or DockerHub image ([example](https://hub.docker.com/repository/docker/simbleau/my_game)), depending on the [hosting strategy](#hosting) you choose to setup.

> 🔥 **WARNING: We enforce releases are tagged with a semantic version name**, e.g. "*v0.1.0*", not "*v1*"
> This can be modified on the [`release-*` workflow files](.github/workflows/).

## 📡 Hosting
There are two ways to host the WASM build of your Bevy game, with Docker or GitHub Pages. You could be creative to adapt this to other hosting platforms, but we will only explain these two. You would likely choose one, not both. If you don't have hosting equipment or know what you're doing, choose GitHub Pages.

### GitHub Pages
To automatically serve your WASM bundle like [our demo](https://kurbos.github.io/bevy-shell-template/), here are the steps:
- Modify the [GitHub Pages GitHub Action file](.github/workflows/release-gh-pages.yml)'s variarable `PUBLIC_URL` with the slug for your GitHub Pages hosting. 
  - If the repo name is the same as the repo owner, this should be `/`, otherwise, it will should be `/<repository-name>/` (e.g. `/bevy-shell-template/`)
- *Optional*: Delete the [DockerHub GitHub Action](.github/workflows/release-dockerhub.yml), as you probably don't need it.
- [Cut a release](#release-cutting) and wait for pipeline completion
- On your GitHub template repo, visit Settings > Pages
- Select `gh-pages` branch from the dropdown menu and press "Save".

  <img src="https://user-images.githubusercontent.com/20546772/184507297-e0f7ff46-57e6-4329-9a79-f2d5ceb5d97a.png" width="600" height="auto"/>

### Docker
To serve your WASM bundle with Docker, here are the steps:
- Signup to DockerHub
- Navigate to your project Settings > Secrets > Actions
- Create two repository GitHub Action secrets:
  - `DOCKERHUB_USERNAME` Your DockerHub username (e.g. *simbleau*)
  - `DOCKERHUB_TOKEN` A [DockerHub access token](https://docs.docker.com/docker-hub/access-tokens/) with write privileges
- Modify the [DockerHub GitHub Action file](.github/workflows/release-dockerhub.yml)'s variable `RELEASE_NAME` with your desired image name (e.g. `my_game`)
- *Optional*: Delete the [GitHub Pages GitHub Action](.github/workflows/release-gh-pages.yml), as you probably don't need it.
- [Cut a release](#release-cutting) and wait for pipeline completion
- On your server hardware with Docker installed, run `docker run -p 80:80 <DOCKERHUB_USERNAME>/<RELEASE_NAME>:latest`, which should pull from DockerHub automatically.
- Your WASM build should be served via `http://localhost:80`, where `localhost` is your server's address

🔸 The [`Dockerfile`](launchers/wasm/Dockerfile) uses NGINX, and uses [`Docker.nginx.conf`](launchers/wasm/Docker.nginx.conf) for configuration.

## 🚀 Launchers
### WASM (Web)
This launcher depends on the [trunk](https://trunkrs.dev/) crate.
To build and run the WASM app locally:
> Serve with `trunk serve` and open [`http://127.0.0.1:8080`](http://127.0.0.1:8080) in your browser
- Assets are streamed through the hosting provider, so that the initial WASM bundle is smaller.
- We use all the WASM optimizations discussed described [here](https://rustwasm.github.io/book/reference/code-size.html) in the Rust and WebAssembly book.
- There is an initial loading screen provided through [Yew](https://yew.rs) while the WASM bundle loads.

### Native (Windows, MacOS, Linux)
> Run with `cargo run`
- Assets are bundled with the release when cut.
- There is no loading screen.
