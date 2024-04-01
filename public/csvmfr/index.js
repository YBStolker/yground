async function add_stage_main() {
	const stage_type = document.getElementById("select_stage_type_main")?.value

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

	const pipeline = document.getElementById("pipeline")

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
 * @returns {Promise<Element>}
 */
async function create_stage_element(stage_type) {
	const new_stage = (await fetchTemplate("csvmfr/stage.html"))

	searchElement(".stage_title", new_stage).forEach(e => {
		e.innerHTML = stage_type[0].toUpperCase() + stage_type.slice(1).toLowerCase()
	})

	return new_stage
}

/**
 * @param {number} index 
 */
function delete_stage(index) {
	const pipeline = searchElement("#pipeline")[0]
	pipeline.children[index].remove()
	update_pipeline_indices()
}

function update_pipeline_indices() {
	const pipeline = searchElement("#pipeline")[0]

	searchElement(".stage_section", pipeline).forEach((section, i) => {
		const select = searchElement(".stage_type_select", section)[0]
		const add_button = searchElement(".add_button", section)[0]
		const delete_button = searchElement(".delete_button", section)[0]

		if (!select || !add_button || !delete_button) {
			console.error("Invalid stage section found.", section)
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
function readCsvContent(term, callback) {
	const file_input = searchElement(term)[0]
	if (!file_input
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
 * @param {string} input_string 
 * @returns {string[][]}
 */
function parse_as_csv(input_string, separator = ";", newline = "\n", quote = "\"") {
	const table = [[""]]

	let is_quoted = false
	for (let i = 0; i < input_string.length; i++) {
		const i_row = table.length - 1
		const i_value = table[i_row].length - 1

		const is_separator = input_string.slice(i, i + separator.length) === separator
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

			const quote_slice = input_string.slice(i, i + quote.length)

			if (quote_slice !== quote) { return false }
			if (0 === i) { return true }

			const separator_slice = input_string.slice(i - separator.length, i)
			const newline_slice = input_string.slice(i - newline.length, i)

			return separator_slice === separator || newline_slice === newline
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

			const quote_slice = input_string.slice(i, i + quote.length)
			if (quote_slice !== quote) { return false }

			if (i + quote.length >= input_string.length) { return true }

			const separator_slice = input_string.slice(i + quote.length, i + quote.length + separator.length)
			const newline_slice = input_string.slice(i + quote.length, i + quote.length + newline.length)

			return separator_slice === separator || newline_slice === newline
		})()

		if (is_quote_end) {
			is_quoted = false
			i += quote.length - 1
			continue
		}

		const is_newline = input_string.slice(i, i + newline.length) === newline
		if (is_newline && !is_quoted) {
			table.push([""])
			continue
		}

		table[i_row][i_value] += input_string[i]
	}

	return table
}

function updateOutput() {
	readCsvContent("#main_file_input", content => {
		const table = parse_as_csv(content)

		const table_element = document.createElement("table")
		for (let i = 0; i < table.length; i++) {
			const row_element = document.createElement("tr")
			table_element.append(row_element)

			for (let j = 0; j < table[i].length; j++) {
				const value_element = document.createElement(i === 0 ? "th" : "td")
				row_element.append(value_element)
				value_element.innerText = table[i][j]
			}
		}
	})
}


window.addEventListener("load", _load_event => {
	const file_input = searchElement("#main_file_input")[0]

	if (!file_input) {
		console.error("Could not find #main_file_input")
		return
	}

	file_input.addEventListener("change", updateOutput)
})



