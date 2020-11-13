const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");

module.exports = {
  devtool: "eval-source-map",
  entry: "./src/index.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.bundle.js",
    publicPath: "/"
  },
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: ["ts-loader"],
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
  plugins: [new HtmlWebpackPlugin({ template: "./src/index.html" })],
  devServer: {
    contentBase: [
        path.resolve(__dirname, './fall'),
        path.resolve(__dirname, './collision')
    ],
    open: true,
    openPage: '',
    compress: true,
    port: 8000,
    proxy: {
        '/fall': {
            target: 'http://localhost:8000/',
            pathRewrite: { '^/fall' : '/' }
        },
        '/examples': {
            target: 'http://localhost:8000/',
            pathRewrite: { '^/collision' : '/' }
        }
    }
}
};
