import baseProfile from './base.json' with { type: 'json' };
import {products} from './products.js';

const profile = {
	...baseProfile,
	locale: 'de-DE',
	timezone: 'Europe/Berlin',

	products,

	refreshJourneyUseOutReconL: true,
	trip: true,
	radar: true,
	refreshJourney: true,
	reachableFrom: true,
};

export {
	profile,
};
