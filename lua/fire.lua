local screen = {}

function Init()
  -- generate palette
  for i=0,255 do
    pal(i, bit.rshift(i, 1), bit.rshift(i, 3), bit.rshift(i, 4))
  end

  -- initialise screen buffer
  for ofs=0,W*(H+2) do
    screen[ofs] = 0
  end
end

function Render(t)
  for y=0,H-1 do
    for x=0,W-1 do
      local ofs = x + W * y
      --[[ local v = pget(math.max(x - 1, 0) , y + 1) + pget(x, y + 1) + pget(x, y + 2) + pget(math.min(x + 1, W - 1), y + 2)
      v = math.max(bit.rshift(v, 2) - 2, 0) ]]
      local v = screen[ofs + W - 1] + screen[ofs + W] + screen[ofs + W * 2] + screen[ofs + W * 2 + 1] 
      v = math.max(bit.rshift(v, 2) - 2, 0)
      
      screen[ofs]=v
    end
  end

  for y=H,H+1 do
    for x=0,W-1 do
      local ofs = x + W * y
      local c
      if math.random() > 0.5 then
        c=255
      else
        c=0
      end
      screen[ofs] = c
    end
  end

  -- render
  for y=0,H-1 do
    for x=0,W-1 do
      pset(x, y, screen[x+W*y])
    end
  end
end
