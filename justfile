default:
    @just --list

build-all: generate-all
    cargo b
build-all-release: generate-all
    cargo b -r
install-all:
    cargo install --force --path cli
    cargo install --force --path component
    cargo install --force --path term
    cargo install --force --path simple
    cargo install --force --path user-input
    cargo install --force --path lib

generate-all:
    #git stash #--include-untracked -a
    just generate-cli
    just gnostr-component
    just generate-component
    just generate-wasm-pack
    just generate-simple
    just generate-term
    just generate-simple-async
    just generate-lib
    just generate-user-input
    just generate-dumbpipe
    #git stash pop

gnostr-component:
    mkdir -p gnostr-component
    rm -rv gnostr-component
    cargo generate --path ./gnostr-component-template \
        --name gnostr-component \
        --define project-description="An example generated using the gnostr component template" \
        --define use-gitserver=false
    touch gnostr-component/.gitkeep

generate-component:
    mkdir -p component
    rm -rv component
    cargo generate --path ./component-template \
        --name component \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch component/.gitkeep

generate-wasm-pack:
    mkdir -p wasm-pack
    rm -rv wasm-pack
    cargo generate --git https://github.com/rustwasm/wasm-pack-template.git \
        --name wasm-pack \
        --define project-description="An example generated using the wasm-pack template" \
        --define use-gitserver=false
    touch wasm-pack/.gitkeep

generate-term:
    mkdir -p term
    rm -rv term
    cargo generate --path ./term-template --name term
    touch term/.gitkeep

generate-simple:
    mkdir -p simple
    rm -rv simple
    cargo generate --path ./simple-template --name simple
    touch simple/.gitkeep

generate-simple-async:
    mkdir -p simple-async
    rm -rv simple-async
    cargo generate --path ./simple-async-template --name simple-async
    touch simple-async/.gitkeep

generate-cli:
    mkdir -p cli
    rm -rv cli
    cargo generate --path ./cli-template --name cli \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch cli/.gitkeep

generate-lib:
    mkdir -p lib
    rm -rv lib
    cargo generate --path ./lib-template --name lib \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch lib/.gitkeep

generate-user-input:
    mkdir -p user-input
    rm -rv user-input
    cargo generate --path ./user-input-template --name user-input
    touch user-input/.gitkeep

generate-dumbpipe:
    mkdir -p dumbpipe
    rm -rv dumbpipe
    cargo generate --path ./dumbpipe-template --name dumbpipe
    touch dumbpipe/.gitkeep
