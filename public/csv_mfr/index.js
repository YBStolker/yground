async function add_stage_main() {
	const stage_type = document.querySelector("#select_stage_type_main")?.value

	if (!stage_type) {
		console.error("Invalid stage type")
		return
	}

	await add_stage(null, stage_type)
}

/**
 * @param {number} index 
 * @param {string} stage_type 
 */
async function add_stage(index, stage_type) {
	const stage_element = await create_stage_element(stage_type)

	const pipeline = document.querySelector("#pipeline")

	if (!pipeline) {
		console.error("Could not find pipeline")
		return
	}

	if ((!index && index !== 0) || !pipeline.childNodes[index]) {
		pipeline.prepend(stage_element)
	} else {
		pipeline.children[index].after(stage_element)
	}

	update_pipeline_indices()
}

/**
 * @param {string} stage_type 
 * @returns {Promise<Element | null>}
 */
async function create_stage_element(stage_type) {
	/** @type Element */
	const new_stage = (await fetchTemplate("csv_mfr/stage.html"))

	if (new_stage) {
		const pretty_title = stage_type[0].toUpperCase() + stage_type.slice(1).toLowerCase()
		new_stage.querySelector(".stage_title").innerHTML = pretty_title

		if (!new_stage?.classList?.contains(stage_type)) {
			new_stage.classList.add(stage_type)
		}
	}

	return new_stage
}

/**
 * @param {number} index 
 */
function delete_stage(index) {
	const pipeline = document.querySelector("#pipeline")
	pipeline.children[index].remove()
	update_pipeline_indices()
	update_output()
}

function update_pipeline_indices() {
	const pipeline = document.querySelector("#pipeline")

	pipeline.querySelectorAll(".stage_section").forEach((section, i) => {
		const select = section.querySelector(".stage_type_select")
		const add_button = section.querySelector(".add_button")
		const delete_button = section.querySelector(".delete_button")

		if (!select || !add_button || !delete_button) {
			console.error("Invalid stage section found.", section)
			return
		}

		if (select && add_button) {
			add_button.onclick = () => add_stage(i, select.value)
		}

		if (delete_button) {
			delete_button.onclick = () => delete_stage(i)
		}
	})
}

/**
 * @param {string} term 
 * @param {(string) => void} callback 
 */
function read_csv_from_input(term, callback) {
	const file_input = document.querySelector(term)
	if (!file_input
		|| !file_input.files[0]
		|| file_input.type !== "file"
		|| file_input.accept !== ".csv"
	) {
		console.log("invalid file_input")
		return null
	}

	const reader = new FileReader()

	reader.onload = load_event => {
		const content = load_event.target.result
		callback(content)
	}

	reader.onerror = err => {
		content = null
		console.error(err)
	}

	reader.readAsText(file_input.files[0])
}


/**
 * Checks if the needle is in the haystack at the index.
 * @param {number} index 
 * @param {string} needle 
 * @param {string} haystack 
 * @returns {boolean}
 */
function is_substring_at(index, needle, haystack) {
	for (let i = 0; i < needle.length; i++) {
		if (needle[i] !== haystack[i + index]) {
			return false
		}
	}
	return true
}


/**
 * @param {string} input_string 
 * @param {string} separator 
 * @param {string} newline 
 * @param {string} quote 
 * @returns {string[][]}
 */
function parse_as_csv(input_string, separator, newline, quote) {
	const table = [[""]]

	let is_quoted = false
	for (let i = 0; i < input_string.length; i++) {
		const i_row = table.length - 1
		const i_value = table[i_row].length - 1

		const is_separator = is_substring_at(i, separator, input_string)
		if (is_separator && !is_quoted) {
			table[i_row].push("")
			i += separator.length - 1
			continue
		}

		const is_quote_start = (() => {
			// The following cases count as a quote start, if it is not currently quoted
			// <separator><quote>
			// <newline><quote>
			// <start of input_string><quote>
			if (is_quoted) { return false }

			if (!is_substring_at(i, quote, input_string)) {
				return false
			}

			if (0 === i) {
				return true
			}

			if (is_substring_at(i - separator.length, separator, input_string)) {
				return true
			}

			return is_substring_at(i - newline.length, newline, input_string)
		})()

		if (is_quote_start) {
			is_quoted = true
			i += quote.length - 1
			continue
		}

		const is_quote_end = (() => {
			// The following cases count as a quote start
			// <quote><separator>
			// <quote><newline>
			// <quote><end of input_string>
			if (!is_quoted) { return false }

			if (!is_substring_at(i, quote, input_string)) {
				return false
			}

			if (i + quote.length === input_string.length) {
				return true
			}

			if (is_substring_at(i + quote.length, separator, input_string)) {
				return true
			}

			return is_substring_at(i + quote.length, newline, input_string)
		})()

		if (is_quote_end) {
			is_quoted = false
			i += quote.length - 1
			continue
		}

		const is_newline = is_substring_at(i, newline, input_string)
		if (is_newline && !is_quoted) {
			table.push([""])
			continue
		}

		table[i_row][i_value] += input_string[i]
	}

	return table
}

function get_parse_parameters() {
	const separator_element = document.querySelector("#main_separator_input")
	const separator_value = separator_element?.value
	if (!separator_value) {
		console.error("Invalid separator")
		// TODO add error element for separator
	}

	const newline_element = document.querySelector("#main_newline_input")
	const newline_value = newline_element?.value
	if (!newline_value) {
		console.error("Invalid newline")
		// TODO add error element for newline
	}

	const quote_element = document.querySelector("#main_quote_input")
	const quote_value = quote_element?.value
	if (!quote_value) {
		console.error("Invalid quote")
		// TODO add error element for quote
	}

	if (!separator_value || !newline_value || !quote_value) {
		return null
	}

	const separator = eval("`" + separator_value + "`")
	const newline = eval("`" + newline_value + "`")
	const quote = eval("`" + quote_value + "`")

	return { separator, newline, quote }
}

let raw_content = null
let result = null

function update_output() {
	read_csv_from_input("#main_file_input", content => {
		raw_content = JSON.stringify(content)

		const { separator, newline, quote } = get_parse_parameters()
		result = parse_as_csv(content, separator, newline, quote)

		document.querySelector("#pipeline")?.querySelectorAll(".stage_body").forEach(stage => {
			const stage_body = stage.value
			if (!Array.isArray(result)) {
				return
			}

			if (stage.id === "map") {
				eval("function map_f (row, i, rows) { " + stage_body + " }")
				result.map(map_f)
			}
			else if (stage.id === "filter") {
				eval("function filter_f (row, i, rows) { " + stage_body + " }")
				result.filter(filter_f)
			}
			else if (stage.id === "reduce") {
				eval("function reduce_f (acc, cur) { " + stage_body + " }")
				result = result.reduce(reduce_f)
			}
		})

		console.info({ raw_content })
		console.info({ result })
	})
}

window.addEventListener("load", _load_event => {
	const file_input = document.querySelector("#main_file_input")

	if (!file_input) {
		console.error("Could not find #main_file_input")
		return
	}

	file_input.addEventListener("change", update_output)
})

