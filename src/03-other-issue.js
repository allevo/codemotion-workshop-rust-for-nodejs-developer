// Our code surely relies on external dependency:
// - web framework (express.js, fastify, etc...) 
// - database access (mongodb, mysql, elastic etc...)
// - nodejs core module (FS, net etc...)
// - log library (pino, etc...)
// So, in our code, we call external function.
// The following code could be an example of
// reading (in a sync way) a file and logs something to output
'use strict'

const fs = require('fs')

const read_options = { encoding: 'utf8' }
const content = fs.readFileSync('./package.json', read_options)

console.log('File read: ', content.length, 'size')
console.log('File read: ', read_options, 'options')

// Nothing strange right?
// Well...
// Node.js core library is well written, so here there's no issue
// so, logging *read_options* after the read it is not a problem
// *BUT*
// Can We said *always* this works as expected ?
// *Always* ?
