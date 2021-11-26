-- Source image used for the rotozoom
local img = {}
local angle = 0

function Init()
  img = load_png("lua/2ndreal.png")
  print("width: "..img.width)
  print("height: "..img.height)
  local ncols = #img.palette / 3
  print("colors in palette: "..ncols)

  -- set the palette to that of the image we just loaded
  for i=0,ncols-1 do
    local r = img.palette[i*3 + 1]
    local g = img.palette[i*3 + 2]
    local b = img.palette[i*3 + 3]
    pal(i, r, g, b)
  end
end

function Render(t)
  local c = math.cos(angle * math.pi / 180)
  local s = math.sin(angle * math.pi / 180)
  angle = angle + 1
  for y=0,H-1 do
    for x=0,W-1 do
      local u = bit.band(math.floor((x * c - y * s) * (s + 1)), 0xff)
      local v = math.floor((x * s + y * c) * (s + 1)) % img.height
      while (v < 0) do
        v = v + img.height
      end
      -- Assumes the width of the texture is 256
      pset(x, y, img.data[u + bit.lshift(v, 8) + 1])
    end
  end
end
