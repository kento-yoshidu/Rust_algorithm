import * as React from "react"
import Link from "next/link"

const Styles = require("../../styles/header.module.scss")

const Header: React.VFC = () => (
	<header className={Styles.header}>
		<h1 className={Styles.headerTitle}>NextJS microCMS Site</h1>
		
		<nav>
			<ul className={Styles.mainNav}>
				<li><Link href="https://google.com">News</Link></li>
				<li><Link href="https://google.com">Blog</Link></li>
				<li><Link href="https://google.com">Contact</Link></li>
			</ul>
		</nav>
	</header>
)

export default Header
