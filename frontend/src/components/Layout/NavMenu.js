import React, {useState} from 'react'
import {Collapse, Nav, Navbar, NavbarBrand, NavbarToggler, NavItem, NavLink} from 'reactstrap'
import {useAuth} from "../../context/auth";

const NavMenu = (props) => {
	const [isOpen, setOpen] = useState(false )
	const { setAuthValid } = useAuth()

	const toggleNavbar = () => setOpen(!isOpen)

	async function makeRequest() {
		return await fetch('/api/logout', {
			method: 'DELETE',
		})
	}

	function logout() {
		makeRequest()
			.then(res => {
				if (res.status === 200) {
					setAuthValid(false)
				} else {
					// The logout attempt failed
					console.error(res)
					setAuthValid(true)
				}
			})
	}

	return (
		<div>
			<Navbar color="dark" dark expand="md">
				<NavbarBrand href="/home">Homrs</NavbarBrand>
				<NavbarToggler onClick={toggleNavbar} />
				<Collapse isOpen={isOpen} navbar />
				<Nav className="mr-auto" navbar>
					<NavItem>
						<NavLink href="/meals">Meals</NavLink>
					</NavItem>
					<NavItem>
						<NavLink href="/" onClick={logout}>Log out</NavLink>
					</NavItem>
				</Nav>
			</Navbar>
		</div>
	)
}

export default NavMenu