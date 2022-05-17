'use strict'

const fs = require('node:fs')
const chapter = cast(process.argv[2])

const rust_binaries = fs.readdirSync('./src/bin')
const other_binaries = fs.readdirSync('./src')

const rust_binary = rust_binaries.find(name => name.startsWith(chapter))
const other_binary = other_binaries.find(name => name.startsWith(chapter))
if (rust_binary) {
    runRust(rust_binary)
} else {
    if (!other_binary) {
        throw new Error('Unknown chapter: ' + chapter)
    }
    const file_name = './src/' + other_binary
    if (other_binary.endsWith('.md')) {
        const convert = require('echomd').raw
        const content = fs.readFileSync(file_name, { encoding: 'utf-8' });
        console.log(convert(content));
    } else if (other_binary.endsWith('.js') || other_binary.endsWith('.mjs')) {
        import(file_name)
    } else {
        throw new Error('Unknown file extension for file: ' + other_binary)
    }
}


function cast(chapter) {
    if (chapter.length > 1) {
        return chapter
    }
    return '0' + chapter
}

function runRust(chapter_name) {
    const child_process = require('node:child_process')
    child_process.spawn('cargo', ['run', '--bin', chapter_name.split('.')[0]], {
        stdio: ['inherit', 'inherit', 'inherit']
    })
}