const fetch_cache = {};

/**
 * @param {string} url 
 * @returns {Promise<string>}
 */
async function cached_fetch(url, type) {
	let result;
	if (fetch_cache[url]) {
		result = fetch_cache[url];
	}
	else {
		result = await fetch(url)
			.then(response => response.text())
			.catch(error => console.error("Could not fetch", url, error));
		fetch_cache[url] = result;
	}

	if (type === "html" && result) {
		result = new DOMParser().parseFromString(result, "text/html").body.firstElementChild;
	}

	return result;
}

function new_instant() {
	return {
		start: new Date(),

		elapsed: function() {
			return new Date() - this.start;
		},
	}
}


