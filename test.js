const napi = require('.')
const util = require('util')

const engine = new napi.Game(
  new napi.GameProps(
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

  "sx": 2,
  "sy": 2,
  "ex": 2,
  "ey": 2,

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

function showPackage(packages) {
  for (pkg in packages) {
    console.log(util.inspect(JSON.parse(packages[pkg]), false, null, true))
  }
}

engine.join(new napi.JoinProps('EtherCD', 0))

let input = new napi.InputProps()
input.down = true
input.left = true
engine.input(0, input)
showPackage(engine.update())

showPackage(engine.update())
