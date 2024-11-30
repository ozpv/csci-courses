customElements.define('cs-footer', class CsFooter extends HTMLElement {
  connectedCallback() {
    this.innerHTML = `
		<nav class="bg-base dark:bg-darkbase">
			<div class="block max-w-screen-xl content-center p-4">
				<p class="text-red dark:text-red-dark">Footer</p>
			</div>
		</nav>
    `;
    htmx.process(this.innerHTML);
  }
});
