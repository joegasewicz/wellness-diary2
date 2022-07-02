const path = require("path");


module.exports = {
    entry: "./public/js/index.js",
    output: {
      path: path.resolve(__dirname, "static/js"),
      filename: "main.js",
    },
    module: {
        rules: [
            {
                test: /\.s[ac]ss$/i,
                use: [
                    "style-loader",
                    "css-loader",
                    {
                        loader: "sass-loader",
                        options: {
                            sassOptions: {
                                includePaths: ["./web_client/public/sass"]
                            }
                        }
                    },
                ]
            }

        ],
    }
};
