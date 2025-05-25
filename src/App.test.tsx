import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect } from "vitest";

import App from "./App";

describe("App", () => {
  it("renders the Vite + React heading", () => {
    render(<App />);
    expect(screen.getByText("Vite + React")).toBeInTheDocument();
  });

  it("increments count when button is clicked", () => {
    render(<App />);
    const button = screen.getByRole("button", { name: /count is/i });
    expect(button.textContent).toMatch(/count is 0/);
    fireEvent.click(button);
    expect(button.textContent).toMatch(/count is 1/);
  });

  it("renders Vite and React logos", () => {
    render(<App />);
    expect(screen.getByAltText("Vite logo")).toBeInTheDocument();
    expect(screen.getByAltText("React logo")).toBeInTheDocument();
  });
});
