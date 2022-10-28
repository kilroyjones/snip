<script lang="ts">
  /* TO FIX
		- Add "all" to the choices to avoid passing multiple types (stores.ts)
	*/
  import Sidebar from "$lib/components/sidebar/Sidebar.svelte";
  import Searchbar from "$lib/components/searchbar/Searchbar.svelte";
  import Searchfilters from "$lib/components/search-filters/Searchfilters.svelte";
  // import { key } from './keyboard/+server';
  import { onMount } from "svelte";
  import { establishSSEConnection } from "./+layout";
  import { darkMode } from "$lib/settings";

  let sidebarWidth = 120;

  // https://dev.to/lenaschnedlitz/create-a-simple-dark-mode-toggle-with-svelte-4b3g
  // function handleKeydown(event: KeyboardEvent) {
  // 	$key = event.key;
  // }
  // let isDarkmode: boolean = false;

  onMount(async () => {
    establishSSEConnection();

    let darkModeCheck = localStorage.getItem("learndeck-darkmode");
    if (darkModeCheck) {
      $darkMode = darkModeCheck === "true";
    } else {
      let prefersDarkmode = window.matchMedia("(prefers-color-scheme: dark)");
      if (prefersDarkmode.matches) {
        $darkMode = false;
        window.localStorage.setItem("learndeck-darkmode", $darkMode.toString());
      }
    }
  });
</script>

<svelte:head>
  {#if $darkMode}
    <style>
      body {
        color: #dfdfdf;
        background-color: #121212;
        transition: background-color 0.3s;
      }

      .sidebar-menu {
        border-right: 1px;
      }
    </style>
  {:else}
    <style>
      body {
        color: #121217;
        background-color: #fefefe;
        transition: background-color 0.3s;
      }
    </style>
  {/if}
</svelte:head>

<div class="main-container">
  <div class="sidebar-container" style="width={sidebarWidth}px">
    <Sidebar />
  </div>
  <div class="search-container">
    <slot />
  </div>
</div>

<style>
  .main-container {
    display: flex;
    flex-direction: row;
    min-height: 0;
    height: 100vh;
    width: 100%;
  }

  .search-container {
    display: flex;
    flex-direction: column;
    min-height: 0;
    width: 100%;
  }

  .sidebar-container {
    display: flex;
    flex: 1;
    flex: 0 0 100px;
    flex-direction: row;
  }
</style>
