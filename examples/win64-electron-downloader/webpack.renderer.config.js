const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  target: 'web',
  entry: './src/index.js', // Your React app entry point
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'renderer.js',
  },
  resolve: {
    extensions: ['.js', '.jsx'], // Resolve these extensions
    fallback: {
      fs: false, // Disable fs polyfill
      path: require.resolve('path-browserify'), // Use path-browserify for path
      stream: require.resolve('stream-browserify'), // Polyfill for stream
      buffer: require.resolve('buffer/'), // Polyfill for buffer
    },
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/, // Process both .js and .jsx files
        exclude: /node_modules/,
        use: {
          loader: require.resolve('babel-loader'),
        },
      },
      {
        test: /\.css$/, // For CSS files
        use: [
            require.resolve('style-loader'), // Injects styles into the DOM
            require.resolve('css-loader'),   // Translates CSS into CommonJS
        ],
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
        template: path.resolve(__dirname, 'src', 'index.html'), // Your HTML template
        filename: 'index.html',
    }),
],
};