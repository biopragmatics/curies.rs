#! /usr/bin/env node

// From https://github.com/biopragmatics/curies.rs/blob/main/js/build_package.js
const fs = require("fs");

// We copy file to the new directory
fs.mkdirSync("pkg");
for (const file of fs.readdirSync("./pkg-web")) {
    fs.copyFileSync(`./pkg-web/${file}`, `./pkg/${file}`);
}
for (const file of fs.readdirSync("./pkg-node")) {
    fs.copyFileSync(`./pkg-node/${file}`, `./pkg/${file}`);
}

const pkg = JSON.parse(fs.readFileSync("./pkg/package.json"));
pkg.name = "@biopragmatics/curies";
pkg.main = "node.js";
pkg.browser = "web.js";
pkg.files = ["*.{js,wasm,d.ts}"];
pkg.homepage = "https://github.com/biopragmatics/curies.rs/tree/main/js";
pkg.license = "MIT";
pkg.bugs = {
    url: "https://github.com/biopragmatics/curies.rs/issues",
};
pkg.collaborators = undefined;
pkg.repository = {
    type: "git",
    url: "git+https://github.com/biopragmatics/curies.rs.git",
    directory: "js",
};
fs.writeFileSync("./pkg/package.json", JSON.stringify(pkg, null, 2));
