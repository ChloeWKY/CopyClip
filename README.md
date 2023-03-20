# Copy Clip

A MacOS app used to manage your clipboard history.

## Bonnie

Use Bonnie to manage the repo.

https://github.com/arctic-hen7/bonnie

### Install

```bash
cargo install bonnie
```

### Use

```bash
# Build
bonnie build frontend

# Run
bonnie run frontend
```

### Config

```toml
version="0.3.2"

[scripts]
## Builds Tailwind CSS for development (no purging)
build-tailwind-dev = [
    "tailwindcss -c ./tailwind.config.js -o ./tailwind.css"
]
## Builds Tailwind CSS for production (maximum purging and minification)
build-tailwind-prod = [
    "NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify"
]
## Builds Tailwind CSS for development usage
setup.subcommands.tailwind = "bonnie build-tailwind-dev"
setup.subcommands.prompt-tailwind = "echo \"Have you installed the Tailwind CLI globally with 'npm i -g tailwindcss' or 'yarn global add tailwindcss'?\""
setup.order = """
tailwind {
    Failure => prompt-tailwind
}
"""

## Builds everything
build.cmd = "cargo build"
## Builds the frontend
build.subcommands.frontend = [
    "bonnie build-tailwind-prod",
    "cargo tauri build"
]
## Runs the frontend, watching for changes (uses Trunk)
## Tailwind is assumed to be set up after `setup`
run.subcommands.frontend = [
    "cargo tauri dev"
]
```