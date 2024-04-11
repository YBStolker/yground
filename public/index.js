

const fetchCache = {}

/**
 * @param {string} url 
 * @returns {Promise<string>}
 */
async function cachedFetch(url) {
	if (fetchCache[url]) {
		return fetchCache[url]
	}

	let result = await fetch(url)
		.then(response => response.text())
		.catch(error => console.error("Could not fetch template", url, error))

	fetchCache[url] = result

	return result
}

/**
 * @param {string} template_id 
 * @returns {Promise<Element | null>}
 */
async function fetchTemplate(template_id) {
	const template_endpoint = (() => {
		if (template_id[0] !== "/") {
			template_id = "/" + template_id
		}

		template_id = "/template" + template_id

		return template_id
	})()


	const html = await cachedFetch(template_endpoint)

	return !html ? null
		: new DOMParser().parseFromString(html, "text/html").body.firstElementChild
}

