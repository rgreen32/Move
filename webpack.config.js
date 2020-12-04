const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
let entrypoint = JSON.parse(process.env.npm_config_argv)["cooked"].pop()
const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: `./${entrypoint}/index.ts`
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: [{
          loader: 'awesome-typescript-loader',
          options: {
              configFileName: `./${entrypoint}/tsconfig.json`
          },
      }],
        exclude: /node_modules/
      }
    ],
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".html", ".jpg"],
  },
  devServer: {
    contentBase: [
        path.resolve(__dirname, `./${entrypoint}`)
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({ template: "./index.html" }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
