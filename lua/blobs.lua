local blobs = {}

function Init()
  -- Initialize palette
  for i=0,255 do
    pal(i, i/3, i/3, i)
  end

  -- Create blobs
  for i=1,5 do
    blobs[i] = {
      scalex = math.random() * 0.6,
      scaley = math.random() * 0.6,
      speed = math.random() * math.pi * 32 - math.pi * 16,
      x = 0,
      y = 0,
    }
  end
end

function Render(t)
  local time = t / 30000

  local shift = 0
  for _,b in ipairs(blobs) do
    b.x = math.sin((time + shift) * math.pi * b.speed) * W * b.scalex + (W/2)
    b.y = math.cos((time + shift) * math.pi * b.speed) * H * b.scaley + (H/2)
    shift = shift + 0.5
  end

  for y=0,H-1 do
    for x=0,W-1 do
      local dsq = 1
      for _,b in ipairs(blobs) do
        local xsq = (x - b.x) * (x - b.x)
        local ysq = (y - b.y) * (y - b.y)
        dsq = dsq * math.sqrt(xsq + ysq)
      end
      local c = math.max(math.min(math.floor(1024 - (dsq / 3e7)), 255), 0)

      pset(x, y, c)
    end
  end
end

