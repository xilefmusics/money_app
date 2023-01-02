export default async (username, password) => {
	const response = await fetch('/api/login', {
		method: 'POST',
		body: JSON.stringify({ username: username, password: password })
	});
	const token = await response.json();

	localStorage.setItem('token', token);
	localStorage.setItem('username', username);
};
