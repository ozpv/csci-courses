customElements.define('cs-navbar', class CsNavbar extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
		<nav class="bg-peach dark:bg-peach-dark">
			<div class="block max-w-screen-xl content-center p-4">
				<p class="text-subtext font-inter font-bold text-center">Navbar</p>
			</div>
		</nav>
    `;
    htmx.process(this.innerHTML); 
  }
});
