const path = require('path');

module.exports = {
  target: 'node',
  entry: './public/main.js', // Your main process entry point

  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'main.js',
  },
  module: {
    rules: [
      {
        test: /\.node$/,
        use: 'node-loader',
      },
    ],
  },
  // Additional main process settings...
};