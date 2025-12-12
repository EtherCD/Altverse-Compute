import ge from '.'
import { App } from 'uWebSockets.js'
import { WebSocket } from 'uWebSockets.js'

const engine = new ge.Game(
  new ge.GameProps(
    `
{
    "spawn": {
  "radius": 15,
  "speed": 17,
  "max_speed": 17,
  "regeneration": 1,
  "energy": 20,
  "max_energy": 20,
  "world": "Celestial Canyon",
  "area": 0,

  "sx": -165,
  "sy": 815,
  "ex": -15,
  "ey": 401,

  "died_timer": 2
}
  }
    `,
    [
      `
  {
  "client": {
    "fillStyle": "#b3cde0",
    "strokeStyle": "#011f4b",
    "areaFill": "#011f4b"
  },
  "name": "Celestial Canyon",
  "areas": [
    {
      "enemies": [
        {
          "types": ["flame", "homing_sniper", "sniper"],
          "radius": 15,
          "speed": 5,
          "count": 10
        },
        {
          "types": ["tree"],
          "radius": 30,
          "speed": 0,
          "count": 2,
          "aura": 400
        },
        {
          "types": ["wall"],
          "radius": 30,
          "speed": 5,
          "count": 5
        }
      ],
      "w": 1920,
      "h": 480
    }
  ]
}`,
    ],
  ),
)

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
                input.setShift(true)
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
            // mousePos(ws, data.mousePos!)
            break
          case 'mouseEnable':
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
 ____  _  _   __   ____  ____  ____    ____       
(  __)/ )( \\ / _\\ (    \\(  __)/ ___)  (  _ \\      
 ) _) \\ \\/ //    \\ ) D ( ) _) \\___ \\   )   /      
(____) \\__/ \\_/\\_/(____/(____)(____/  (__\\_)      
  `)

    console.log('Started at 8080')
  })

setInterval(() => {
  for (const index in clientsInput) {
    console.log(clientsInput[index])
    engine.input(Number(index), clientsInput[index])
  }
  const packages = engine.update() as Record<string, Buffer>

  for (const [id, client] of clients) {
    let pkg = packages[id]
    if (pkg != undefined) {
      if (pkg.length != 0) client.send(pkg, true)
    }
  }
}, 1000 / 60)
