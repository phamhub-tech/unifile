<div id="app-logo" align="center">
    <br />
    <img src="./app-icon.png" height="101px" />
    <h1>Unifile</h1>
	<h3>A simple file deduplicator</h3>
</div>

<div id="badges" align="center">

[![current release](https://img.shields.io/github/release/phamhub-tech/unifile.svg)](https://github.com/phamhub-tech/unifile/releases)
[![license](https://img.shields.io/github/license/phamhub-tech/unifile.svg)](https://github.com/phamhub-tech/unifile/blob/main/LICENSE)

[![build status](https://img.shields.io/github/actions/workflow/status/phamhub-tech/unifile/dev-release.yml?branch=dev&label=build)](https://github.com/phamhub-tech/unifile/actions/workflows/dev-release.yml?query=branch%3Adev)
</div>


> [!WARNING]
> Unifile is still under **active development** and features are being added regularly. Things may break or change at any time without notice! Keep an eye out for new releases, report bugs and give feedback on new features you'd like to see.

## Features

The app currently supports:
- **Folder Scanning**: Retrieve duplicated files within a folder. Duplication is determined by filename.


## Download / Install

You can download the latest release from [here](https://github.com/phamhub-tech/unifile/releases).

## Build from source

Ensure you have the following dependencies installed:
- [rust](https://www.rust-lang.org/tools/install).
- [node](https://nodejs.org/en/download/package-manager).
- [pnpm](https://pnpm.io/installation).
	
	To install `pnpm` after installing `node` run:
	```sh
	npm i -g pnpm
	```

Install frontend dependencies:

```bash
pnpm install
```

Build the app:

```sh
pnpm tauri build
```

## Contributing

### Runing the app

Clone this repository:

```sh
# ssh (recommended)
git clone git@github.com:phamhub-tech/unifile.git

# http
git clone https://github.com/phamhub-tech/unifile.git
```

Instal the dependencies:

```sh
pnpm install
```

Start the app in develpment (runs on `http://localhost:4000`):

```sh
pnpm tauri dev
```

### Contributing to the project

We welcome contributions!.  Here's how you can help:

- Fork the project.
- Create a branch from the `main` branch:
	```sh
	git checkout -b feat/<feature-name>
	```
- Commit you changes.
- Push the branch.
- Open a Pull Request against the `main` branch.


## Pending features

### Core Features

- [ ] Implement a robust logging system.
- [ ] Detect and report chrashes.
- [ ] Templating project settings based on the type of project selected.

### UI/Ux Features

- [ ] Add custom themes.
- [ ] Implementing a command palette for quick actions.

## Feedback & Support

If you encounter any issues or have suggestions with the app, please [Open an Issue](https://github.com/phamhub-tech/unifile/issues).  We appreciate your feedback
