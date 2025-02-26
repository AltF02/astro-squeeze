import squeezer from "astro-squeeze";
import { defineConfig } from "astro/config";

export default defineConfig({
    integrations: [squeezer({
        gzip: true
    })],
});
