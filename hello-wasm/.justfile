webpack-watchr:
	# ls index.js src/** |  entr -cr just rebuild-all
	npm run build-watch

pack:
  npm run build

build:
  wasm-pack build --target bundler
