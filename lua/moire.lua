function Init()
  -- Initialize palette
  pal(0, 0, 0, 0)
  pal(255, 255, 255, 255)
end

function Render(t)
  local time = t / 1000 

  local cx1 = math.sin(time / 2) * W / 3 + W / 2
  local cy1 = math.sin(time / 4) * H / 3 + H / 2
  local cx2 = math.cos(time / 3) * W / 3 + W / 2
  local cy2 = math.cos(time) * H / 3 + H / 2
  
  for y=0,H-1 do
    local dy = (y - cy1) * (y - cy1)
    local dy2 = (y - cy2) * (y - cy2)
    for x=0,W-1 do
      local dx = (x - cx1) * (x - cx1)
      local dx2 = (x - cx2) * (x - cx2)
      local shade = bit.band(bit.rshift(bit.bxor(math.sqrt(dx + dy), math.sqrt(dx2 + dy2)) , 4), 1) * 255

      pset(x, y, shade)
    end
  end
end
