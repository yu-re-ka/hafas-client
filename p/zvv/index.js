import baseProfile from './base.json' with { type: 'json' };
import {products} from './products.js';

const profile = {
	...baseProfile,
	locale: 'de-CH',
	timezone: 'Europe/Zurich',

	products,

	trip: true,
	radar: true,
	refreshJourneyUseOutReconL: true,
	reachableFrom: true,
};

export {
	profile,
};
