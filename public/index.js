/**
 * @param {string} term 
 * @param {Element} [source=document] 
 * @returns {Element[] | null}
 */
function searchElement(term, source = document) {
	if (!term || !source) {
		return null
	}

	const elements = (() => {
		if (term[0] === "#") {
			const element = source.getElementById(term.slice(1));
			return element ? [element] : [];
		} else if (term[0] === ".") {
			return [...source.getElementsByClassName(term.slice(1))];
		} else {
			return [...source.getElementsByTagName(term)];
		}
	})()

	return elements;
}

const fetchCache = {}

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

