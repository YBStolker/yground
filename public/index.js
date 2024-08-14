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

class Query {
	constructor(query, source) {
		/** @type Element[] */
		this.result = [...(source ? source : document).querySelector(query)];
	}

	set_attribute(attributeName, attributeValue) {
		for (const element of this.result) {
			element.setAttribute(attributeName, attributeValue);
		}
		return this;
	}

	add_attribute(attributeName, attributeValue) {
		for (const element of this.result) {
			const attribute = element.getAttribute(attributeName);
			element.setAttribute(attributeName, attribute ? attribute + " " + attributeValue : attributeValue);
		}
		return this;
	}
}

/**
* @param {string} query
* @returns {Element | Element[]}
*/
function get_el(query) {
	const result = [...document.querySelectorAll(query)];

	if (result.length < 1) {
		return null;
	}

	if (result.length === 1) {
		return result[0];
	}

	return result;
}

