/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path');
const webpack = require('webpack');
const ForkTsCheckerNotifierWebpackPlugin = require('fork-ts-checker-notifier-webpack-plugin');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    context: process.cwd(), // to automatically find tsconfig.json
    entry: "./src/typescript/entry/index.ts",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js',
        publicPath: "/"
    },
    plugins: [
        new ForkTsCheckerWebpackPlugin({
            eslint: false
        }),
        new ForkTsCheckerNotifierWebpackPlugin({ title: 'TypeScript', excludeWarnings: false }),
        new HtmlWebpackPlugin({
            inject: true,
            template: 'src/typescript/entry/index.html'
        }),
        new webpack.HotModuleReplacementPlugin()
    ],
    module: {
        rules: [
            {
                test: /.ts$/,
                use: [
                    { loader: 'ts-loader', options: { transpileOnly: true } }
                ]
            },
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ]
    },
    resolve: {
        extensions: [".ts", ".js", ".css"],
        alias: {
            "@events": path.resolve(__dirname, "src/typescript/shared/events"),
            "@ui": path.resolve(__dirname, "src/typescript/ui"),
        }
    },
    devtool: 'inline-source-map',
    devServer: {
        contentBase: path.resolve(__dirname, './_static'),
        clientLogLevel: 'warning',
        open: true,
        historyApiFallback: true,
        stats: 'errors-only',
        watchContentBase: true,
    }
};