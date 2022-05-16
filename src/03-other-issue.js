'use strict'

const fs = require('fs')

const read_options = { encoding: 'utf8' }
const content = fs.readFileSync('./package.json', read_options)

// What the following lines will log ?
console.log('File read: ', content.length, 'size')
console.log('File read: ', read_options, 'options')

// Can We said *always* this works as expected ?
// *Always* ?
