export default (array) => {
	let result = {};
	array.forEach((element) => {
		result[element[0]] = element[1];
	});
	return result;
};
