async function makeRequest(email, password) {
	let body = JSON.stringify({"email": email, "password": password})

	return await fetch('/api/login', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: body
	})
}

export async function login(email, password) {
	try {
		let res = await makeRequest(email, password)
		if (res.status === 200) {
			let body = await res.json()
			return body.logged_in
		}
	} catch (e) {
		console.error(e)
		return false
	}
	// return makeRequest(email, password)
	// 	.then(res => {
	// 		if (res.status === 200) {
	// 			res.json().then(body => {
	// 				if (body.logged_in === true) {
	// 					return
	// 					// setAuthValid(true)
	// 					// setLoggedIn(true)
	// 				} else {
	// 					return false
	// 				}
	// 			})
	// 		}
	// 	})
	// 	.catch(e => {
	// 		console.error(e)
	// 		return false
	// 	})
}