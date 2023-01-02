export default async (url, meta) => {
	if (!meta) {
		meta = {
			method: 'GET',
			headers: {}
		};
	}
	meta.headers['Token'] = localStorage.getItem('token');
	return await fetch(url, meta);
};
