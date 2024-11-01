default:
    @just --list

build-all:
    cargo b
build-release-all:
    cargo b -r

generate-all:
    just gnostr-component
    just generate-component
    #just generate-wasm-pack
    just generate-simple
    just generate-simple-async
    just generate-cli
    just generate-lib

gnostr-component:
    mkdir -p gnostr-component-generated
    rm -rv gnostr-component-generated
    cargo generate --path ./gnostr-component \
        --name gnostr-component-generated \
        --define project-description="An example generated using the gnostr component template" \
        --define use-gitserver=false
    touch gnostr-component-generated/.gitkeep

generate-wasm-pack:
    mkdir -p wasm-pack-generated
    rm -rv wasm-pack-generated
    cargo generate --path ./wasm-pack \
        --name wasm-pack-generated \
        --define project-description="An example generated using the wasm-pack template" \
        --define use-gitserver=false
    touch wasm-pack-generated/.gitkeep

generate-component:
    mkdir -p component-generated
    rm -rv component-generated
    cargo generate --path ./component \
        --name component-generated \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch component-generated/.gitkeep

generate-simple:
    mkdir -p simple-generated
    rm -rv simple-generated
    cargo generate --path ./simple --name simple-generated
    touch simple-generated/.gitkeep

generate-simple-async:
    mkdir -p simple-async-generated
    rm -rv simple-async-generated
    cargo generate --path ./simple-async --name simple-async-generated
    touch simple-async-generated/.gitkeep

generate-cli:
    mkdir -p cli-generated
    rm -rv cli-generated
    cargo generate --path ./cli --name cli-generated \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch cli-generated/.gitkeep

generate-lib:
    mkdir -p lib-generated
    rm -rv lib-generated
    cargo generate --path ./lib --name lib-generated \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false
    touch lib-generated/.gitkeep

