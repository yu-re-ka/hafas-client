'use strict'

const {gunzipSync} = require('zlib')

const parseSubscription = (ctx, id, s) => {
	const {profile, opt} = ctx

	const res = {
		id,
		hysteresis: s.hysteresis,
		monitorFlags: s.monitorFlags,
		connectionInfo: s.connectionInfo,
		journeyRefreshToken: s.ctxRecon,
	}

	if (opt.journey && s.data && s.data.slice(0, 5) === 'GZip:') {
		const gzipped = Buffer.from(s.data.slice(5), 'base64')
		const gunzipped = gunzipSync(gzipped)
		const {
			connection,
			// todo: version, search, cst, checksum
			// todo: reminderOrigin, reminderChange, reminderCheckout, reminderDestination
		} = JSON.parse(gunzipped.toString('utf8'))
		res.journey = profile.parseJourney(ctx, connection)
	}

	return res
}

module.exports = parseSubscription
