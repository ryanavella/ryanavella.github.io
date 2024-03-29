#!/bin/sh

CWD=$(pwd)
mkdir -p docs/blog
mkdir -p docs/mscalc

cd lib/mscalc || exit
wasm-pack -v build --target web
# todo: minify
cp pkg/mscalc.js "$CWD/docs/mscalc/"
cp pkg/mscalc_bg.wasm "$CWD/docs/mscalc/"
cd "$CWD" || exit

cd lib/pwgen || exit
wasm-pack -v build --target web
# todo: minify
cp pkg/pwgen.js "$CWD/docs/pwgen/"
cp pkg/pwgen_bg.wasm "$CWD/docs/pwgen/"
cd "$CWD" || exit

cd blog || exit
mdbook build
cp -R book/ "$CWD/docs/blog/"
cd "$CWD" || exit
