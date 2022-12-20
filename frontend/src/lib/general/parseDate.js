// TODO: What happens if parseInt fails

export default (str) => {
	const parts = str.split('-');
	const year = parts.length >= 1 ? parseInt(parts[0], 10) : 1990;
	const month = parts.length >= 2 ? parseInt(parts[1], 10) - 1 : 0;
	const day = parts.length >= 3 ? parseInt(parts[2], 10) : 1;
	return new Date(year, month, day);
};
