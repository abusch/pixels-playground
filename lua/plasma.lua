
function _Update()
  local time = time / 5
  for y=0,H-1 do
    local dy = (y / H) - 0.5
    for x=0,W-1 do
      local dx = (x / W) - 0.5
      local v = math.sin(dx * 10 + time)
      local cx = dx + 0.5 * math.sin(time / 5)
      local cy = dy + 0.5 * math.cos(time / 3)
      v = v + math.sin(math.sqrt(50 * (cx * cx + cy * cy) + 1 + time))
      v = v + math.cos(math.sqrt(dx * dx + dy * dy) - time)

      local r = math.abs(math.floor(math.sin(v * math.pi) * 15))
      local b = math.abs(math.floor(math.cos(v * math.pi) * 15))

      screen[x+320*y]=r*16+b
    end
  end
  --[[ local t = screen:t() / 5
  cls(t % 255) ]]
end
