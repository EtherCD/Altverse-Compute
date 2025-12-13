import ge from '..'
import { App } from 'uWebSockets.js'
import { WebSocket } from 'uWebSockets.js'
import fs from 'fs'
import path from 'path'

const worldsDir = fs.readdirSync('./test/worlds')
let worlds = []
for (const world in worldsDir) {
  console.log(worldsDir[world])
  try {
    worlds.push(fs.readFileSync('./test/worlds/' + worldsDir[world]) + '')
  } catch {}
}
const config = fs.readFileSync('./test/config.json') + ''

const engine = new ge.Game(new ge.GameProps(config, worlds))

let lastId = 0
let clients = new Map<number, WebSocket<Client>>()
let clientsInput: Record<number, ge.InputProps> = {}
const accessedKeys = ['left', 'right', 'up', 'down', 'shift']

interface Client {
  id: number
  input: ge.InputProps
  packages: object[]
  username: string
}

App()
  .ws<Client>('/server', {
    open: (ws) => {
      const data = ws.getUserData()
      data.id = lastId++
      data.input = new ge.InputProps()
      data.packages = []
      data.username = ''
      clientsInput[lastId - 1] = new ge.InputProps()
      // logger.info(`User connected ${data.id}`)
      clients.set(data.id, ws)
    },
    message: (ws, msg, isBinary) => {
      const data = JSON.parse(Buffer.from(msg).toString())
      let client = ws.getUserData()!

      const keys = Object.keys(data)
      const input = clientsInput[client.id]

      for (const i of keys) {
        switch (i) {
          case 'message':
            break
          case 'keyUp':
            switch (data[i]) {
              case 'down':
                input.setDown(false)
                break
              case 'left':
                input.setLeft(false)
                break
              case 'right':
                input.setRight(false)
                break
              case 'up':
                input.setUp(false)
                break
              case 'shift':
                input.setShift(false)
                break
            }
            break
          case 'keyDown':
            switch (data[i]) {
              case 'down':
                input.setDown(true)
                break
              case 'left':
                input.setLeft(true)
                break
              case 'right':
                input.setRight(true)
                break
              case 'up':
                input.setUp(true)
                break
              case 'shift':
                input.setShift(true)
                break
            }
            break
          case 'init':
            engine.join(new ge.JoinProps('EtherCD', client.id))
            break
          case 'mousePos':
            const pos = data[i]
            input.setMousePosX(pos[0])
            input.setMousePosY(pos[1])
            // mousePos(ws, data.mousePos!)
            break
          case 'mouseEnable':
            input.setMouseEnable(data[i])
            // mouseEnable(ws, data.mouseEnable!)
            break
          case 'ability':
            // ability(ws, data.ability!)
            break
        }
      }
    },
    close: (ws: WebSocket<Client>) => {
      const client = ws.getUserData()
      if (client.id !== undefined) {
        clients.delete(client.id)
        console.info(`Client ${client.id} disconnected`)
        engine.leave(client.id)
      }
    },
  })
  .listen(8080, () => {
    console.info(`
   ___   ____                               
  / _ | / / /__  _____ _______ ___   _______
 / __ |/ / __/ |/ / -_) __(_-</ -_) / __(_-<
/_/ |_/_/\\__/|___/\\__/_/ /___/\\__/ /_/ /___/
                                            
`)

    console.log('Started at 8080')
  })

let timeout: NodeJS.Timeout

const tick = () => {
  clearTimeout(timeout)
  for (const index in clientsInput) {
    engine.input(Number(index), clientsInput[index])
  }
  const packages = engine.update() as Record<string, Buffer>

  for (const [id, client] of clients) {
    let pkg = packages[id]
    if (pkg != undefined) {
      if (pkg.length != 0) client.send(pkg, true)
    }
  }

  timeout = setTimeout(tick, 1000 / 60)
}

tick()
