const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const webpack = require('webpack');

const paths = {
  src: path.resolve(__dirname, 'src'),
  entryFile: path.resolve(__dirname, 'index.js'),
  dist: path.resolve(__dirname, 'dist'),
  sudoku: path.resolve(__dirname, 'sudoku', 'target', 'wasm32-unknown-unknown', 'release'),
  rust: path.resolve(__dirname, 'rust'),
};

module.exports = {
  /** webpack dev server settings **/
  mode: 'development',
  devtool: 'inline-source-map',
  devServer: {
    contentBase: __dirname,
    hot: true,
    port: 10001,
    open: true, // will open on browser after started
  },
  /**********************************/

  entry: paths.entryFile,
  output: {
    path: paths.dist,
    filename: 'main.js'
  },
  resolve: {
    alias: {
      sudoku: paths.sudoku,
    }
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: [{
          loader: 'babel-loader',
          options: {
            cacheDirectory: true,
          }
        }],
      },
    ],
  },
  plugins: [
    new CleanWebpackPlugin(['dist']),
    new HtmlWebpackPlugin({
      title: 'WebAssembly Hello World'
    }),
    new webpack.NamedModulesPlugin(),
    new webpack.HotModuleReplacementPlugin(),
  ],
};