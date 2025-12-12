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
      "w": 60,
      "h": 15
    }
  ]
}`,
    ],
  ),
)

engine.join(new napi.JoinProps('EtherCD', 0))
engine.join(new napi.JoinProps('EtherCD', 2))

engine.update()

const pack = JSON.parse(engine.getPackagePerPlayer(2))

console.log(util.inspect(pack, false, null, true))
