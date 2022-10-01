let fs = require('fs');
let http = require('http');
let path = require('path');

let client_path = path.join(__dirname, '/client/');
let debug_path = __dirname;
let debug_prefix = '/DEBUG';

function starts_with_ignorecase(input, prefix) {
    return input.toLowerCase().startsWith(prefix.toLowerCase());
}
http.createServer(function (req, res) {
    let url = req.url;
    if (url == '/') { url = '/index.html'; }

    if (starts_with_ignorecase(url, debug_prefix + '/')) {
        url = url.substring(debug_prefix.length);
        url = path.join(debug_path, url);
    }
    else {
        url = path.join(client_path, url);
    }
    
    fs.readFile(url, function (err, data) {
        if (err) {
            res.writeHead(404);
            res.end(JSON.stringify(err));
            return;
        }
        res.writeHead(200, {
            'Content-Type': guess_mime(url),
            'Cache-Control': 'public, no-cache, no-store, max-age=0, must-revalidate',
        });
        res.end(data);
    });
}).listen(8080);

const mimes = {
    'html': 'text/html',
    'htm': 'text/html',
    'js': 'text/javascript',
    'mjs': 'text/javascript',
    'css': 'text/css',
    'wasm': 'application/wasm',
    'png': 'image/png',
    'jpeg': 'image/jpeg',
    'jpg': 'image/jpeg',
    'json': 'application/json',
    'txt': 'text/plain',
    'rs': 'text/plain',
};
function guess_mime(url) {
    let split = url.split('.');
    let ext = split[split.length-1].toLowerCase();
    if (mimes.hasOwnProperty(ext)) {
        return mimes[ext];
    }
    return 'application/octet-stream';
}