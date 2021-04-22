const addon = require('../native/index.node');

console.log(addon.screenSizeGet())

addon.windowFocus("Calculator");

const size = addon.windowSizeGet("Calculator")
const pos = addon.windowPositionGet("Calculator")

console.log({...size, ...pos})

console.log(addon.pixelColorGet({x: 1842, y: 838}))

module.exports = addon;
