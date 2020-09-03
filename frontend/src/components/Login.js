import React, {Component} from 'react';
import {Button, Container, Alert, Form, FormGroup, Input, Label} from 'reactstrap'

export default class Login extends Component {
	constructor(props) {
		super(props);
		this.state = {
			email: '',
			emailErrorPresent: false,
			emailErrorMessage: '',
			password: '',
			passwordErrorPresent: false,
			passwordErrorMessage: '',
			generalErrorMessage: '',
			generalErrorPresent: false,
		}
	}

	async makeRequest(email, password) {
		let body = JSON.stringify({"email": email, "password": password})

		const response = await fetch('/api/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: body
		})

		return response.json()
	}

	async validateForm() {
		const email = document.getElementById('email').value.trim()
		const password = document.getElementById('password').value.trim()

		return await this.makeRequest(email, password)
	}

	login(e) {
		e.preventDefault()

		this.validateForm().then(res => {
			console.log(res)
			if (res.password_is_incorrect === true) {
				this.setState({generalErrorPresent: true})
				this.setState({generalErrorMessage: 'Email or password is incorrect'})
			}
		})
	}

	render() {
		return (
			<Container className="text-left">
				<Form id="loginForm" onSubmit={e => this.login(e)}>
					<FormGroup>
						<Label for="email">Email</Label>
						<Input type="email" name="email" id="email" placeholder="Enter Email"/>
						{/*{this.EmailValidationError}*/}
					</FormGroup>
					<FormGroup>
						<Label for="password">Password</Label>
						<Input type="password" name="password" id="password" placeholder="Enter password"/>
					</FormGroup>
					<Alert color="warning" isOpen={this.state.generalErrorPresent}>
						{this.state.generalErrorMessage}
					</Alert>
					<Button>Login</Button>
				</Form>
			</Container>
		)
	}
}
