import baseProfile from './base.json';
import {products} from './products.js'

const profile = {
	...baseProfile,
	locale: 'de-LU',
	timezone: 'Europe/Luxembourg',

	products: products,

	trip: true,
	radar: true,
	reachableFrom: true,

	refreshJourneyUseOutReconL: true,
}

export {
	profile,
}
