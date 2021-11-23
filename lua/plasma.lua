function _Init()
  -- Initialize palette
  for i=0,255 do
    local v = i / 255 * 6 - 3 
    --[[ local r = math.max(math.floor(math.sin(v * math.pi) * 255), 0)
    local b = math.max(math.floor(math.cos(v * math.pi) * 255), 0) ]]
    local r = 255
    local g = math.max(math.floor(math.cos(v * math.pi) * 255), 0)
    local b = math.max(math.floor(math.sin(v * math.pi) * 255), 0)
    pal(i, r, g, b)
  end

  --[[ for i=0,15 do
    for j=0,15 do
      pal(i*16+j, i*16, 0, j*16)
    end
  end ]]
end

function _Update(t)
  local time = t / 500
  for y=0,H-1 do
    local dy = (y / H) - 0.5
    for x=0,W-1 do
      local dx = (x / W) - 0.5
      local v = math.sin(dx * 10 + time)
      local cx = dx + 0.5 * math.sin(time / 5)
      local cy = dy + 0.5 * math.cos(time / 3)
      v = v + math.sin(math.sqrt(50 * (cx * cx + cy * cy) + 1) + time)
      v = v + math.cos(math.sqrt(dx * dx + dy * dy) - time)

      -- Since we've added 3 sinusoids, v should be in the [-3, 3] range.
      -- Map it to a palette index in [0, 255]
      v = math.floor((v / 6.0 + 0.5) * 255)

      pset(x, y, v)
    end
  end
end
