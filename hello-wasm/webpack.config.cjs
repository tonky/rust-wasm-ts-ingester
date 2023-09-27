const path = require('path');
// const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require("copy-webpack-plugin");
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = [
    {
        name: "js_wasm",
        entry: './bootstrap.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: 'wasm_bundle.js',
            // clean: true,
        },
        plugins: [
            // new HtmlWebpackPlugin(),
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, ".")
            }),
            // Have this example work in Edge which doesn't ship `TextEncoder` or
            // `TextDecoder` at this time.
            /*
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder']
            }),
            */
            new CopyWebpackPlugin({ patterns: ['index.html'] })
        ],
        mode: 'development',
        experiments: { asyncWebAssembly: true }
    },
    {
        name: "ts",
        entry: './index.ts',
        devtool: 'inline-source-map',
        /*
        module: {
            rules: [
                {
                    use: 'ts-loader',
                    exclude: /node_modules/,
                },
            ],
        },
        resolve: { extensions: ['.ts'], },
        */
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: 'ts_bundle.js',
            clean: true,
        },
        mode: 'development',
        experiments: { asyncWebAssembly: true },
        // dependencies: ['js_wasm'],
    } //*/
];


