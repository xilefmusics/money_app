export default (name, pods) => {
	const result = pods.filter((pod) => pod.name === name);
	if (result.length != 1) {
		return null;
	}
	return result[0].type;
};
