const path = require("path");

const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: "./js/index.js",
  output: {
    path: dist,
    filename: "bundle.js",
  },
  mode: 'development',
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html',
    }),
    new HtmlWebpackPlugin({
      filename: 'video-call',
      template: 'video-call.html',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '../'),
      // WasmPackPlugin defaults to compiling in "dev" profile.
      // To change that, use `forceMode: 'release'`.
    }),
  ],
  experiments: {
      asyncWebAssembly: true
  }
};
