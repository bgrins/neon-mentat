// This would be really easy if it only needed to work in a standalone build:
// "postinstall": "node ./node_modules/neon-cli/bin/cli.js build",
// Unfortunately if it's installed as a dependency then neon-cli is installed
// as a sibling of neon-mentat so we try to find that if possible.

var fs = require('fs');
var path = require('path');
const spawn = require('child_process').spawn;
let cliPath = 'node_modules/neon-cli/bin/cli.js';

console.log("Attempting to find install script at: " + path.resolve(cliPath));
if (!fs.existsSync(cliPath)) {
    cliPath = '../neon-cli/bin/cli.js';
    console.log("Couldn't find it, now attempting: " + path.resolve(cliPath));
}
const nodeProcess = spawn('node', [cliPath, 'build', '--path', '.']);
nodeProcess.stdout.on('data', (data) => {
    process.stdout.write(data);
});
nodeProcess.stderr.on('data', (data) => {
    process.stderr.write(data);
});
nodeProcess.on('close', (code) => {
    console.log(`child process exited with code ${code}`);
});
