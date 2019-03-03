const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: {
    client: "./client.js"
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js",
  },
  plugins: [
    new CopyPlugin([
      {from: 'index.html', to: 'index.html'},
      {from: 'client.css', to: 'client.css'}
    ])
  ],
  mode: "development"
};
