local carsJson = LoadResourceFile('carexporter', 'exporter/data.json')
local carData = json.decode(carsJson)
local hashs = {}

exports('carLabelHashs', function()
	return hashs
end)

Citizen.CreateThread(function()
	for i = 1, #carData do
		local data = carData[i]
		local model = GetHashKey(data.modelName)
		local name = data.gameName
		AddTextEntryByHash(model, name)
		if IsModelAVehicle(model) then
			hashs[model] = name
		end
	end
end)