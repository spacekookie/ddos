-- example: always return localhost, except when asked for "my.ip", then return own ip

function map(func, array)
  local new_array = {}
  for i,v in ipairs(array) do
    new_array[i] = func(v)
  end
  return new_array
end

function string:split(sep)
   local sep, fields = sep or ":", {}
   local pattern = string.format("([^%s]+)", sep)
   self:gsub(pattern, function(c) fields[#fields+1] = c end)
   return fields
end

function AQuery(record,addr)
  print("Query for:",record)
  print("From:",addr)
  if record=="my.ip" then
  	print("faking...")
  	return unpack(map(tonumber,addr:split('.')))
  end
  return 127,0,0,1
end

function AAAAQuery(record,addr)
  print("Query for:",record)
  print("From:",addr)
  if record == "my.ip" then
  	print("faking...")
  	return 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, unpack(map(tonumber, addr:split('.')))
  end
  return 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
end
