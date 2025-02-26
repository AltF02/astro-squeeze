import { createRequire } from 'module';
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const { runSqueezer } = require("./bindings.cjs");

const defaultFileExtensions = [".css", ".js", ".html", ".xml", ".cjs", ".mjs", ".svg", ".txt"];

const defaultOptions = {
    gzip: true,
    brotli: true,
    fileExtensions: defaultFileExtensions,
    batchSize: 10,
};

export default function squeezer(opts = defaultOptions) {
    const options = { ...defaultOptions, ...opts };

    return {
        name: "astro-squeeze",
        hooks: {
            "astro:build:done": async ({ dir, logger }) => {
                const path = fileURLToPath(dir);
                await runSqueezer(options, path);
                logger.info("Compression finished\n");
            },
        },
    };
}
