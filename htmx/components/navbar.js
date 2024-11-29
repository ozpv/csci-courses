customElements.define('cs-navbar', class CsNavbar extends HTMLElement {
  // This method runs when your custom element is added to the page
  connectedCallback() {
    const root = this.attachShadow({ mode: 'closed' })
    root.innerHTML = `
		<nav class="bg-base dark:bg-darkbase">
			<div class="block max-w-screen-xl content-center p-4">
				<p>Navbar</p>
			</div>
		</nav>
    `
    htmx.process(root) // Tell HTMX about this component's shadow DOM
  }
})
