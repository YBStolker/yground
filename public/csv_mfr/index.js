/**
 * @param {number} index 
 * @param {string} stage_type 
 * @returns {void}
 */
async function add_stage(index) {
	const stage_element = (await cached_fetch("/csv_mfr/get_pipeline_stage", "html"));

	const pipeline = document.querySelector("#pipeline_section");

	if (!pipeline) {
		console.error("Could not find pipeline");
		return;
	}

	if (index > 0) {
		console.log({ stage_element })
		pipeline.children[index - 1].after(stage_element);
	} else {
		pipeline.prepend(stage_element);
	}

	update_pipeline_indices();
}

/**
 * @param {number} index 
 * @returns {void}
 */
function delete_stage(index) {
	const pipeline = document.querySelector("#pipeline_section");
	pipeline.children[index].remove();
	update_pipeline_indices();
	update_output();
}

/**
 * @returns {void}
 */
function update_pipeline_indices() {
	const pipeline = document.querySelector("#pipeline_section");

	pipeline?.querySelectorAll(".stage_section").forEach((section, i) => {
		const add_button = section.querySelector(".add_button");
		const select = section.querySelector(".stage_type_select");
		const delete_button = section.querySelector(".delete_button");

		if (!add_button || !delete_button) {
			console.error("Invalid stage section found.", section);
			return;
		}

		add_button.onclick = () => add_stage(i + 1);
		select.onchange = update_stage_signatures;
		delete_button.onclick = () => delete_stage(i);
	})

	update_stage_signatures()
}

/**
 * @param {string} input_id 
 * @param {(string) => void} callback 
 * @returns {void}
 */
function read_csv_from_input(input_id, callback) {
	const file_input = document.querySelector(input_id);
	if (!file_input
		|| !file_input.files[0]
		|| file_input.type !== "file"
		|| file_input.accept !== ".csv"
	) {
		console.log("invalid file_input");
		return null;
	}

	const reader = new FileReader();

	reader.onload = load_event => {
		const content = load_event.target.result;
		callback(content);
	}

	reader.onerror = err => {
		content = null;
		console.error(err);
	}

	reader.readAsText(file_input.files[0]);
}

/**
 * @param {string} val 
 * @returns {void}
 */
function parse_value_to_js_char(val) {
	const quote_char = !val.includes('"') ? '"' : !val.includes("'") ? "'" : "`";
	return eval(quote_char + val + quote_char);
}


let raw_content = null;
let result = null;

const map_signature = "function do_map (value, index, array) {"
const filter_signature = "function do_filter (value, index, array) {"
const reduce_signature = "function do_reduce (previousValue, currentValue, currentIndex, array) {"

/**
 * @returns {void}
 */
function update_output() {
	read_csv_from_input("#main_file_input", content => {
		raw_content = content;
		const detect_instant = new_instant();
		const params = detect_parse_csv_params(content)
		const { separator, newline, quote } = params;
		console.info(`Detected params in ${detect_instant.elapsed()} ms`, params);

		const parse_instant = new_instant();
		result = parse_as_csv(content, separator, newline, quote);
		console.info(`Parsed csv ${result.length} lines in ${parse_instant.elapsed()} ms`);

		const pipeline_instant = new_instant();
		document.querySelectorAll(".stage_section").forEach(stage => {
			const stage_type = stage.querySelector(".stage_type_select").value;
			console.log({ stage_type });

			const stage_body = stage.querySelector(".stage_body")?.value;
			if (!Array.isArray(result)) {
				console.error("Cannot proces anything other than an array.");
				return;
			}

			console.log({ id: stage.id });

			if (stage_type === "map") {
				eval(map_signature + stage_body + "}");
				result = result.map(do_map);
			}
			else if (stage_type === "filter") {
				console.log({ filter_signature, stage_body });
				eval(filter_signature + stage_body + "}");
				result = result.filter(do_filter);
			}
			else if (stage_type === "reduce") {
				eval(reduce_signature + stage_body + "}");
				result = result.reduce(do_reduce);
			}
		})
		console.info(`Processed pipeline in ${pipeline_instant.elapsed()} ms`);

		console.info("Debug variables available as \"raw_content\" and \"result\"");
	})
}

/**
 * @returns {void}
 */
function update_stage_signatures() {
	const pipeline = document.querySelector("#pipeline_section");

	pipeline?.querySelectorAll(".stage_section").forEach((section) => {
		const select = section.querySelector(".stage_type_select");
		const signature = section.querySelector(".signature_paragraph");
		if (!select || !signature) {
			console.log("update_stage_signatures", { select, signature });
			return;
		}

		signature.innerHTML = {
			"map": map_signature,
			"filter": filter_signature,
			"reduce": reduce_signature,
		}[select.value] || "";
	})
}

/**
 * @returns {{content: string, extension: string}}
 */
function result_to_string() {
	if (!Array.isArray(result)) {
		const content = JSON.stringify(result);
		const extension = ".json";
		return { content, extension };
	}

	// result is probably CSV like
	const extension = ".csv";

	let headers = null;

	/** @type {any[][]} */
	const to_parse = Array.isArray(result[0])
		? result
		: result.map(row => {
			if (!headers) {
				headers = [];
				Object.keys(row).forEach(k => headers.push(k));
			}

			const new_row = [];
			for (let i = 0; i < headers.length; i++) {
				new_row.push(row[headers[i]]);
			}
			return new_row;
		})

	if (headers) {
		to_parse.unshift(headers);
	}

	let content = "";
	const separator = ';';
	const newline = "\n";
	const quote = '"';

	for (let i = 0; i < to_parse.length; i++) {
		const row = to_parse[i];

		for (let j = 0; j < row.length; j++) {
			let value = row[j];
			value = !value ? "" : typeof value === "string" ? value : JSON.stringify(value);

			if (value.includes(separator)
				|| value.includes(newline)
				|| ((value.startsWith(quote) || value.endsWith(quote))
					&& !(value.startsWith(quote) && value.endsWith(quote)))
			) {
				content += quote + value + quote;
			} else {
				content += value;
			}

			content += separator;
		}

		content += newline;
	}

	return { content, extension };
}


/**
 * @returns {void}
 */
function download_result() {
	const main_file_input = get_el("#main_file_input");
	if (!main_file_input) {
		console.error("Could not find main_file_input");
		return;
	}

	/** @type {string} */
	const original_file_name = main_file_input?.files[0]?.name;
	if (!original_file_name) {
		console.error("Could not find original_file_name");
		return;
	}

	const { content, extension } = result_to_string();

	const file_name = original_file_name.split(".").slice(0, -1).join(".") + "_result" + extension;

	download(file_name, content);
}

/**
 * @param {string} file_name 
 * @param {string} content 
 * @returns {void}
 */
function download(file_name, content) {
	const blob = new Blob([content], { type: "text/csv" });

	const url = window.URL.createObjectURL(blob);
	const elem = window.document.createElement("a");

	elem.href = url;
	elem.download = file_name;

	document.body.appendChild(elem);

	elem.click();

	document.body.removeChild(elem);
	window.URL.revokeObjectURL(url);
}


