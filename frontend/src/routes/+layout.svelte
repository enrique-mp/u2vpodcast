<script lang="ts">
    import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
    import { DarkMode, Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
    import { ChevronDownOutline } from 'flowbite-svelte-icons';
    import "../app.css";
    import logo from '$lib/assets/favicon/favicon-48x48.png';
    import LoginForm from '$lib/components/LoginForm.svelte';

    let btnClass = "text-primary-500 dark:text-primary-600 border dark:border-gray-800";
    let loginFormShow = false;
</script>
<Navbar class="mb-8" let:hidden let:toggle>
  <NavBrand href="/">
    <img
      src={logo}
      class="mr-3 h-6 sm:h-9"
      alt="U2VPodcast"
    />
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
    U2VPodcast
    </span>
  </NavBrand>
  <NavHamburger on:click={toggle} />
  <NavUl {hidden}>
    <NavLi href="/" active={true}>Home</NavLi>
    <NavLi class="cursor-pointer">
        Configuration<ChevronDownOutline class="w-3 h-3 ms-2 text-primary-800 dark:text-white inline" />
        <Dropdown class="w-44 z-20">
            <DropdownItem href="/app/configure/channels">Channels</DropdownItem>
            <DropdownItem href="/app/configure/users">Users</DropdownItem>
            <DropdownItem href="/app/configure/general">General</DropdownItem>
        </Dropdown>
    </NavLi>
    <Button on:click={() => (loginFormShow = true)}>Login</Button>
    <DarkMode {btnClass} />
  </NavUl>
</Navbar>
<slot />
<LoginForm bind:open={loginFormShow} />
