const esbuild = require('esbuild');

esbuild.build({
    entryPoints: ['package.js'],
    bundle: true,
    outfile: 'src/ffi/ffi.js',
    format: 'esm',
    minify: true,
}).catch(err => {
    process.exit(1);
    console.error(err);
});