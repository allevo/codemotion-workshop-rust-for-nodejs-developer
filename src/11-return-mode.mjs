'use strict'

// 1
function returnSync() {
    return "my result"
}

// 2
function returnPromise() {
    return Promise.resolve("my result")
}

// 3
async function asyncFunction() {
    return "my result"
}

// 4
function throwErr() {
    throw new Error("the error")
}

// 5
function throwSyncErrWhileYouAreExpectingAPromise() {
    if (true) {
        throw new Error("the error")
    }
    return Promise.resolve("my result")
}

// 6
async function throwAsyncErr() {
    throw new Error("the error")
}

// 7
async function exit() {
    const process = await import('process')
    process.exit(0)
}

console.log('1 - return sync:', returnSync())
console.log('2 - return promise:', returnPromise())
console.log('3 - async function:', await asyncFunction())
try { throwErr() } catch (e) {
    console.log('4 - throw error:', e)
}
try { throwSyncErrWhileYouAreExpectingAPromise() } catch (e) {
    console.log('5 - throw error while you are expecting a promise:', e)
}
try { await throwAsyncErr() } catch (e) {
    console.log('6 - await a rejected promise:', e)
}
console.log('7 - exiting...')
exit()
