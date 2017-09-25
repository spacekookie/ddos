local AQuery
local AAAAQuery
local ddos = {}

local records = 'records.json'
local json = require 'cjson'

AQuery = function(record, addr)
  
end

AAAAQuery = function(record, addr)

end

-- Return both functions to allow easy calling
ddos.AQuery = AQuery
ddos.AAAAQuery = AAAAQuery
return ddos