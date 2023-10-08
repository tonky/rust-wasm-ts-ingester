alias t := test
alias th := test-headless

ls:
  just --list

webpack-watchr:
	# ls index.js src/** |  entr -cr just rebuild-all
	npm run build-watch

pack:
  npm run build

build:
  wasm-pack build --target bundler

test:
  wasm-pack test --firefox

test-headless:
  wasm-pack test --firefox --headless
