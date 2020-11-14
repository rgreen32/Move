const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
let entrypoint = JSON.parse(process.env.npm_config_argv)["cooked"].pop()
module.exports = {
  devtool: "eval-source-map",
  entry: [`./${entrypoint}/index.ts`],
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.bundle.js",
    publicPath: "/"
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
      },
      {
        test: /\.html$/,
        loader: 'html-loader',
        exclude: /node_modules/
      },
      {
        test: /\.(png|svg|jpg|gif)$/,
        use: ['file-loader'],
        exclude: /node_modules/
      }
    ],
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".html", ".jpg"],
  },
  plugins: [new HtmlWebpackPlugin({ template: "./index.html" })],
  devServer: {
    contentBase: [
        path.resolve(__dirname, `./${entrypoint}`)
    ]
}
};
