'use strict'

const baseProfile = require('./base.json')

const products = [{
	id: 'bart',
	mode: 'train',
	bitmasks: [128],
	name: 'BART',
	short: 'BART',
	default: true,
}, {
	id: 'regional-train',
	mode: 'train',
	bitmasks: [8],
	name: 'regional trains (Caltrain, Capitol Corridor, ACE)',
	short: 'regional trains',
	default: true,
}, {
	id: 'bus',
	mode: 'bus',
	bitmasks: [32],
	name: 'Bus',
	short: 'Bus',
	default: true,
}, {
	id: 'ferry',
	mode: 'watercraft',
	bitmasks: [64],
	name: 'Ferry',
	short: 'Ferry',
	default: true,
}, {
	id: 'tram',
	mode: 'train',
	bitmasks: [256],
	name: 'Tram',
	short: 'Tram',
	default: true,
}, {
	id: 'cable-car',
	mode: 'train',
	bitmasks: [4],
	name: 'cable car',
	short: 'cable car',
	default: true,
}]

const bartProfile = {
	...baseProfile,

	locale: 'en-US',
	timezone: 'America/Los_Angeles',

	products,

	trip: true,
	radar: true,
	refreshJourney: false, // todo: depends on 8615b85 from #204
	reachableFrom: true,
}

module.exports = bartProfile
