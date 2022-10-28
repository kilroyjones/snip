<script lang="ts">
	import SidebarMenuLink from './SidebarMenuLink.svelte';
	import SidebarSpacer from './SidebarSpacer.svelte';
	import SidebarMenuToggle from './SidebarMenuToggle.svelte';
	import {
		faBars,
		faBookmark,
		faHouse,
		faImage,
		faQuoteLeft,
		faTrash,
		faSun,
		faGear,
		faMoon
	} from '@fortawesome/free-solid-svg-icons';
	import { darkMode } from '$lib/settings';
	import SidebarDarkMode from './SidebarDarkMode.svelte';

	let isExpanded: boolean = false;

	const fadeIn = {
		delay: 100,
		duration: 200
	};

	const fadeOut = {
		delay: 0,
		duration: 100
	};

	function toggleMenuExpand() {
		isExpanded = !isExpanded;
	}

	function toggleDarkMode() {
		$darkMode = !$darkMode;
		console.log($darkMode);
		localStorage.setItem('learndeck-darkmode', $darkMode.toString());
	}
</script>

<nav class:expanded={isExpanded} class="sidebar-menu">
	<ul>
		<SidebarMenuToggle icon={faBars} {toggleMenuExpand} />
		<SidebarMenuLink link="/home" icon={faHouse} {isExpanded} text="Home" />
		<SidebarMenuLink link="/bookmarks" icon={faBookmark} {isExpanded} text="Bookmarks" />
		<SidebarMenuLink link="/images" icon={faImage} {isExpanded} text="Images" />
		<SidebarMenuLink link="/quotes" icon={faQuoteLeft} {isExpanded} text="Quotes" />
		<SidebarSpacer height={50} color="#f0f0f" />
		<SidebarDarkMode icon={faSun} {toggleDarkMode} {isExpanded} text="Dark mode" />
		<SidebarMenuLink link="/settings" icon={faGear} {isExpanded} text="Settings" />
		<SidebarMenuLink link="/trash" icon={faTrash} {isExpanded} text="Trash" />
	</ul>
</nav>

<style>
	nav {
		transition: ease-out 200ms;
		width: 60px;
		overflow: hidden;
		height: 100vh;
	}

	.expanded {
		transition: ease-out 200ms;
		width: 200px;
	}

	ul {
		list-style: none;
		margin: 0px;
		padding: 0px;
	}
</style>
