const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
let entrypoint = JSON.parse(process.env.npm_config_argv)["cooked"].pop()
const dist = path.resolve(__dirname, "dist");

module.exports = {
  context: path.resolve(__dirname, `example`),
  experiments: {
    asyncWebAssembly: true
  },
  mode: "production",
  entry: {
    index: `./index.ts`
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
              configFileName: `./example/tsconfig.json`
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
    writeToDisk: true,
    contentBase: [
        path.resolve(__dirname, `./example`)
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({ template: "./index.html" }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      outDir: `./example/pkg`
    }),
  ]
};
