// @ts-check

const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const OptimizeCSSAssetsPlugin = require("optimize-css-assets-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = (env, options) => ({
  entry: "./src/index.js",
  output: {
    path: dist,
    filename: "bundle.js"
  },
  devServer: {
    contentBase: dist
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader"]
      }
    ]
  },
  plugins: [
    new MiniCssExtractPlugin(),

    new HtmlWebpackPlugin({
      template: "index.html",
      favicon: "./src/favicon.ico",
      inject: false,
      minify:
        options.mode !== "production"
          ? false
          : {
              collapseWhitespace: true,
              removeComments: true,
              removeRedundantAttributes: true,
              removeScriptTypeAttributes: true,
              removeStyleLinkTypeAttributes: true,
              useShortDoctype: true
            }
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "crate")
    })
  ],
  optimization: {
    minimizer: [new TerserPlugin(), new OptimizeCSSAssetsPlugin()]
  }
});
