webpack-watchr:
	# ls index.js src/** |  entr -cr just rebuild-all
	npm run build-watch

webpack:
  npm run build

wasm:
  wasm-pack build --target bundler
