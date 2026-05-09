import { Component } from "@angular/core";

@Component({
  selector: "app-root",
  standalone: true,
  template: `
    <main class="app-shell">
      <h1>Angular App</h1>
      <p>ビルドができる最小構成です。</p>
    </main>
  `,
  styles: [
    `
      :host {
        display: block;
        min-height: 100vh;
        padding: 2rem;
        background: linear-gradient(135deg, #1d4ed8 0%, #2563eb 100%);
        color: white;
        font-family: Roboto, Arial, sans-serif;
      }
      .app-shell {
        max-inline-size: 48rem;
        margin: 0 auto;
        text-align: center;
      }
      h1 {
        margin: 0;
        font-size: clamp(2rem, 4vw, 3rem);
      }
      p {
        margin: 1rem 0 0;
        font-size: 1.1rem;
      }
    `,
  ],
})
export class App {}
