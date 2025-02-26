<h1 align="center">astro-squeeze</h1>
<p align="center">
    <a href="https://github.com/altf02/astro-squeeze/actions"><img alt="GitHub Actions Status" src="https://github.com/altf02/astro-squeeze/workflows/CI/badge.svg" /></a>
    <a href="https://www.npmjs.com/package/astro-squeeze"><img src="https://img.shields.io/npm/v/astro-squeeze" alt="npm version"></a>
</p>

_A high-performance gzip and brotli compressor for Astro written in Rust._

Astro Squeeze compresses your static files ahead of time for optimal performance during asset delivery. 

## Table of Contents

- [Introduction](#introduction)
- [Quickstart](#quickstart)
- [Usage](#usage)
  - [Basic Integration](#basic-integration)
  - [Configuration Options](#configuration-options)
- [Important Notes](#important-notes)
- [License](#license)

## Introduction

Astro Squeeze is designed to smoothly integrate into your Astro project. It compresses your assets using both gzip and brotli. This leads to reduced file sizes and improved page load times without any manual intervention.

## Quickstart

Install Astro Squeeze using your preferred package manager:

```sh
# Using NPM
npx astro add astro-squeeze

# Using Yarn
yarn astro add astro-squeeze

# Using PNPM
pnpm astro add astro-squeeze
```

After installation, simply build your project. Look for the compression messages in your build log:

```sh
pnpm build
```

## Usage

### Basic Integration

1. First, add the package as a development dependency:

   ```sh
   pnpm add --dev astro-squeeze
   ```

2. Then, configure it in your ```astro.config.*``` file by updating the ```integrations``` property:

   ```js
   import { defineConfig } from "astro/config";
   import squeezer from "astro-squeeze";

   export default defineConfig({
     // ...
     integrations: [
       // other integrations,
       squeezer(),
     ],
   });
   ```

### Configuration Options

You can customize Astro Squeeze by passing options to enable or disable specific compression types or limit the file formats:

- **Enable/Disable Specific Compression**

  ```js
  import { defineConfig } from "astro/config";
  import squeezer from "astro-squeeze";

  export default defineConfig({
    // ...
    integrations: [
      // other integrations,
      squeezer({ gzip: true, brotli: false }),
    ],
  });
  ```

- **Custom File Extensions**

  By default, Astro Squeeze compresses files with the following extensions:

  ```
  [".css", ".js", ".html", ".xml", ".cjs", ".mjs", ".svg", ".txt"]
  ```

  To restrict compression to specific file types, set the ```fileExtensions``` property:

  ```js
  import { defineConfig } from "astro/config";
  import squeezer from "astro-squeeze";

  export default defineConfig({
    // ...
    integrations: [
      // other integrations,
      squeezer({
        fileExtensions: [".html"] // only compress HTML files
      }),
    ],
  });
  ```

## Important Notes

1. **Integration Order:** Ensure that Astro Squeeze is the **last integration** in the ```integrations``` array. This guarantees that all generated files are available for compression.
2. **Static Exports:** Astro Squeeze works only with static exports. SSR does not generate compressible assets ahead of time.

## Credits

Special thanks to sondr3's [astro-compressor](https://github.com/sondr3/astro-compressor) for serving as a great source of inspiration.

## License

This project is licensed under the [EUPL-1.2](LICENSE) license.