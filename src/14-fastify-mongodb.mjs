'use strict'

import fastifyFactory from 'fastify'
import fastifyMongodb from '@fastify/mongodb'

const fastify = fastifyFactory({ logger: true })

fastify.register(fastifyMongodb, {
    forceClose: true,
    url: 'mongodb://localhost:27017/mydb'
})

fastify.get('/', async function (request, reply) {
    const employees = this.mongo.db.collection('employees')
    return employees.find().toArray()
})

const start = async () => {
    try {
        await fastify.listen(3000)
    } catch (err) {
        fastify.log.error(err)
        process.exit(1)
    }
}
start()