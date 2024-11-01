default:
    @just --list

build-all:generate-all
    cargo b
build-release-all:generate-all
    cargo b -r

generate-all:
    just gnostr-component
    just generate-component
    just generate-wasm-pack
    just generate-simple
    just generate-simple-async

gnostr-component:
    rm -rv gnostr-component-generated
    cargo generate --path ./gnostr-component \
        --name gnostr-component-generated \
        --define project-description="An example generated using the gnostr component template" \
        --define use-gitserver=false
    touch gnostr-component-generated/.gitkeep

generate-wasm-pack:
    rm -rv wasm-pack-generated
    cargo generate --git https://github.com/rustwasm/wasm-pack-template.git \
        --name wasm-pack-generated \
        --define project-description="An example generated using the wasm-pack template" \
        --define use-gitserver=false
    touch wasm-pack-generated/.gitkeep

generate-component:
    rm -rv component-generated
    cargo generate --path ./component \
        --name component-generated \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch component-generated/.gitkeep

generate-simple:
    rm -rv simple-generated
    cargo generate --path ./simple --name simple-generated
    touch simple-generated/.gitkeep

generate-simple-async:
    rm -rv simple-async-generated
    cargo generate --path ./simple-async --name simple-async-generated
    touch simple-async-generated/.gitkeep
