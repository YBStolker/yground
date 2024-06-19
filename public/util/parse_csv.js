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
			return false;
		}
	}
	return true;
}

/**
 * @param {string} needle 
 * @param {string} haystack 
 * @returns {number}
 */
function count_occurrences(needle, haystack) {
	let count = 0;
	for (let i = 0; i < haystack.length; i++) {
		if (is_substring_at(i, needle, haystack)) {
			count++;
		}
	}
	return count;
}

/**
 * @param {string} input_string 
 * @returns {{newline?: string, separator?: string, quote?: string}}
 */
function detect_parse_csv_params(input_string) {
	const newline = (() => {
		const possible_newlines = ["\r\n", "\n", "\r"]
		for (const n of possible_newlines) {
			if (input_string.includes(n)) {
				return n;
			}
		}
	})();

	if (!newline) {
		return {};
	}


	const separator = (() => {
		const lines = input_string.split(newline).slice(0, 1000).filter(l => !!l);

		const possible_separators = [";", ",", "\t"];
		const count_obj = {};
		for (const sep of possible_separators) {
			for (const line of lines) {
				const sep_count = count_occurrences(sep, line);
				if (sep_count === 0) {
					continue;
				}

				if (!count_obj[sep]) {
					count_obj[sep] = {};
				}

				if (!count_obj[sep][sep_count]) {
					count_obj[sep][sep_count] = 1;
				}
				else {
					count_obj[sep][sep_count] += 1;
				}
			}
		}

		let maybe_sep;
		let max_rate = 0.5;
		for (const [sep, counts] of Object.entries(count_obj)) {
			for (const line_count of Object.values(counts)) {
				const rate = line_count / lines.length;
				if (rate > max_rate) {
					maybe_sep = sep;
					max_rate = rate;
				}
			}
		}

		return maybe_sep;
	})()

	const quote = (() => {
		if (!separator) {
			return;
		}

		const possible_quotes = [`"`, "'", '`'];

		let maybe_quote;
		let max_count = 0;

		for (const q of possible_quotes) {
			const start_occurences = count_occurrences(`${separator}${q}`, input_string)
				+ count_occurrences(`${newline}${q}`, input_string)
				+ Number(input_string.startsWith(q));

			const end_occurences = count_occurrences(`${q}${separator}`, input_string)
				+ count_occurrences(`${q}${newline}`, input_string)
				+ Number(input_string.endsWith(q));

			if (start_occurences === end_occurences && start_occurences > max_count) {
				maybe_quote = q;
				max_count = start_occurences;
			}
		}

		return maybe_quote;
	})()

	const params = { newline, separator, quote };
	return params;
}

/**
 * @param {string} input_string 
 * @param {string} separator 
 * @param {string} newline 
 * @param {string} quote 
 * @returns {string[][]}
 */
function parse_as_csv(input_string, separator, newline, quote, is_first_row_headers = true) {
	const table = [[""]];

	let is_quoted = false;
	for (let i = 0; i < input_string.length; i++) {
		const i_row = table.length - 1;
		const i_value = table[i_row].length - 1;

		const is_separator = is_substring_at(i, separator, input_string);
		if (is_separator && !is_quoted) {
			table[i_row].push("");
			i += separator.length - 1;
			continue;
		}

		const is_quote_start = (() => {
			if (!quote) {
				return false;
			}

			// The following cases count as a quote start
			// <separator><quote>
			// <newline><quote>
			// <start of input_string><quote>
			if (!is_substring_at(i, quote, input_string)) {
				return false;
			}

			if (0 === i) {
				return true;
			}

			if (is_substring_at(i - separator.length, separator, input_string)) {
				return true;
			}

			return is_substring_at(i - newline.length, newline, input_string);
		})()

		if (!is_quoted && is_quote_start) {
			is_quoted = true;
			i += quote.length - 1;
			continue;
		}

		const is_quote_end = (() => {
			if (!quote) {
				return false;
			}

			// The following cases count as a quote start
			// <quote><separator>
			// <quote><newline>
			// <quote><end of input_string>
			if (!is_substring_at(i, quote, input_string)) {
				return false;
			}

			if (i + quote.length === input_string.length) {
				return true;
			}

			if (is_substring_at(i + quote.length, separator, input_string)) {
				return true;
			}

			return is_substring_at(i + quote.length, newline, input_string);
		})()

		if (is_quoted && is_quote_end) {
			is_quoted = false;
			i += quote.length - 1;
			continue;
		}

		const is_newline = is_substring_at(i, newline, input_string);
		if (is_newline && !is_quoted) {
			table.push([""]);
			i += newline.length - 1;
			continue;
		}

		table[i_row][i_value] += input_string[i];
	}

	table.length = table.length - 1;

	if (is_first_row_headers) {
		const headers = table.shift()?.map(header => header.replace(/[^\w\d]+/g, "_").replace(/_+/g, "_"));

		for (let i = 0; i < headers.length - 1; i++) {
			let count = 0;
			for (let j = i + 1; j < headers.length; j++) {
				if (headers[i] === headers[j]) {
					headers[j] += `${++count}`;
				}
			}

			if (count > 0) {
				headers[i] += "0"
			}
		}

		for (let i = 0; i < table.length; i++) {
			const row = table[i];
			const new_row = {};
			headers.forEach((header, i) => {
				new_row[header] = row[i];
			})
			table[i] = new_row;
		}
	}


	return table;
}

